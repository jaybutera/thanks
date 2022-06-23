use std::fmt;
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

#[derive(Debug, Clone)]
pub struct Thunk {
    pub text: String,
    pub refs: Vec<Hash>,
}

#[derive(Debug)]
pub struct Thesis {
    pub name: String,
    pub refs: Vec<Hash>,
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

impl From<Thesis> for DagJsonThesis {
    fn from(thesis: Thesis) -> DagJsonThesis {
        DagJsonThesis {
            name: thesis.name,
            refs: thesis.refs.into_iter().map(|h| HashMap::from([(String::from("/"), h)])).collect(),
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

impl From<Thunk> for DagJsonThunk {
    fn from(thunk: Thunk) -> DagJsonThunk {
        DagJsonThunk {
            text: thunk.text,
            refs: thunk.refs.into_iter().map(|h| HashMap::from([(String::from("/"), h)])).collect(),
        }
    }
}

impl fmt::Display for Thesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        for (i, hash) in self.refs.iter().enumerate() {
            let thunk = crate::thunks::get_thunk(hash).unwrap();
            write!(f, "[{}]\n{}\n\n", i, thunk.text)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub theses: HashMap<String, Hash>,
}
