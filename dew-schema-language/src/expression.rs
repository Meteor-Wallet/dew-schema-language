use crate::tokenizer::DewSchemaLanguageToken;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DewSchemaLanguageExpression {
    Number(String),
    StringLiteral(String),
    Identifier(String),
    Call {
        method_name: String,
        args: Vec<DewSchemaLanguageExpression>,
    },
    Chain(Vec<DewSchemaLanguageExpression>),
}

pub struct DewSchemaLanguageParser {
    tokens: Vec<DewSchemaLanguageToken>,
    position: usize,
}

#[allow(dead_code)]
impl DewSchemaLanguageParser {
    pub fn consume(input: &str) -> Result<DewSchemaLanguageExpression, String> {
        let tokens = crate::tokenizer::tokenize(input)?;
        let mut parser = Self::new(tokens);
        parser.parse()
    }

    pub fn new(tokens: Vec<DewSchemaLanguageToken>) -> Self {
        DewSchemaLanguageParser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&DewSchemaLanguageToken> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<DewSchemaLanguageToken> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected_token: DewSchemaLanguageToken) -> Result<(), String> {
        match self.next() {
            Some(token) if token == expected_token => Ok(()),
            Some(token) => Err(format!("Expected {:?}, got {:?}", expected_token, token)),
            None => Err(format!("Expected {:?}, but got EOF", expected_token)),
        }
    }

    /// Entry point
    pub fn parse(&mut self) -> Result<DewSchemaLanguageExpression, String> {
        let mut chain = Vec::new();
        chain.push(self.parse_atom()?);

        while let Some(DewSchemaLanguageToken::Dot) = self.peek() {
            self.next(); // consume dot
            chain.push(self.parse_atom()?);
        }

        if chain.len() == 1 {
            Ok(chain.remove(0))
        } else {
            Ok(DewSchemaLanguageExpression::Chain(chain))
        }
    }

    fn parse_atom(&mut self) -> Result<DewSchemaLanguageExpression, String> {
        match self.next() {
            Some(DewSchemaLanguageToken::Number(n)) => Ok(DewSchemaLanguageExpression::Number(n)),
            Some(DewSchemaLanguageToken::StringLiteral(s)) => {
                Ok(DewSchemaLanguageExpression::StringLiteral(s))
            }
            Some(DewSchemaLanguageToken::Identifier(name)) => {
                // function call or just identifier
                if let Some(DewSchemaLanguageToken::LeftParenthesis) = self.peek() {
                    self.next(); // consume '('
                    let mut args = Vec::new();

                    if let Some(DewSchemaLanguageToken::RightParenthesis) = self.peek() {
                        self.next(); // consume ')'
                    } else {
                        loop {
                            args.push(self.parse()?);
                            match self.peek() {
                                Some(DewSchemaLanguageToken::Comma) => {
                                    self.next(); // consume ','
                                }
                                Some(DewSchemaLanguageToken::RightParenthesis) => {
                                    self.next(); // consume ')'
                                    break;
                                }
                                other => {
                                    return Err(format!("Unexpected token in args: {:?}", other))
                                }
                            }
                        }
                    }

                    Ok(DewSchemaLanguageExpression::Call {
                        method_name: name,
                        args,
                    })
                } else {
                    Ok(DewSchemaLanguageExpression::Identifier(name))
                }
            }
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let input = r#"foo.bar(123, "hello")"#;
        let expected = DewSchemaLanguageExpression::Chain(vec![
            DewSchemaLanguageExpression::Identifier("foo".into()),
            DewSchemaLanguageExpression::Call {
                method_name: "bar".into(),
                args: vec![
                    DewSchemaLanguageExpression::Number("123".into()),
                    DewSchemaLanguageExpression::StringLiteral("hello".into()),
                ],
            },
        ]);
        let result = DewSchemaLanguageParser::consume(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_with_whitespace() {
        let input = r#"foo(
        bar(
            "Hello, World!"
        , 42)
    )"#;
        let expected = DewSchemaLanguageExpression::Call {
            method_name: "foo".into(),
            args: vec![DewSchemaLanguageExpression::Call {
                method_name: "bar".into(),
                args: vec![
                    DewSchemaLanguageExpression::StringLiteral("Hello, World!".into()),
                    DewSchemaLanguageExpression::Number("42".into()),
                ],
            }],
        };
        let result = DewSchemaLanguageParser::consume(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_simple_number() {
        let input = r#"add(42, 38)"#;
        let expected = DewSchemaLanguageExpression::Call {
            method_name: "add".into(),
            args: vec![
                DewSchemaLanguageExpression::Number("42".into()),
                DewSchemaLanguageExpression::Number("38".into()),
            ],
        };
        let result = DewSchemaLanguageParser::consume(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_simple_negative_number() {
        let input = r#"-42.sub(-20)"#;
        let expected = DewSchemaLanguageExpression::Chain(vec![
            DewSchemaLanguageExpression::Number("-42".into()),
            DewSchemaLanguageExpression::Call {
                method_name: "sub".into(),
                args: vec![DewSchemaLanguageExpression::Number("-20".into())],
            },
        ]);
        let result = DewSchemaLanguageParser::consume(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_complex_numbers() {
        let input = r#"-3.2e3.mul(1e20)"#;
        let expected = DewSchemaLanguageExpression::Chain(vec![
            DewSchemaLanguageExpression::Number("-3.2e3".into()),
            DewSchemaLanguageExpression::Call {
                method_name: "mul".into(),
                args: vec![DewSchemaLanguageExpression::Number("1e20".into())],
            },
        ]);
        let result = DewSchemaLanguageParser::consume(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_string_with_escape() {
        let input = r#"alert("hello, \"world\"!")"#;
        let expected = DewSchemaLanguageExpression::Call {
            method_name: "alert".into(),
            args: vec![DewSchemaLanguageExpression::StringLiteral(
                r#"hello, "world"!"#.into(),
            )],
        };
        let result = DewSchemaLanguageParser::consume(input).unwrap();
        assert_eq!(result, expected);
    }
}
