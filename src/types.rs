use std::collections::HashMap;
use serde_json::Result;
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

//#[derive(Debug)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Thunk {
    text: String,
    refs: Vec<Hash>,
}

#[derive(Debug)]
pub struct Thesis {
    name: String,
    refs: Vec<Hash>,
}

type IpldLink = HashMap<String, Hash>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DagJsonThesis {
    name: String,
    refs: Vec<IpldLink>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DagJsonThunk {
    #[serde(with = "b64")]
    text: String,
    refs: Vec<IpldLink>,
}

impl From<DagJsonThesis> for Thesis {
    fn from(dag: DagJsonThesis) -> Thesis {
        Thesis {
            name: dag.name,
            refs: dag.refs.into_iter().map(|mut d| d.drain().map(|(_,v)| v).take(1).collect()).collect(),
        }
    }
}

impl From<DagJsonThunk> for Thunk {
    fn from(dag: DagJsonThunk) -> Thunk {
        Thunk {
            text: dag.text,
            refs: dag.refs.into_iter().map(|mut d| d.drain().map(|(_,v)| v).take(1).collect()).collect(),
        }
    }
}

pub fn thesis_to_dagjson(thesis: Thesis) -> Result<String> {
    let dj = DagJsonThesis {
        name: thesis.name,
        refs: thesis.refs.into_iter().map(|h| HashMap::from([(String::from("/"), h)])).collect(),
    };

    serde_json::to_string(&dj)
}

pub fn thunk_to_dagjson(thunk: Thunk) -> Result<String> {
    let dj = DagJsonThunk {
        text: thunk.text,
        refs: thunk.refs.into_iter().map(|h| HashMap::from([(String::from("/"), h)])).collect(),
    };

    serde_json::to_string(&dj)
}