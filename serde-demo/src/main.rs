use std::{collections::HashMap, path::Path};

use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Deserialize)]
struct Foo {
    a: u8,
    b: u8,
}

#[derive(Serialize, Deserialize)]
struct Bar {
    #[serde(rename = "x")] // rename this field when serializing
    a: u8,

    #[serde(skip)] // skip this field when serializing
    b: u8,

    #[serde(default)] // use default value when deserializing
    default_value: u32,

    foo: Foo,
}

#[derive(Serialize)]
#[serde(from = "String")] // can be useful if the MyStruct type is not owned by you
struct MyStruct {
    a: u8,
    b: u8,
}

/// Serialize and deserialize a newtype struct or a braced struct with one field exactly
/// the same as if its one field were serialized and deserialized by itself. Analogous to #[repr(transparent)].
#[derive(Serialize)]
#[serde(transparent)]
struct NewType(String);

// if we want to serialize struct without derive macro, we can implement Serialize trait
impl Serialize for Foo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // we can use serializer.serialize_struct to serialize struct, name Foo and 2 fields
        let mut s = serializer.serialize_struct("Foo", 2)?;
        s.serialize_field("a", &self.a)?;
        s.serialize_field("b", &self.b)?;
        s.end()
    }
}

/*
the liftime of the deserializer is 'de which is the lifetime of input data
which means it have a reference to the input data

impl<'de> _serde::Deserialize<'de> for Foo {
        fn deserialize<__D>(
            __deserializer: __D,
 */

fn main() {
    let res = load_model("./model.json");
    println!("{:#?}", res);
}

pub fn load_model<P: AsRef<Path>>(path: P) -> ElementCustom {
    let content = std::fs::read_to_string(path).unwrap();
    let ele: ElementCustom = serde_json::from_str(&content).unwrap();
    ele
}

#[derive(Deserialize, Debug)]
pub struct Model {
    pub definitions: HashMap<String, Definition>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "camelCase")]
pub enum Definition {
    Entity(Entity),
}

#[derive(Deserialize, Debug)]
pub struct Entity {
    pub elements: HashMap<String, Element>,
}

#[derive(Deserialize, Debug)]
pub struct Element {
    #[serde(default)]
    pub key: bool,
    #[serde(rename(deserialize = "type"))]
    pub element_type: String,
    pub annotations: HashMap<String, serde_json::Value>,
}

#[derive(Default, Debug)]
pub struct ElementCustom {
    pub key: bool,
    pub element_type: String,
    pub annotations: HashMap<String, serde_json::Value>,
}

impl<'de> Deserialize<'de> for ElementCustom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ElementVisitor{})
    }
}

struct ElementVisitor {}

impl<'de> Visitor<'de> for ElementVisitor {
    type Value = ElementCustom;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("could not deserialize Element")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut element = Self::Value { ..Default::default() };

        while let Some(key) = map.next_key::<String>()? {
            if key.starts_with("@") {
                element.annotations.insert(key, map.next_value()?);
            } else {
                match key.as_str() {
                    "type" => {
                        element.element_type = map.next_value()?;
                    }
                    "key" => {
                        element.key = map.next_value()?;
                    }
                    _ => {}
                }
            }
        }
        if element.element_type.is_empty() {
            return Err(serde::de::Error::missing_field("type is required"));
        }
        Ok(element)
    }
}
