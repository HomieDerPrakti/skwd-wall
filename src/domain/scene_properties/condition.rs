use super::ScenePropertyValue;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Ident(String),
    Number(f64),
    Bool(bool),
    Op(&'static str),
    Open,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operand {
    Number(f64),
    Bool(bool),
}

impl Operand {
    fn truthy(self) -> bool {
        match self {
            Self::Number(number) => number != 0.0,
            Self::Bool(flag) => flag,
        }
    }

    fn number(self) -> f64 {
        match self {
            Self::Number(number) => number,
            Self::Bool(flag) => f64::from(u8::from(flag)),
        }
    }

    fn loosely_equal(self, other: Self) -> bool {
        (self.number() - other.number()).abs() < 1e-9
    }

    fn strictly_equal(self, other: Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => (a - b).abs() < 1e-9,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            _ => false,
        }
    }
}

fn tokenize(text: &str) -> Option<Vec<Token>> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let ch = chars[index];
        if ch.is_whitespace() {
            index += 1;
        } else if ch == '(' {
            tokens.push(Token::Open);
            index += 1;
        } else if ch == ')' {
            tokens.push(Token::Close);
            index += 1;
        } else if ch.is_ascii_digit()
            || (ch == '.' && chars.get(index + 1).is_some_and(char::is_ascii_digit))
        {
            let start = index;
            while index < chars.len() && (chars[index].is_ascii_digit() || chars[index] == '.') {
                index += 1;
            }
            let text: String = chars[start..index].iter().collect();
            tokens.push(Token::Number(text.parse().ok()?));
        } else if ch.is_alphabetic() || ch == '_' {
            let start = index;
            while index < chars.len()
                && (chars[index].is_alphanumeric() || chars[index] == '_' || chars[index] == '.')
            {
                index += 1;
            }
            let word: String = chars[start..index].iter().collect();
            tokens.push(match word.as_str() {
                "true" => Token::Bool(true),
                "false" => Token::Bool(false),
                _ => Token::Ident(word),
            });
        } else {
            let rest: String = chars[index..chars.len().min(index + 3)].iter().collect();
            let op = ["===", "!==", "==", "!=", "<=", ">=", "&&", "||", "<", ">", "!"]
                .into_iter()
                .find(|op| rest.starts_with(op))?;
            tokens.push(Token::Op(op));
            index += op.len();
        }
    }
    Some(tokens)
}

struct Parser<'a, F: Fn(&str) -> Option<Operand>> {
    tokens: Vec<Token>,
    position: usize,
    lookup: &'a F,
}

impl<F: Fn(&str) -> Option<Operand>> Parser<'_, F> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn take_op(&mut self, wanted: &[&str]) -> Option<&'static str> {
        if let Some(Token::Op(op)) = self.peek()
            && wanted.contains(op)
        {
            let op = *op;
            self.position += 1;
            return Some(op);
        }
        None
    }

    fn or(&mut self) -> Option<Operand> {
        let mut left = self.and()?;
        while self.take_op(&["||"]).is_some() {
            let right = self.and()?;
            left = Operand::Bool(left.truthy() || right.truthy());
        }
        Some(left)
    }

    fn and(&mut self) -> Option<Operand> {
        let mut left = self.equality()?;
        while self.take_op(&["&&"]).is_some() {
            let right = self.equality()?;
            left = Operand::Bool(left.truthy() && right.truthy());
        }
        Some(left)
    }

    fn equality(&mut self) -> Option<Operand> {
        let mut left = self.comparison()?;
        while let Some(op) = self.take_op(&["===", "!==", "==", "!="]) {
            let right = self.comparison()?;
            left = Operand::Bool(match op {
                "===" => left.strictly_equal(right),
                "!==" => !left.strictly_equal(right),
                "==" => left.loosely_equal(right),
                _ => !left.loosely_equal(right),
            });
        }
        Some(left)
    }

    fn comparison(&mut self) -> Option<Operand> {
        let mut left = self.unary()?;
        while let Some(op) = self.take_op(&["<=", ">=", "<", ">"]) {
            let right = self.unary()?;
            let (a, b) = (left.number(), right.number());
            left = Operand::Bool(match op {
                "<=" => a <= b,
                ">=" => a >= b,
                "<" => a < b,
                _ => a > b,
            });
        }
        Some(left)
    }

    fn unary(&mut self) -> Option<Operand> {
        if self.take_op(&["!"]).is_some() {
            return Some(Operand::Bool(!self.unary()?.truthy()));
        }
        self.primary()
    }

    fn primary(&mut self) -> Option<Operand> {
        let token = self.peek()?.clone();
        self.position += 1;
        match token {
            Token::Number(number) => Some(Operand::Number(number)),
            Token::Bool(flag) => Some(Operand::Bool(flag)),
            Token::Ident(name) => (self.lookup)(name.trim_end_matches(".value")),
            Token::Open => {
                let inner = self.or()?;
                (self.peek() == Some(&Token::Close)).then(|| {
                    self.position += 1;
                    inner
                })
            }
            Token::Close | Token::Op(_) => None,
        }
    }
}

fn operand_of(value: &ScenePropertyValue) -> Option<Operand> {
    match value {
        ScenePropertyValue::Flag(flag) => Some(Operand::Bool(*flag)),
        ScenePropertyValue::Number(number) => Some(Operand::Number(*number)),
        ScenePropertyValue::Text(text) => match text.trim().to_ascii_lowercase().as_str() {
            "true" => Some(Operand::Bool(true)),
            "false" => Some(Operand::Bool(false)),
            other => other.parse().ok().map(Operand::Number),
        },
        ScenePropertyValue::Vector(_) | ScenePropertyValue::Absent => None,
    }
}

#[must_use]
pub fn evaluate<F>(condition: &str, value_of: F) -> Option<bool>
where
    F: Fn(&str) -> Option<ScenePropertyValue>,
{
    let condition = condition.trim();
    if condition.is_empty() {
        return Some(true);
    }
    let lookup = |name: &str| value_of(name).as_ref().and_then(operand_of);
    let mut parser = Parser { tokens: tokenize(condition)?, position: 0, lookup: &lookup };
    let result = parser.or()?;
    (parser.position == parser.tokens.len()).then(|| result.truthy())
}
