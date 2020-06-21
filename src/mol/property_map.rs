use std::collections::HashMap;

pub type PropertyMap<T> = HashMap<T, Property>;

#[derive(Debug)]
pub enum Property {
    UInt(u32),
    Int(i32),
    Float(f64),
}