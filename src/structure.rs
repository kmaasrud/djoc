use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::{fs, io, path::PathBuf, str::FromStr};

#[derive(Deserialize)]
pub struct DocumentDef {
    pub title: String,
    pub authors: Vec<AuthorDef>,
    pub chapters: Vec<ChapterDef>,
}

#[derive(Default)]
pub struct AuthorDef {
    pub name: String,
    pub organization: Option<String>,
}

impl<'de> Deserialize<'de> for AuthorDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Aux {
            pub name: String,
            pub organization: Option<String>,
        }
        struct AuthorDefVisitor;

        impl<'de> Visitor<'de> for AuthorDefVisitor {
            type Value = AuthorDef;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AuthorDef {
                    name: value.into(),
                    ..Default::default()
                })
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let aux: Aux =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(AuthorDef {
                    name: aux.name,
                    organization: aux.organization,
                })
            }
        }

        deserializer.deserialize_any(AuthorDefVisitor)
    }
}

pub struct ChapterDef {
    pub title: String,
    pub path: PathBuf,
}

impl FromStr for ChapterDef {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = fs::canonicalize(s)?;
        Ok(Self {
            path,
            title: s.into(),
        })
    }
}

impl<'de> Deserialize<'de> for ChapterDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Aux {
            title: String,
            path: PathBuf,
        }
        struct ChapterDefVisitor;

        impl<'de> Visitor<'de> for ChapterDefVisitor {
            type Value = ChapterDef;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FromStr::from_str(value).unwrap())
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let aux: Aux =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(ChapterDef {
                    title: aux.title,
                    path: aux.path,
                })
            }
        }

        deserializer.deserialize_any(ChapterDefVisitor)
    }
}
