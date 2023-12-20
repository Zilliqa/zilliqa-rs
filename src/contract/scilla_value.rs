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
    fn to_value(self) -> Value;
    fn scilla_type() -> String;
}

impl ToScillaValue for i32 {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "Int32".to_string()
    }
}

impl ToScillaValue for i64 {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "Int64".to_string()
    }
}

impl ToScillaValue for i128 {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "Int128".to_string()
    }
}

impl ToScillaValue for u32 {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "Uint32".to_string()
    }
}

impl ToScillaValue for u64 {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "Uint64".to_string()
    }
}

impl ToScillaValue for u128 {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "Uint128".to_string()
    }
}

impl ToScillaValue for primitive_types::U256 {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "Uint256".to_string()
    }
}

impl ToScillaValue for String {
    fn to_value(self) -> Value {
        Value::Primitive(self)
    }

    fn scilla_type() -> String {
        "String".to_string()
    }
}

impl ToScillaValue for &str {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "String".to_string()
    }
}

impl ToScillaValue for &ZilAddress {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "ByStr20".to_string()
    }
}

impl ToScillaValue for ZilAddress {
    fn to_value(self) -> Value {
        Value::Primitive(self.to_string())
    }

    fn scilla_type() -> String {
        "ByStr20".to_string()
    }
}

impl<T: ToScillaValue> ToScillaValue for Option<T> {
    fn to_value(self) -> Value {
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
    fn to_value(self) -> Value {
        Value::Adt(AdtValue {
            constructor: if self { "True".to_string() } else { "False".to_string() },
            argtypes: vec![],
            arguments: vec![],
        })
    }

    fn scilla_type() -> String {
        "Bool".to_string()
    }
}

impl<T: ToScillaValue, U: ToScillaValue> ToScillaValue for (T, U) {
    fn to_value(self) -> Value {
        Value::Adt(AdtValue {
            constructor: "Pair".to_string(),
            argtypes: vec![T::scilla_type(), U::scilla_type()],
            arguments: vec![self.0.to_value(), self.1.to_value()],
        })
    }

    fn scilla_type() -> String {
        format!("Pair ({} {})", T::scilla_type(), U::scilla_type())
    }
}

impl<K: ToScillaValue, V: ToScillaValue> ToScillaValue for HashMap<K, V> {
    fn to_value(self) -> Value {
        Value::Map(
            self.into_iter()
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::ToScillaValue;

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
        assert_eq!("Pair (String Uint32)", <(String, u32)>::scilla_type());

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
}
