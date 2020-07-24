use crate::mol::errors::PropertyError;
use std::any::{type_name, Any};
use std::collections::HashMap;
use std::hash::Hash;

pub type PropertyMap<T> = HashMap<T, Box<dyn Any>>;

pub trait HasProperties<T> {
    fn get_property<U: 'static + Copy>(&self, property: &T) -> Result<Option<U>, PropertyError>;
    fn get_property_ref<U: 'static>(&self, property: &T) -> Result<Option<&U>, PropertyError>;
    fn get_string(&self, property: &T) -> Result<Option<&str>, PropertyError>;
    fn set_property<U: 'static>(&mut self, property: T, value: U);
}

impl<T: Eq + Hash> HasProperties<T> for PropertyMap<T> {
    fn get_property<U: 'static + Copy>(&self, property: &T) -> Result<Option<U>, PropertyError> {
        match self.get_property_ref(property) {
            Ok(Some(value)) => Ok(Some(*value)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn get_property_ref<U: 'static>(&self, property: &T) -> Result<Option<&U>, PropertyError> {
        match self.get(property) {
            Some(value) => match value.downcast_ref::<U>() {
                Some(value) => Ok(Some(value)),
                None => Err(PropertyError::IncorrectType {
                    expected_type: type_name::<U>().to_string(),
                }),
            },
            None => Ok(None),
        }
    }

    fn get_string(&self, property: &T) -> Result<Option<&str>, PropertyError> {
        match self.get_property_ref::<String>(property) {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn set_property<U: 'static>(&mut self, property: T, value: U) {
        self.insert(property, Box::new(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq, Debug, Hash)]
    enum TestProperty {
        StringProp,
        IntProp,
        Undefined,
    }

    #[test]
    fn get_property() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();
        map.insert(TestProperty::StringProp, Box::new("My value".to_string()));
        map.insert(TestProperty::IntProp, Box::new(42));

        let value_int = map.get_property::<i32>(&TestProperty::IntProp)?;
        let value_undefined = map.get_property::<i32>(&TestProperty::Undefined)?;

        assert_eq!(value_int, Some(42));
        assert_eq!(value_undefined, None);

        match map.get_property::<i32>(&TestProperty::StringProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "i32");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn get_property_ref() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();
        map.insert(TestProperty::StringProp, Box::new("My value".to_string()));
        map.insert(TestProperty::IntProp, Box::new(42));

        let value_string = map.get_property_ref::<String>(&TestProperty::StringProp)?;
        let value_int = map.get_property_ref::<i32>(&TestProperty::IntProp)?;
        let value_undefined = map.get_property_ref::<i32>(&TestProperty::Undefined)?;

        assert_eq!(value_string, Some(&"My value".to_string()));
        assert_eq!(value_int, Some(&42));
        assert_eq!(value_undefined, None);

        match map.get_property_ref::<String>(&TestProperty::IntProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "alloc::string::String");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        match map.get_property_ref::<i32>(&TestProperty::StringProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "i32");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn get_string() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();
        map.insert(TestProperty::StringProp, Box::new("My value".to_string()));
        map.insert(TestProperty::IntProp, Box::new(42));

        let value_string = map.get_string(&TestProperty::StringProp)?;
        let value_undefined = map.get_string(&TestProperty::Undefined)?;

        assert_eq!(value_string, Some("My value"));
        assert_eq!(value_undefined, None);

        match map.get_string(&TestProperty::IntProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "alloc::string::String");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn set_property() -> Result<(), PropertyError> {
        let mut map: PropertyMap<TestProperty> = PropertyMap::new();

        map.set_property(TestProperty::StringProp, "My value");
        map.set_property(TestProperty::IntProp, 42);

        assert_eq!(
            map.get(&TestProperty::StringProp)
                .unwrap()
                .downcast_ref::<&str>(),
            Some(&"My value")
        );
        assert_eq!(
            map.get(&TestProperty::IntProp)
                .unwrap()
                .downcast_ref::<i32>(),
            Some(&42)
        );
        assert!(map.get(&TestProperty::Undefined).is_none());

        Ok(())
    }
}
