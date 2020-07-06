use super::PropertyError;
use std::collections::HashMap;
use std::hash::Hash;

pub type PropertyMap<T> = HashMap<T, Property>;

#[derive(PartialEq, Debug)]
pub enum Property {
    String(String),
    UInt(u32),
    Int(i32),
    Float(f64),
}

trait HasProperties<T> {
    fn get_string(&self, property: &T) -> Result<Option<&str>, PropertyError>;
    fn get_u32(&self, property: &T) -> Result<Option<u32>, PropertyError>;
    fn get_i32(&self, property: &T) -> Result<Option<i32>, PropertyError>;
    fn get_f64(&self, property: &T) -> Result<Option<f64>, PropertyError>;
    fn set_string(&mut self, property: T, value: String);
    fn set_u32(&mut self, property: T, value: u32);
    fn set_i32(&mut self, property: T, value: i32);
    fn set_f64(&mut self, property: T, value: f64);
}

impl<T: Eq + Hash> HasProperties<T> for PropertyMap<T> {
    fn get_string(&self, property: &T) -> Result<Option<&str>, PropertyError> {
        match self.get(property) {
            Some(Property::String(value)) => Ok(Some(value)),
            Some(value) => Err(PropertyError::IncorrectType {
                expected_type: "String".to_string(),
                actual_type: get_type_name(&value),
            }),
            None => Ok(None),
        }
    }

    fn get_u32(&self, property: &T) -> Result<Option<u32>, PropertyError> {
        match self.get(property) {
            Some(Property::UInt(value)) => Ok(Some(*value)),
            Some(value) => Err(PropertyError::IncorrectType {
                expected_type: "u32".to_string(),
                actual_type: get_type_name(&value),
            }),
            None => Ok(None),
        }
    }

    fn get_i32(&self, property: &T) -> Result<Option<i32>, PropertyError> {
        match self.get(property) {
            Some(Property::Int(value)) => Ok(Some(*value)),
            Some(value) => Err(PropertyError::IncorrectType {
                expected_type: "i32".to_string(),
                actual_type: get_type_name(&value),
            }),
            None => Ok(None),
        }
    }

    fn get_f64(&self, property: &T) -> Result<Option<f64>, PropertyError> {
        match self.get(property) {
            Some(Property::Float(value)) => Ok(Some(*value)),
            Some(value) => Err(PropertyError::IncorrectType {
                expected_type: "f64".to_string(),
                actual_type: get_type_name(&value),
            }),
            None => Ok(None),
        }
    }

    fn set_string(&mut self, property: T, value: String) {
        self.insert(property, Property::String(value));
    }

    fn set_u32(&mut self, property: T, value: u32) {
        self.insert(property, Property::UInt(value));
    }

    fn set_i32(&mut self, property: T, value: i32) {
        self.insert(property, Property::Int(value));
    }

    fn set_f64(&mut self, property: T, value: f64) {
        self.insert(property, Property::Float(value));
    }
}

fn get_type_name(value: &Property) -> String {
    match value {
        Property::String(_) => "String".to_string(),
        Property::UInt(_) => "u32".to_string(),
        Property::Int(_) => "i32".to_string(),
        Property::Float(_) => "f64".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq, Debug, Hash)]
    enum TestProperty {
        First,
        Second,
        Third,
    }

    #[test]
    fn map_get_string() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();
        map.insert(
            TestProperty::First,
            Property::String("My value".to_string()),
        );
        map.insert(TestProperty::Third, Property::UInt(42));

        let value_first = map.get_string(&TestProperty::First)?;
        let value_second = map.get_string(&TestProperty::Second)?;
        let value_error = map.get_string(&TestProperty::Third);

        assert_eq!(value_first, Some("My value"));
        assert_eq!(value_second, None);

        match value_error {
            Err(PropertyError::IncorrectType {
                expected_type,
                actual_type,
            }) => {
                assert_eq!(expected_type, "String");
                assert_eq!(actual_type, "u32");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn map_get_u32() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();
        map.insert(TestProperty::First, Property::UInt(42));
        map.insert(
            TestProperty::Third,
            Property::String("My value".to_string()),
        );

        let value_first = map.get_u32(&TestProperty::First)?;
        let value_second = map.get_u32(&TestProperty::Second)?;
        let value_error = map.get_u32(&TestProperty::Third);

        assert_eq!(value_first, Some(42));
        assert_eq!(value_second, None);

        match value_error {
            Err(PropertyError::IncorrectType {
                expected_type,
                actual_type,
            }) => {
                assert_eq!(expected_type, "u32");
                assert_eq!(actual_type, "String");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn map_get_i32() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();
        map.insert(TestProperty::First, Property::Int(42));
        map.insert(
            TestProperty::Third,
            Property::String("My value".to_string()),
        );

        let value_first = map.get_i32(&TestProperty::First)?;
        let value_second = map.get_i32(&TestProperty::Second)?;
        let value_error = map.get_i32(&TestProperty::Third);

        assert_eq!(value_first, Some(42));
        assert_eq!(value_second, None);

        match value_error {
            Err(PropertyError::IncorrectType {
                expected_type,
                actual_type,
            }) => {
                assert_eq!(expected_type, "i32");
                assert_eq!(actual_type, "String");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn map_get_f64() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();
        map.insert(TestProperty::First, Property::Float(1.234));
        map.insert(
            TestProperty::Third,
            Property::String("My value".to_string()),
        );

        let value_first = map.get_f64(&TestProperty::First)?;
        let value_second = map.get_f64(&TestProperty::Second)?;
        let value_error = map.get_f64(&TestProperty::Third);

        assert_eq!(value_first, Some(1.234));
        assert_eq!(value_second, None);

        match value_error {
            Err(PropertyError::IncorrectType {
                expected_type,
                actual_type,
            }) => {
                assert_eq!(expected_type, "f64");
                assert_eq!(actual_type, "String");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn map_set_string() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();

        map.set_string(TestProperty::First, "My value".to_string());

        assert_eq!(
            map.get(&TestProperty::First),
            Some(&Property::String("My value".to_string()))
        );
        assert_eq!(map.get(&TestProperty::Second), None);

        Ok(())
    }

    #[test]
    fn map_set_u32() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();

        map.set_u32(TestProperty::First, 42);

        assert_eq!(map.get(&TestProperty::First), Some(&Property::UInt(42)));
        assert_eq!(map.get(&TestProperty::Second), None);

        Ok(())
    }

    #[test]
    fn map_set_i32() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();

        map.set_i32(TestProperty::First, 42);

        assert_eq!(map.get(&TestProperty::First), Some(&Property::Int(42)));
        assert_eq!(map.get(&TestProperty::Second), None);

        Ok(())
    }

    #[test]
    fn map_set_f64() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();

        map.set_f64(TestProperty::First, 1.234);

        assert_eq!(map.get(&TestProperty::First), Some(&Property::Float(1.234)));
        assert_eq!(map.get(&TestProperty::Second), None);

        Ok(())
    }
}
