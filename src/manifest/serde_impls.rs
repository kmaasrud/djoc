use super::{AuthorManifest, ChapterManifest};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::path::PathBuf;
use std::{fs, io, str::FromStr};

impl FromStr for AuthorManifest {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.into(),
            email: None,
            organization: None,
        })
    }
}

impl<'de> Deserialize<'de> for AuthorManifest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Aux {
            pub name: String,
            pub email: Option<String>,
            pub organization: Option<String>,
        }
        struct AuthorDefVisitor;

        impl<'de> Visitor<'de> for AuthorDefVisitor {
            type Value = AuthorManifest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(AuthorManifest {
                    name: value.into(),
                    email: None,
                    organization: None,
                })
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let aux: Aux =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(AuthorManifest {
                    name: aux.name,
                    email: aux.email,
                    organization: aux.organization,
                })
            }
        }

        deserializer.deserialize_any(AuthorDefVisitor)
    }
}

impl FromStr for ChapterManifest {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = fs::canonicalize(s)?;
        Ok(Self {
            title: path
                .file_stem()
                .map(|s| s.to_string_lossy().into())
                .unwrap_or(s.into()),
            path,
        })
    }
}

impl<'de> Deserialize<'de> for ChapterManifest {
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
            type Value = ChapterManifest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FromStr::from_str(value).map_err(|e| de::Error::custom(e))
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let aux: Aux =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(ChapterManifest {
                    title: aux.title,
                    path: aux.path,
                })
            }
        }

        deserializer.deserialize_any(ChapterDefVisitor)
    }
}
