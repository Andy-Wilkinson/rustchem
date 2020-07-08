use super::ParseError;

pub fn parse_u32(val: &str, dest_nature: &str) -> Result<u32, ParseError> {
    val.trim()
        .parse::<u32>()
        .map_err(|source| ParseError::ParseInt {
            source,
            name: dest_nature.to_string(),
            value: val.to_string(),
        })
}

pub fn parse_i32(val: &str, dest_nature: &str) -> Result<i32, ParseError> {
    val.trim()
        .parse::<i32>()
        .map_err(|source| ParseError::ParseInt {
            source,
            name: dest_nature.to_string(),
            value: val.to_string(),
        })
}

pub fn parse_usize(val: &str, dest_nature: &str) -> Result<usize, ParseError> {
    val.trim()
        .parse::<usize>()
        .map_err(|source| ParseError::ParseInt {
            source,
            name: dest_nature.to_string(),
            value: val.to_string(),
        })
}

pub fn parse_f64(val: &str, dest_nature: &str) -> Result<f64, ParseError> {
    val.trim()
        .parse::<f64>()
        .map_err(|source| ParseError::ParseFloat {
            source,
            name: dest_nature.to_string(),
            value: val.to_string(),
        })
}
