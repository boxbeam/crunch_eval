pub struct ParserState<'a> {
    source: &'a [char],
    pos: usize,
}

#[derive(Debug)]
pub enum ParserError {
    ExpectedChar(char),
    ExpectedStr(&'static str),
    ExpectedToken(&'static str),
    MissingOperand,
    DanglingValue,
    NoValue,
}

impl ParserState<'_> {
    pub fn peek(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.source.get(self.pos).copied();
        self.pos += 1;
        c
    }

    pub fn assert_char(&mut self, c: char) -> Result<(), ParserError> {
        if self.advance() == Some(c) {
            Ok(())
        } else {
            Err(ParserError::ExpectedChar(c))
        }
    }

    pub fn assert_str(&mut self, str: &'static str) -> Result<(), ParserError> {
        str.chars().try_for_each(|c| self.assert_char(c)).map(|_| ())
    }

    pub fn take_while(&mut self, token_type: &'static str, filter: fn(char) -> bool) -> Result<String, ParserError> {
        let collected = self.source[self.pos..].iter().take_while(|&c| filter(*c)).collect::<String>();
        if collected.len() > 0 {
            self.pos += collected.len();
            Ok(collected)
        } else {
            Err(ParserError::ExpectedToken(token_type))
        }
    }

    pub fn skip_whitespace(&mut self) {
        self.pos += self.source[self.pos..].iter().take_while(|&c| c.is_whitespace()).count();
    }

    pub fn has_char(&self, c: char) -> bool {
        self.peek() == Some(c)
    }

    pub fn check_char(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    pub fn check_str(&mut self, str: &'static str) -> bool {
        let start = self.pos;
        if self.assert_str(str).is_ok() {
            true
        } else {
            self.pos = start;
            false
        }
    }
}