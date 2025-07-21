use serde::Serialize;
use serde::de::{self, Visitor, SeqAccess};
use std::fmt::{self, Display};
use std::str::FromStr;
use std::collections::HashSet;
use surrealdb::dbs::capabilities::Targets;

pub fn serialize<T, S>(targets: &Targets<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display + Eq + std::hash::Hash + Clone,
    S: serde::Serializer,
{
    match targets {
        Targets::None => serializer.serialize_bool(false),
        Targets::All => serializer.serialize_bool(true),
        Targets::Some(set) => set.iter().map(|t| t.to_string()).collect::<Vec<_>>().serialize(serializer),
        #[allow(unreachable_patterns)]
        _ => Err(serde::ser::Error::custom("Unknown Targets variant")),
    }
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Targets<T>, D::Error>
where
    T: FromStr + Eq + std::hash::Hash,
    <T as FromStr>::Err: fmt::Display,
    D: serde::Deserializer<'de>,
{
    struct TargetsVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> Visitor<'de> for TargetsVisitor<T>
    where
        T: FromStr + Eq + std::hash::Hash,
        <T as FromStr>::Err: fmt::Display,
    {
        type Value = Targets<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("false, true, or a list of targets")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
            Ok(if v { Targets::All } else { Targets::None })
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut set = HashSet::new();
            while let Some(elem) = seq.next_element::<String>()? {
                set.insert(elem.parse::<T>().map_err(de::Error::custom)?);
            }
            Ok(Targets::Some(set))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Accept a comma-separated string as a list
            let set: Result<HashSet<T>, _> = v
                .split(',')
                .map(|s| s.trim().parse::<T>())
                .collect();
            set.map(Targets::Some).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(TargetsVisitor(std::marker::PhantomData))
} 