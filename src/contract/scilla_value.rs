use std::{collections::HashMap, hash::Hash};

use serde::Deserialize;

use crate::{
    core::{BNum, ZilAddress},
    Error,
};

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ScillaValue {
    Primitive(String),
    Adt(AdtValue),
    Map(HashMap<String, ScillaValue>),
    List(Vec<ScillaValue>),
}

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
pub struct KeyVal {
    key: ScillaValue,
    val: ScillaValue,
}

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
pub struct ScillaVariable {
    pub vname: String,
    pub r#type: String,
    pub value: ScillaValue,
}

impl ScillaVariable {
    pub fn new(vname: String, r#type: String, value: ScillaValue) -> Self {
        Self { vname, value, r#type }
    }

    pub fn new_from_str<T: ToScillaValue>(vname: &str, r#type: &str, value: T) -> Self {
        Self {
            vname: vname.to_string(),
            value: value.to_value(),
            r#type: r#type.to_string(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
pub struct AdtValue {
    constructor: String,
    argtypes: Vec<String>,
    arguments: Vec<ScillaValue>,
}

// TODO: Set better names for trait functions
pub trait ToScillaValue {
    fn to_value(&self) -> ScillaValue;
    fn scilla_type() -> String;
}

pub trait TryFromScillaValue: Sized {
    fn try_from_scilla_value(value: ScillaValue) -> Result<Self, Error>;
}

// Can't use std TryFrom because it conflicts with std implementation for option, map, etc
pub trait TryIntoRustType<T>: Sized {
    fn try_into_rust_type(self) -> Result<T, Error>;
}

impl<T> TryIntoRustType<T> for ScillaValue
where
    T: TryFromScillaValue,
{
    fn try_into_rust_type(self) -> Result<T, Error> {
        T::try_from_scilla_value(self)
    }
}

macro_rules! from_scilla_value_for {
    ($t:ty) => {
        impl TryFromScillaValue for $t {
            fn try_from_scilla_value(value: ScillaValue) -> Result<$t, Error> {
                match value {
                    ScillaValue::Primitive(s) => s
                        .parse()
                        .map_err(|_| Error::FailedToParseScillaValue(s, stringify!($t).to_string())),
                    _ => Err(Error::FailedToParseScillaValue(
                        serde_json::to_string(&value)?,
                        stringify!($t).to_string(),
                    )),
                }
            }
        }
    };
}

macro_rules! to_scilla_value_for {
    ($t:ty, $scilla_type:expr) => {
        impl ToScillaValue for $t {
            fn to_value(&self) -> ScillaValue {
                ScillaValue::Primitive(self.to_string())
            }

            fn scilla_type() -> String {
                $scilla_type.to_string()
            }
        }
    };
}

to_scilla_value_for!(i32, "Int32");
to_scilla_value_for!(i64, "Int64");
to_scilla_value_for!(i128, "Int128");
to_scilla_value_for!(u32, "Uint32");
to_scilla_value_for!(u64, "Uint64");
to_scilla_value_for!(u128, "Uint128");
to_scilla_value_for!(primitive_types::U256, "Uint256");
to_scilla_value_for!(String, "String");
to_scilla_value_for!(&str, "String");
to_scilla_value_for!(ZilAddress, "ByStr20");
to_scilla_value_for!(&ZilAddress, "ByStr20");
to_scilla_value_for!(BNum, "BNum");

from_scilla_value_for!(i32);
from_scilla_value_for!(i64);
from_scilla_value_for!(i128);
from_scilla_value_for!(u32);
from_scilla_value_for!(u64);
from_scilla_value_for!(u128);
from_scilla_value_for!(primitive_types::U256);
from_scilla_value_for!(String);
from_scilla_value_for!(ZilAddress);
from_scilla_value_for!(BNum);

impl<T: ToScillaValue> ToScillaValue for Option<T> {
    fn to_value(&self) -> ScillaValue {
        match self {
            Some(v) => ScillaValue::Adt(AdtValue {
                constructor: "Some".to_string(),
                argtypes: vec![T::scilla_type()],
                arguments: vec![v.to_value()],
            }),
            None => ScillaValue::Adt(AdtValue {
                constructor: "None".to_string(),
                argtypes: vec![T::scilla_type()],
                arguments: vec![],
            }),
        }
    }

    fn scilla_type() -> String {
        format!("Option ({})", T::scilla_type())
    }
}

impl<T> TryFromScillaValue for Option<T>
where
    T: TryFromScillaValue,
{
    fn try_from_scilla_value(value: ScillaValue) -> Result<Self, Error> {
        let error = Error::FailedToParseScillaValue(serde_json::to_string(&value)?, "Option".to_string());
        if let ScillaValue::Adt(adt) = &value {
            match adt.constructor.as_str() {
                "Some" => return Ok(Some(adt.arguments.get(0).ok_or(error)?.to_owned().try_into_rust_type()?)),
                "None" => return Ok(None),
                _ => (),
            }
        }
        Err(error)
    }
}

impl ToScillaValue for bool {
    fn to_value(&self) -> ScillaValue {
        ScillaValue::Adt(AdtValue {
            constructor: if *self { "True".to_string() } else { "False".to_string() },
            argtypes: vec![],
            arguments: vec![],
        })
    }

    fn scilla_type() -> String {
        "Bool".to_string()
    }
}

impl TryFromScillaValue for bool {
    fn try_from_scilla_value(value: ScillaValue) -> Result<Self, Error> {
        if let ScillaValue::Adt(adt) = &value {
            match adt.constructor.as_str() {
                "True" => return Ok(true),
                "False" => return Ok(false),
                _ => (),
            }
        }
        Err(Error::FailedToParseScillaValue(
            serde_json::to_string(&value)?,
            "bool".to_string(),
        ))
    }
}

impl<T: ToScillaValue, U: ToScillaValue> ToScillaValue for (T, U) {
    fn to_value(&self) -> ScillaValue {
        ScillaValue::Adt(AdtValue {
            constructor: "Pair".to_string(),
            argtypes: vec![T::scilla_type(), U::scilla_type()],
            arguments: vec![self.0.to_value(), self.1.to_value()],
        })
    }

    fn scilla_type() -> String {
        format!("Pair {} {}", T::scilla_type(), U::scilla_type())
    }
}

impl<T, U> TryFromScillaValue for (T, U)
where
    T: TryFromScillaValue,
    U: TryFromScillaValue,
{
    fn try_from_scilla_value(value: ScillaValue) -> Result<Self, Error> {
        let error = Error::FailedToParseScillaValue(serde_json::to_string(&value)?, "Pair".to_string());
        if let ScillaValue::Adt(mut adt) = value {
            if adt.arguments.len() != 2 {
                return Err(error);
            }
            // Safe to call unwrap because we already checked the size
            let y = adt.arguments.pop().unwrap().try_into_rust_type()?;
            let x = adt.arguments.pop().unwrap().try_into_rust_type()?;
            return Ok((x, y));
        }

        Err(error)
    }
}

impl<K: ToScillaValue, V: ToScillaValue> ToScillaValue for HashMap<K, V> {
    fn to_value(&self) -> ScillaValue {
        // FIXME:
        todo!()
        // ScillaValue::Map(
        //     self.iter()
        //         .map(|(key, value)| KeyVal {
        //             key: key.to_value(),
        //             val: value.to_value(),
        //         })
        //         .collect(),
        // )
    }

    fn scilla_type() -> String {
        format!("Map {} {}", K::scilla_type(), V::scilla_type())
    }
}

impl<K: TryFromScillaValue + std::cmp::Eq + Hash, V: TryFromScillaValue> TryFromScillaValue for HashMap<K, V> {
    fn try_from_scilla_value(value: ScillaValue) -> Result<Self, Error> {
        let error = Error::FailedToParseScillaValue(serde_json::to_string(&value)?, "Map".to_string());
        if let ScillaValue::Map(map) = value {
            return map
                .into_iter()
                .map(|(key, val)| {
                    Ok((
                        K::try_from_scilla_value(ScillaValue::Primitive(key))?,
                        V::try_from_scilla_value(val)?,
                    ))
                })
                .collect::<Result<HashMap<K, V>, Error>>();
        }

        Err(error)
    }
}

impl<T: ToScillaValue> ToScillaValue for [T] {
    fn to_value(&self) -> ScillaValue {
        if self.is_empty() {
            ScillaValue::Adt(AdtValue {
                constructor: "Nil".to_string(),
                argtypes: vec![T::scilla_type()],
                arguments: vec![],
            })
        } else {
            ScillaValue::Adt(AdtValue {
                constructor: "Cons".to_string(),
                argtypes: vec![T::scilla_type()],
                arguments: vec![self[0].to_value(), self[1..].to_value()],
            })
        }
    }

    fn scilla_type() -> String {
        format!("(List ({}))", T::scilla_type())
    }
}

impl<T: ToScillaValue> ToScillaValue for Vec<T> {
    fn to_value(&self) -> ScillaValue {
        self[..].to_value()
    }

    fn scilla_type() -> String {
        <[T]>::scilla_type()
    }
}

impl<T: TryFromScillaValue> TryFromScillaValue for Vec<T> {
    fn try_from_scilla_value(value: ScillaValue) -> Result<Self, Error> {
        let error = Error::FailedToParseScillaValue(serde_json::to_string(&value)?, "List".to_string());
        if let ScillaValue::List(list) = value {
            return list
                .into_iter()
                .map(|f| f.try_into_rust_type())
                .collect::<Result<Vec<T>, Error>>();
        }

        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::core::ZilAddress;

    use super::ToScillaValue;

    #[test]
    fn check_scilla_types() {
        assert_eq!(
            Vec::<(ZilAddress, Vec<(ZilAddress, u32)>)>::scilla_type(),
            "(List (Pair ByStr20 (List (Pair ByStr20 Uint32))))"
        );
    }

    #[test]
    fn test_bool_value() {
        let scilla_value = true.to_value();
        let scilla_value = serde_json::to_string(&scilla_value).unwrap();
        assert_eq!(r#"{"constructor":"True","argtypes":[],"arguments":[]}"#, scilla_value);
        assert_eq!("Bool", bool::scilla_type());
    }

    #[test]
    fn test_option_value() {
        let scilla_value = Some(true).to_value();
        let scilla_value = serde_json::to_string(&scilla_value).unwrap();
        assert_eq!(
            r#"{"constructor":"Some","argtypes":["Bool"],"arguments":[{"constructor":"True","argtypes":[],"arguments":[]}]}"#,
            scilla_value
        );
        assert_eq!("Option (Bool)", Option::<bool>::scilla_type());
    }

    #[test]
    fn test_pair_value() {
        assert_eq!("Pair String Uint32", <(String, u32)>::scilla_type());

        let scilla_value = ("hello".to_string(), 123u32).to_value();
        let scilla_value = serde_json::to_string(&scilla_value).unwrap();
        assert_eq!(
            r#"{"constructor":"Pair","argtypes":["String","Uint32"],"arguments":["hello","123"]}"#,
            scilla_value
        );
    }

    #[test]
    fn test_map_value() {
        assert_eq!("Map (String) (Int32)", HashMap::<String, i32>::scilla_type());

        let mut vikings = HashMap::new();
        vikings.insert("Denmark", 24);

        let json = json!([
            {
                "key": "Denmark", "val": "24",
            },
        ]);
        let scilla_value = vikings.to_value();
        let scilla_value = serde_json::to_string(&scilla_value).unwrap();
        assert_eq!(serde_json::to_string(&json).unwrap(), scilla_value);
    }

    #[test]
    fn test_list_value() {
        assert_eq!("List (String)", Vec::<String>::scilla_type());
        let scilla_value = vec!["salam".to_string(), "salam2".to_string()].to_value();
        let scilla_value = serde_json::to_string(&scilla_value).unwrap();
        assert_eq!("{\"constructor\":\"Cons\",\"argtypes\":[\"String\"],\"arguments\":[\"salam\",{\"constructor\":\"Cons\",\"argtypes\":[\"String\"],\"arguments\":[\"salam2\",{\"constructor\":\"Nil\",\"argtypes\":[\"String\"],\"arguments\":[]}]}]}", scilla_value);
    }

    #[test]
    // FIXME
    fn test_list_of_pair_value() {
        assert_eq!("List (Pair String Uint128)", Vec::<(String, u128)>::scilla_type());
        let scilla_value = vec![("salam".to_string(), 1u128), ("salam2".to_string(), 2u128)].to_value();
        // let scilla_value = serde_json::to_string(&scilla_value).unwrap();
        println!("{}", serde_json::to_string_pretty(&scilla_value).unwrap());
        // assert_eq!("k1", scilla_value);
        // assert_eq!("{\"constructor\":\"Cons\",\"argtypes\":[\"String\"],\"arguments\":[\"salam\",{\"constructor\":\"Cons\",\"argtypes\":[\"String\"],\"arguments\":[\"salam2\",{\"constructor\":\"Nil\",\"argtypes\":[\"String\"],\"arguments\":[]}]}]}", scilla_value);
    }
}
