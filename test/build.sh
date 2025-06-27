cd ../crates/demo
cargo build --target wasm32-wasip1 --release
cd ../../target/wasm32-wasip1/release
wasm-strip demo.wasm
wasm-opt -O2 --strip-dwarf --strip-debug --strip-producers -o out.wasm --enable-bulk-memory demo.wasm
cp out.wasm ../../../test/

cd ../../../crates/surrealism-runtime
cargo build --release
cp ../../target/release/surrealism-runtime ../../test/

cd ../../test