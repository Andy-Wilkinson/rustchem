use std::io::{BufRead, BufReader, Lines, Read};

pub struct LineReader<R> {
    lines: Lines<BufReader<R>>,
}

impl<R: Read> LineReader<R> {
    pub fn new(reader: R) -> LineReader<R> {
        let buf = BufReader::new(reader);
        LineReader { lines: buf.lines() }
    }
}

impl<R: Read> LineReader<R> {
    pub fn read_line_optional(&mut self) -> Option<std::io::Result<String>> {
        self.lines.next()
    }

    pub fn read_line(&mut self) -> std::io::Result<String> {
        match self.read_line_optional() {
            Some(result) => result,
            None => Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected end of file",
            )),
        }
    }

    pub fn read_lines<'a>(&'a mut self, count: u32) -> LineReaderIter<'a, R> {
        LineReaderIter {
            line_reader: self,
            count,
        }
    }
}

pub struct LineReaderIter<'a, R> {
    line_reader: &'a mut LineReader<R>,
    count: u32,
}

impl<'a, R: Read> Iterator for LineReaderIter<'a, R> {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<std::io::Result<String>> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.line_reader.read_line())
        } else {
            None
        }
    }
}
