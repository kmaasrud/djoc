use crate::Author;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::{io, str::FromStr};

impl FromStr for Author {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.into(),
            email: None,
            affiliation: None,
        })
    }
}

impl<'de> Deserialize<'de> for Author {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Aux {
            pub name: String,
            pub email: Option<String>,
            pub affiliation: Option<String>,
        }
        struct AuthorDefVisitor;

        impl<'de> Visitor<'de> for AuthorDefVisitor {
            type Value = Author;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Author {
                    name: value.into(),
                    email: None,
                    affiliation: None,
                })
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let aux: Aux =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Author {
                    name: aux.name,
                    email: aux.email,
                    affiliation: aux.affiliation,
                })
            }
        }

        deserializer.deserialize_any(AuthorDefVisitor)
    }
}
