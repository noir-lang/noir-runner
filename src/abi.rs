use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

pub use acvm::FieldElement;
pub use noirc_abi::input_parser::InputValue;

pub trait ToNoir {
    fn to_noir(self) -> InputValue;
}

impl<T: Serialize> ToNoir for T {
    fn to_noir(self) -> InputValue {
        match serde_json::to_value(self).unwrap() {
            Value::Null => InputValue::Field(0u32.into()),
            Value::Bool(b) => InputValue::Field(b.into()),
            Value::Number(n) => {
                if n.is_i64() {
                    InputValue::Field((n.as_i64().unwrap() as i128).into())
                } else if n.is_u64() {
                    InputValue::Field(n.as_u64().unwrap().into())
                } else {
                    InputValue::Field((n.as_f64().unwrap() as u64).into())
                }
            }
            Value::Array(a) => InputValue::Vec(a.into_iter().map(|v| v.to_noir()).collect()),
            Value::String(s) => InputValue::String(s),
            Value::Object(o) => {
                let map = o
                    .into_iter()
                    .map(|(k, v)| (k, v.to_noir()))
                    .collect::<BTreeMap<String, InputValue>>();

                InputValue::Struct(map)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let null = serde_json::Value::Null;

        let input_value = ToNoir::to_noir(null);

        assert_eq!(input_value, InputValue::Field(0u32.into()));
    }

    #[test]
    fn test_bool() {
        let a = true;
        let b = false;

        let input_value_a = ToNoir::to_noir(a);
        let input_value_b = ToNoir::to_noir(b);

        assert_eq!(input_value_a, InputValue::Field(1u32.into()));
        assert_eq!(input_value_b, InputValue::Field(0u32.into()));
    }

    #[test]
    fn test_number() {
        let a = 1u64;
        let b = 1.0;
        let c = 1i64;

        let input_value_a = ToNoir::to_noir(a);
        let input_value_b = ToNoir::to_noir(b);
        let input_value_c = ToNoir::to_noir(c);

        assert_eq!(input_value_a, InputValue::Field(1u32.into()));
        assert_eq!(input_value_b, InputValue::Field(1u32.into()));
        assert_eq!(input_value_c, InputValue::Field(1u32.into()));
    }

    #[test]
    fn test_array() {
        let a = vec![1u64, 1u64, 1u64];

        let input_value = ToNoir::to_noir(a);

        assert_eq!(
            input_value,
            InputValue::Vec(vec![
                InputValue::Field(1u32.into()),
                InputValue::Field(1u32.into()),
                InputValue::Field(1u32.into()),
            ])
        );
    }

    #[test]
    fn test_string() {
        let a = "hello".to_string();

        let input_value = ToNoir::to_noir(a);

        assert_eq!(input_value, InputValue::String("hello".to_string()));
    }

    #[test]
    fn test_object() {
        #[derive(Serialize)]
        struct Test {
            a: u32,
            b: String,
        }

        let a = Test {
            a: 1,
            b: "hello".to_string(),
        };

        let input_value = ToNoir::to_noir(a);

        let map = BTreeMap::from([
            ("a".to_string(), InputValue::Field(1u32.into())),
            ("b".to_string(), InputValue::String("hello".to_string())),
        ]);

        assert_eq!(input_value, InputValue::Struct(map));
    }

    #[test]
    fn test_bytes() {
        let a = [1u8, 2u8, 3u8];

        let input_value = ToNoir::to_noir(a);

        assert_eq!(
            input_value,
            InputValue::Vec(vec![
                InputValue::Field(1u32.into()),
                InputValue::Field(2u32.into()),
                InputValue::Field(3u32.into()),
            ])
        );
    }
}
