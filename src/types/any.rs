use crate::{GQLScalar, InputValueResult, ScalarType, Value};
use serde::de::DeserializeOwned;

/// Any scalar (For [Apollo Federation](https://www.apollographql.com/docs/apollo-server/federation/introduction))
///
/// The `Any` scalar is used to pass representations of entities from external services into the root `_entities` field for execution.
#[derive(Clone, PartialEq, Debug)]
pub struct Any(pub Value);

/// The `_Any` scalar is used to pass representations of entities from external services into the root `_entities` field for execution.
#[GQLScalar(internal, name = "_Any")]
impl ScalarType for Any {
    fn parse(value: Value) -> InputValueResult<Self> {
        Ok(Self(value))
    }

    fn is_valid(_value: &Value) -> bool {
        true
    }

    fn to_value(&self) -> Value {
        self.0.clone()
    }
}

impl Any {
    /// Parse this `Any` value to T by `serde_json`.
    pub fn parse_value<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_value(self.to_value().into_json()?)
    }
}

impl<T: Into<Value>> From<T> for Any {
    fn from(value: T) -> Any {
        Any(value.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_conversion_ok() {
        let value = Value::List(vec![
            Value::Number(1.into()),
            Value::Boolean(true),
            Value::Null,
        ]);
        let expected = Any(value.clone());
        let output: Any = value.into();
        assert_eq!(output, expected);
    }
}
