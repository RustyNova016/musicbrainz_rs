use serde_json::Value;

pub trait NullEq {
    /// Compares two json values for equality
    fn null_eq(&self, other: &Self) -> bool;
}

impl NullEq for Value {
    fn null_eq(&self, other: &Self) -> bool {
        match self {
            Value::Null => other.is_null(),

            Value::Array(val) => other.as_array().is_some_and(|val2| val.null_eq(val2)),
            Value::Object(val) => other.as_object().is_some_and(|val2| val.null_eq(val2)),

            Value::Bool(val) => other.as_bool().is_some_and(|val2| val.eq(&val2)),
            Value::Number(val) => other.as_number().is_some_and(|val2| val.eq(val2)),
            Value::String(val) => other.as_str().is_some_and(|val2| val.eq(val2)),
        }
    }
}

impl NullEq for Vec<Value> {
    /// Compares two json arrays for equality. This doesn't check for order
    fn null_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if !self
                .get(i)
                .is_some_and(|val1| other.get(i).is_some_and(|val2| val1.null_eq(val2)))
            {
                return false;
            }
        }

        true
    }
}

impl NullEq for serde_json::Map<String, Value> {
    fn null_eq(&self, other: &Self) -> bool {
        for (key, value) in self.iter() {
            let other_value = other.get(key).unwrap_or(&Value::Null);

            if !value.null_eq(other_value) {
                return false;
            }
        }

        true
    }
}
