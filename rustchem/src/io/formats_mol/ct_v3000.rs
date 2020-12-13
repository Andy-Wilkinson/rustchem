use crate::io::LineReader;
use lazy_static::lazy_static;
use regex::Regex;

pub fn read_v3000_line(
    line: &str,
    line_reader: &mut LineReader<impl std::io::Read>,
) -> std::io::Result<String> {
    let mut line = line.to_string();
    while line.ends_with('-') {
        line.pop();
        line.push_str(&line_reader.read_line()?[6..]);
    }
    Ok(line)
}

pub fn pop_v3000_value(line: &str) -> (String, &str) {
    let (value, rest) = if line.starts_with('"') {
        match split_quoted_string(line) {
            Some(s) => s,
            None => panic!(),
        }
    } else if let Some(index) = line.find(char::is_whitespace) {
        (&line[..index], &line[index + 1..])
    } else {
        panic!()
    };

    (value.replace("\"\"", "\""), rest)
}

fn split_quoted_string(line: &str) -> Option<(&str, &str)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^\"(?P<value>([^\"]|\"\")*)\"(?P<rest>.*)$").unwrap();
    }

    match RE.captures(line) {
        Some(cap) => Some((
            cap.name("value").unwrap().as_str(),
            cap.name("rest").unwrap().as_str(),
        )),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_v3000_line_simple() -> Result<(), Box<dyn std::error::Error>> {
        let lines = "M  V30 COUNTS 6 5 0 0 1\nNext Line\n";
        let mut reader = LineReader::new(&lines.as_bytes()[..]);
        let line = reader.read_line()?;
        let result = read_v3000_line(&line, &mut reader)?;

        assert_eq!(result, "M  V30 COUNTS 6 5 0 0 1");
        assert_eq!(reader.read_line()?, "Next Line");

        Ok(())
    }

    #[test]
    fn read_v3000_line_multiline() -> Result<(), Box<dyn std::error::Error>> {
        let lines = "M  V30 COUNTS 6-\nM  V30 5 0 0 1\nNext Line\n";
        let mut reader = LineReader::new(&lines.as_bytes()[..]);
        let line = reader.read_line()?;
        let result = read_v3000_line(&line, &mut reader)?;

        assert_eq!(result, "M  V30 COUNTS 6 5 0 0 1");
        assert_eq!(reader.read_line()?, "Next Line");

        Ok(())
    }

    #[test]
    fn pop_v3000_value_simple() -> Result<(), Box<dyn std::error::Error>> {
        let line = "Value Rest of string";
        let (value, line) = pop_v3000_value(&line);

        assert_eq!(value, "Value");
        assert_eq!(line, "Rest of string");

        Ok(())
    }

    #[test]
    fn pop_v3000_value_tabseparated() -> Result<(), Box<dyn std::error::Error>> {
        let line = "Value\tRest of string";
        let (value, line) = pop_v3000_value(&line);

        assert_eq!(value, "Value");
        assert_eq!(line, "Rest of string");

        Ok(())
    }

    #[test]
    fn pop_v3000_value_quoted() -> Result<(), Box<dyn std::error::Error>> {
        let line = "\"Value with space\" Rest of string";
        let (value, line) = pop_v3000_value(&line);

        assert_eq!(value, "Value with space");
        assert_eq!(line, " Rest of string");

        Ok(())
    }

    #[test]
    fn pop_v3000_value_quotedwithquote() -> Result<(), Box<dyn std::error::Error>> {
        let line = "\"Value \"\" quote\" Rest of string";
        let (value, line) = pop_v3000_value(&line);

        assert_eq!(value, "Value \" quote");
        assert_eq!(line, " Rest of string");

        Ok(())
    }

    #[test]
    fn pop_v3000_value_unquotedwithquote() -> Result<(), Box<dyn std::error::Error>> {
        let line = "Value\"\"quote Rest of string";
        let (value, line) = pop_v3000_value(&line);

        assert_eq!(value, "Value\"quote");
        assert_eq!(line, "Rest of string");

        Ok(())
    }
}
