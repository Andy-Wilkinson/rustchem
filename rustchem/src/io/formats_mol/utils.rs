use crate::io::utils::{parse_f64, parse_i32, parse_u32, parse_usize};
use crate::io::ParseError;

pub fn parse_u32_default(val: &str, dest_nature: &str) -> Result<u32, ParseError> {
    if val.trim().is_empty() {
        Ok(0)
    } else {
        parse_u32(val, dest_nature)
    }
}

pub fn parse_i32_default(val: &str, dest_nature: &str) -> Result<i32, ParseError> {
    if val.trim().is_empty() {
        Ok(0)
    } else {
        parse_i32(val, dest_nature)
    }
}

pub fn parse_f64_default(val: &str, dest_nature: &str) -> Result<f64, ParseError> {
    if val.trim().is_empty() {
        Ok(0.0)
    } else {
        parse_f64(val, dest_nature)
    }
}

pub fn parse_usize_default(val: &str, dest_nature: &str) -> Result<usize, ParseError> {
    if val.trim().is_empty() {
        Ok(0)
    } else {
        parse_usize(val, dest_nature)
    }
}
