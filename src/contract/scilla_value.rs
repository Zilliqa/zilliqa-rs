use std::collections::HashMap;

use serde::Deserialize;

use crate::crypto::ZilAddress;

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Primitive(String),
    Map(Vec<KeyVal>),
    Adt(AdtValue),
}

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
pub struct KeyVal {
    key: Value,
    val: Value,
}

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
pub struct ScillaValue {
    pub vname: String,
    pub r#type: String,
    pub value: Value,
}

impl ScillaValue {
    pub fn new(vname: String, r#type: String, value: Value) -> Self {
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
    arguments: Vec<Value>,
}

// TODO: Set better names for trait functions
pub trait ToScillaValue {
    fn to_value(&self) -> Value;
    fn scilla_type() -> String;
}

macro_rules! to_scilla_value_for {
    ($t:ty, $scilla_type:expr) => {
        impl ToScillaValue for $t {
            fn to_value(&self) -> Value {
                Value::Primitive(self.to_string())
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

impl<T: ToScillaValue> ToScillaValue for Option<T> {
    fn to_value(&self) -> Value {
        match self {
            Some(v) => Value::Adt(AdtValue {
                constructor: "Some".to_string(),
                argtypes: vec![T::scilla_type()],
                arguments: vec![v.to_value()],
            }),
            None => Value::Adt(AdtValue {
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

impl ToScillaValue for bool {
    fn to_value(&self) -> Value {
        Value::Adt(AdtValue {
            constructor: if *self { "True".to_string() } else { "False".to_string() },
            argtypes: vec![],
            arguments: vec![],
        })
    }

    fn scilla_type() -> String {
        "Bool".to_string()
    }
}

impl<T: ToScillaValue, U: ToScillaValue> ToScillaValue for (T, U) {
    fn to_value(&self) -> Value {
        Value::Adt(AdtValue {
            constructor: "Pair".to_string(),
            argtypes: vec![T::scilla_type(), U::scilla_type()],
            arguments: vec![self.0.to_value(), self.1.to_value()],
        })
    }

    fn scilla_type() -> String {
        format!("Pair {} {}", T::scilla_type(), U::scilla_type())
    }
}

impl<K: ToScillaValue, V: ToScillaValue> ToScillaValue for HashMap<K, V> {
    fn to_value(&self) -> Value {
        Value::Map(
            self.iter()
                .map(|(key, value)| KeyVal {
                    key: key.to_value(),
                    val: value.to_value(),
                })
                .collect(),
        )
    }

    fn scilla_type() -> String {
        format!("Map {} {}", K::scilla_type(), V::scilla_type())
    }
}

impl<T: ToScillaValue> ToScillaValue for [T] {
    fn to_value(&self) -> Value {
        if self.is_empty() {
            Value::Adt(AdtValue {
                constructor: "Nil".to_string(),
                argtypes: vec![T::scilla_type()],
                arguments: vec![],
            })
        } else {
            Value::Adt(AdtValue {
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
    fn to_value(&self) -> Value {
        self[..].to_value()
    }

    fn scilla_type() -> String {
        <[T]>::scilla_type()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::crypto::ZilAddress;

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
