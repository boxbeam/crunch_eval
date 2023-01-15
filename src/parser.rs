pub(crate) struct ParserState<'a> {
    pub source: &'a [char],
    pub pos: usize,
}

#[derive(Debug)]
pub enum ParserError {
    ExpectedChar(usize, char),
    ExpectedStr(usize, &'static str),
    ExpectedToken(usize, &'static str),
    MissingOperand(usize),
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
            Err(ParserError::ExpectedChar(self.pos, c))
        }
    }

    pub fn take_while(&mut self, token_type: &'static str, filter: fn(char) -> bool) -> Result<String, ParserError> {
        let collected = self.source[self.pos..].iter().take_while(|&c| filter(*c)).collect::<String>();
        if collected.len() > 0 {
            self.pos += collected.len();
            Ok(collected)
        } else {
            Err(ParserError::ExpectedToken(self.pos, token_type))
        }
    }

    pub fn check_char(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
}