//! Minimal plaintext Kafka producer for Surrealism.
//!
//! Speaks just enough of the Kafka wire protocol (`Produce` API, version 7,
//! with a v2 record batch) to publish a single record to partition 0 of a
//! topic on one broker. There is no metadata/leader discovery, no
//! compression, no transactions/idempotence, no SASL/TLS, and no retries —
//! `broker` must be the literal `ip:port` of a broker that is itself the
//! leader for partition 0 of `topic` (true of a single-broker Kafka, e.g. for
//! local development).
//!
//! Register with e.g. `DEFINE MODULE mod::kafka AS f"bucket:/kafka.surli";`
//! and call `mod::kafka::produce("10.0.0.5:9092", "events", "user-42",
//! {type: "signup"})`. `broker` must be a literal `ip:port`.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};

use surrealdb_types::Value;
use surrealism::surrealism;

const PRODUCE_API_KEY: i16 = 0;
const PRODUCE_API_VERSION: i16 = 7;
const CLIENT_ID: &str = "surrealism-kafka";

fn write_i16(buf: &mut Vec<u8>, v: i16) {
	buf.extend_from_slice(&v.to_be_bytes());
}

fn write_i32(buf: &mut Vec<u8>, v: i32) {
	buf.extend_from_slice(&v.to_be_bytes());
}

fn write_i64(buf: &mut Vec<u8>, v: i64) {
	buf.extend_from_slice(&v.to_be_bytes());
}

/// `NULLABLE_STRING` / `STRING`: `int16` length prefix (-1 for null) + UTF-8 bytes.
fn write_nullable_string(buf: &mut Vec<u8>, s: Option<&str>) {
	match s {
		None => write_i16(buf, -1),
		Some(s) => {
			write_i16(buf, s.len() as i16);
			buf.extend_from_slice(s.as_bytes());
		}
	}
}

/// `BYTES`: `int32` length prefix + raw bytes. Used for the top-level
/// `records` field, which carries an opaque, independently-framed record
/// batch (see [`encode_record_batch`]).
fn write_bytes_field(buf: &mut Vec<u8>, b: &[u8]) {
	write_i32(buf, b.len() as i32);
	buf.extend_from_slice(b);
}

/// Zigzag-encoded variable-length integer, as used inside a record batch's
/// records (independent of whether the surrounding request uses the
/// "flexible"/compact protocol — the record batch wire format is fixed by
/// the message format version, here v2).
fn write_varint(buf: &mut Vec<u8>, v: i64) {
	let mut z = ((v << 1) ^ (v >> 63)) as u64;
	loop {
		let byte = (z & 0x7F) as u8;
		z >>= 7;
		if z == 0 {
			buf.push(byte);
			break;
		}
		buf.push(byte | 0x80);
	}
}

fn write_varint_bytes(buf: &mut Vec<u8>, b: Option<&[u8]>) {
	match b {
		None => write_varint(buf, -1),
		Some(b) => {
			write_varint(buf, b.len() as i64);
			buf.extend_from_slice(b);
		}
	}
}

/// Bit-by-bit CRC32C (Castagnoli, reflected polynomial `0x82F63B78`), as
/// required by the Kafka record batch header. Deliberately not
/// table-driven: batches here are tiny (one record), so the simplicity is
/// worth more than the throughput.
fn crc32c(data: &[u8]) -> u32 {
	let mut crc: u32 = 0xFFFF_FFFF;
	for &byte in data {
		crc ^= u32::from(byte);
		for _ in 0..8 {
			let mask = (crc & 1).wrapping_neg();
			crc = (crc >> 1) ^ (0x82F6_3B78 & mask);
		}
	}
	!crc
}

/// Encodes a single `Record` (message format v2), length-prefixed with its
/// own varint length as the wire format requires.
fn encode_record(key: Option<&[u8]>, value: Option<&[u8]>) -> Vec<u8> {
	let mut body = Vec::new();
	body.push(0u8); // attributes (always 0 for a record)
	write_varint(&mut body, 0); // timestampDelta
	write_varint(&mut body, 0); // offsetDelta
	write_varint_bytes(&mut body, key);
	write_varint_bytes(&mut body, value);
	write_varint(&mut body, 0); // headers count

	let mut record = Vec::new();
	write_varint(&mut record, body.len() as i64);
	record.extend_from_slice(&body);
	record
}

/// Encodes a `RecordBatch` (message format v2, magic byte 2) containing a
/// single record.
fn encode_record_batch(key: Option<&[u8]>, value: Option<&[u8]>, timestamp_ms: i64) -> Vec<u8> {
	let record = encode_record(key, value);

	let mut batch_body = Vec::new();
	write_i32(&mut batch_body, -1); // partitionLeaderEpoch (unknown)
	batch_body.push(2); // magic (record batch v2)
	let crc_pos = batch_body.len();
	write_i32(&mut batch_body, 0); // crc placeholder, patched below
	write_i16(&mut batch_body, 0); // attributes (no compression, not transactional/control)
	write_i32(&mut batch_body, 0); // lastOffsetDelta (0: a single record)
	write_i64(&mut batch_body, timestamp_ms); // firstTimestamp
	write_i64(&mut batch_body, timestamp_ms); // maxTimestamp
	write_i64(&mut batch_body, -1); // producerId (no idempotence)
	write_i16(&mut batch_body, -1); // producerEpoch
	write_i32(&mut batch_body, -1); // baseSequence
	write_i32(&mut batch_body, 1); // recordsCount
	batch_body.extend_from_slice(&record);

	// CRC32C covers everything from `attributes` onward (i.e. after the crc field itself).
	let crc_data_start = crc_pos + 4;
	let crc = crc32c(&batch_body[crc_data_start..]);
	batch_body[crc_pos..crc_pos + 4].copy_from_slice(&crc.to_be_bytes());

	let mut full = Vec::new();
	write_i64(&mut full, 0); // baseOffset
	write_i32(&mut full, batch_body.len() as i32); // batchLength
	full.extend_from_slice(&batch_body);
	full
}

/// Encodes a length-prefixed `ProduceRequest` (API key 0, version 7) for a
/// single topic/partition.
fn encode_produce_request(
	correlation_id: i32,
	acks: i16,
	timeout_ms: i32,
	topic: &str,
	partition: i32,
	record_batch: &[u8],
) -> Vec<u8> {
	let mut body = Vec::new();
	// RequestHeader (v1): api_key, api_version, correlation_id, client_id.
	write_i16(&mut body, PRODUCE_API_KEY);
	write_i16(&mut body, PRODUCE_API_VERSION);
	write_i32(&mut body, correlation_id);
	write_nullable_string(&mut body, Some(CLIENT_ID));

	// ProduceRequest body (v7, non-flexible).
	write_nullable_string(&mut body, None); // transactional_id (none)
	write_i16(&mut body, acks);
	write_i32(&mut body, timeout_ms);
	write_i32(&mut body, 1); // topic_data: 1 topic
	write_nullable_string(&mut body, Some(topic));
	write_i32(&mut body, 1); // partition_data: 1 partition
	write_i32(&mut body, partition);
	write_bytes_field(&mut body, record_batch);

	let mut framed = Vec::new();
	write_i32(&mut framed, body.len() as i32);
	framed.extend_from_slice(&body);
	framed
}

struct Reader<'a> {
	data: &'a [u8],
	pos: usize,
}

impl<'a> Reader<'a> {
	fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
		let end = self.pos.checked_add(n).ok_or("Malformed produce response: length overflow")?;
		let slice = self.data.get(self.pos..end).ok_or("Malformed produce response: truncated")?;
		self.pos = end;
		Ok(slice)
	}

	fn i16(&mut self) -> Result<i16, String> {
		Ok(i16::from_be_bytes(self.take(2)?.try_into().expect("2 bytes")))
	}

	fn i32(&mut self) -> Result<i32, String> {
		Ok(i32::from_be_bytes(self.take(4)?.try_into().expect("4 bytes")))
	}

	fn i64(&mut self) -> Result<i64, String> {
		Ok(i64::from_be_bytes(self.take(8)?.try_into().expect("8 bytes")))
	}

	fn nullable_string(&mut self) -> Result<Option<String>, String> {
		let len = self.i16()?;
		if len < 0 {
			return Ok(None);
		}
		let bytes = self.take(len as usize)?;
		Ok(Some(String::from_utf8_lossy(bytes).into_owned()))
	}
}

/// Parses a `ProduceResponse` (v7) for a single topic/partition, returning
/// the assigned base offset on success.
fn parse_produce_response(data: &[u8]) -> Result<i64, String> {
	let mut r = Reader {
		data,
		pos: 0,
	};
	let _correlation_id = r.i32()?;
	let topic_count = r.i32()?;
	for _ in 0..topic_count {
		let _topic_name = r.nullable_string()?;
		let partition_count = r.i32()?;
		for _ in 0..partition_count {
			let _index = r.i32()?;
			let error_code = r.i16()?;
			let base_offset = r.i64()?;
			let _log_append_time = r.i64()?;
			let _log_start_offset = r.i64()?;
			if error_code != 0 {
				return Err(format!(
					"Kafka broker rejected the produce request (error code {error_code}); see \
					 https://kafka.apache.org/protocol.html#protocol_error_codes"
				));
			}
			return Ok(base_offset);
		}
	}
	Err("Empty produce response".to_string())
}

fn current_timestamp_millis() -> i64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map(|d| d.as_millis() as i64)
		.unwrap_or(0)
}

/// Converts a SurrealQL value into Kafka record-value bytes: strings and
/// `bytes` pass through as-is, `NONE`/`NULL` become a tombstone, anything
/// else is JSON-encoded.
fn value_to_kafka_bytes(value: Value) -> Option<Vec<u8>> {
	match value {
		Value::None | Value::Null => None,
		Value::String(s) => Some(s.into_bytes()),
		Value::Bytes(b) => Some(b.into_inner().to_vec()),
		other => Some(other.into_json_value().to_string().into_bytes()),
	}
}

fn produce_impl(
	broker: &str,
	topic: &str,
	key: Option<&str>,
	value: Option<&[u8]>,
) -> Result<i64, String> {
	let addr: SocketAddr = broker.parse().map_err(|_| {
		format!(
			"'{broker}' is not a literal ip:port address. Guest code cannot resolve hostnames \
			 (see the module README), so the broker must be given as e.g. '10.0.0.5:9092'."
		)
	})?;
	let mut stream = TcpStream::connect(addr).map_err(|e| format!("Failed to connect to {broker}: {e}"))?;

	let record_batch = encode_record_batch(key.map(str::as_bytes), value, current_timestamp_millis());
	// acks=1 (leader only), 5s broker-side timeout, always partition 0.
	let request = encode_produce_request(1, 1, 5000, topic, 0, &record_batch);

	stream.write_all(&request).map_err(|e| format!("Failed to send produce request: {e}"))?;

	let mut size_buf = [0u8; 4];
	stream.read_exact(&mut size_buf).map_err(|e| format!("Failed to read response size: {e}"))?;
	let size = i32::from_be_bytes(size_buf);
	let size = usize::try_from(size).map_err(|_| "Broker sent a negative response size".to_string())?;
	let mut response = vec![0u8; size];
	stream.read_exact(&mut response).map_err(|e| format!("Failed to read response body: {e}"))?;

	parse_produce_response(&response)
}

/// Publishes a single record to partition 0 of `topic` on `broker`
/// (`ip:port`). `key` may be empty to publish an unkeyed record. Returns the
/// offset the broker assigned to the record.
#[surrealism]
fn produce(broker: String, topic: String, key: String, value: Value) -> Result<i64, String> {
	let key = if key.is_empty() { None } else { Some(key.as_str()) };
	let value = value_to_kafka_bytes(value);
	produce_impl(&broker, &topic, key, value.as_deref())
}
