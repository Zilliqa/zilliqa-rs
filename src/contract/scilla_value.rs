use serde::Deserialize;

use crate::crypto::ZilAddress;

#[derive(serde::Serialize, Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Primitive(String),
    Adt(AdtValue),
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

#[cfg(test)]
mod tests {
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
}
