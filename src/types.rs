use serde::{Serialize, Deserialize};
pub type Hash = String;

mod b64 {
    use serde::Deserialize;
    use serde::{Serializer, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode(s).map(|b| String::from_utf8(b).unwrap()).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(bytes: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&base64::encode(bytes.as_bytes()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thunk {
    #[serde(with = "b64")]
    text: String,
    refs: Vec<Hash>,
}
