use crate::mol::errors::PropertyError;
use std::any::{type_name, Any};
use std::collections::HashMap;
use std::hash::Hash;

pub type PropertyMap<T> = HashMap<T, Box<dyn Any>>;

pub trait HasProperties<T: 'static + Eq + Hash> {
    fn get_property_map(&self) -> &PropertyMap<T>;
    fn get_property_map_mut(&mut self) -> &mut PropertyMap<T>;

    fn get_property<U: 'static + Copy>(&self, property: &T) -> Result<Option<U>, PropertyError> {
        match self.get_property_ref(property) {
            Ok(Some(value)) => Ok(Some(*value)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn get_property_ref<U: 'static>(&self, property: &T) -> Result<Option<&U>, PropertyError> {
        match self.get_property_map().get(property) {
            Some(value) => match value.downcast_ref::<U>() {
                Some(value) => Ok(Some(value)),
                None => Err(PropertyError::IncorrectType {
                    expected_type: type_name::<U>().to_string(),
                }),
            },
            None => Ok(None),
        }
    }

    fn get_property_string(&self, property: &T) -> Result<Option<&str>, PropertyError> {
        match self.get_property_ref::<String>(property) {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn set_property<U: 'static>(&mut self, property: T, value: U) {
        self.get_property_map_mut()
            .insert(property, Box::new(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct {
        properties: PropertyMap<TestProperty>,
    }

    impl TestStruct {
        pub fn new() -> TestStruct {
            TestStruct {
                properties: PropertyMap::new(),
            }
        }
    }

    impl HasProperties<TestProperty> for TestStruct {
        fn get_property_map(&self) -> &PropertyMap<TestProperty> {
            &self.properties
        }
        fn get_property_map_mut(&mut self) -> &mut PropertyMap<TestProperty> {
            &mut self.properties
        }
    }

    #[derive(Eq, PartialEq, Debug, Hash)]
    enum TestProperty {
        StringProp,
        IntProp,
        Undefined,
    }

    #[test]
    fn get_property() -> Result<(), PropertyError> {
        let mut obj = TestStruct::new();
        let map = obj.get_property_map_mut();
        map.insert(TestProperty::StringProp, Box::new("My value".to_string()));
        map.insert(TestProperty::IntProp, Box::new(42));

        let value_int = obj.get_property::<i32>(&TestProperty::IntProp)?;
        let value_undefined = obj.get_property::<i32>(&TestProperty::Undefined)?;

        assert_eq!(value_int, Some(42));
        assert_eq!(value_undefined, None);

        match obj.get_property::<i32>(&TestProperty::StringProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "i32");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn get_property_ref() -> Result<(), PropertyError> {
        let mut obj = TestStruct::new();
        let map = obj.get_property_map_mut();
        map.insert(TestProperty::StringProp, Box::new("My value".to_string()));
        map.insert(TestProperty::IntProp, Box::new(42));

        let value_string = obj.get_property_ref::<String>(&TestProperty::StringProp)?;
        let value_int = obj.get_property_ref::<i32>(&TestProperty::IntProp)?;
        let value_undefined = obj.get_property_ref::<i32>(&TestProperty::Undefined)?;

        assert_eq!(value_string, Some(&"My value".to_string()));
        assert_eq!(value_int, Some(&42));
        assert_eq!(value_undefined, None);

        match obj.get_property_ref::<String>(&TestProperty::IntProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "alloc::string::String");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        match obj.get_property_ref::<i32>(&TestProperty::StringProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "i32");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn get_string() -> Result<(), PropertyError> {
        let mut obj = TestStruct::new();
        let map = obj.get_property_map_mut();
        map.insert(TestProperty::StringProp, Box::new("My value".to_string()));
        map.insert(TestProperty::IntProp, Box::new(42));

        let value_string = obj.get_property_string(&TestProperty::StringProp)?;
        let value_undefined = obj.get_property_string(&TestProperty::Undefined)?;

        assert_eq!(value_string, Some("My value"));
        assert_eq!(value_undefined, None);

        match obj.get_property_string(&TestProperty::IntProp) {
            Err(PropertyError::IncorrectType { expected_type }) => {
                assert_eq!(expected_type, "alloc::string::String");
            }
            _ => panic!("Expected PropertyError::Parse"),
        }

        Ok(())
    }

    #[test]
    fn set_property() -> Result<(), PropertyError> {
        let mut obj = TestStruct::new();
        obj.set_property(TestProperty::StringProp, "My value");
        obj.set_property(TestProperty::IntProp, 42);

        let map = obj.get_property_map();

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
