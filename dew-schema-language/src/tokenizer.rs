#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DewSchemaLanguageToken {
    Number(String),
    Identifier(String),
    StringLiteral(String),
    Dot,
    Comma,
    LeftParenthesis,
    RightParenthesis,
}

pub fn tokenize(input: &str) -> Result<Vec<DewSchemaLanguageToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // skip whitespace
            c if c.is_whitespace() => {
                chars.next();
            }

            // numbers (supports floats and scientific notation)
            c if c.is_ascii_digit() || c == '-' => {
                let mut number = String::new();
                let mut seen_dot = false;
                let mut seen_exp = false;

                // allow leading negative sign
                if c == '-' {
                    number.push(c);
                    chars.next(); // consume sign
                }

                while let Some(&c2) = chars.peek() {
                    if c2.is_ascii_digit() {
                        number.push(c2);
                        chars.next();
                    } else if c2 == '.' {
                        // only allow one dot and only if followed by a digit
                        let mut iter = chars.clone();
                        iter.next(); // skip the dot
                        if seen_dot
                            || seen_exp
                            || iter.peek().map(|d| d.is_ascii_digit()) != Some(true)
                        {
                            break; // stop parsing number
                        }
                        seen_dot = true;
                        number.push(c2);
                        chars.next();
                    } else if c2 == 'e' || c2 == 'E' {
                        if seen_exp {
                            break; // only one exponent allowed
                        }
                        seen_exp = true;
                        number.push(c2);
                        chars.next();

                        // optional sign after e/E
                        if let Some(&sign) = chars.peek() {
                            if sign == '+' || sign == '-' {
                                number.push(sign);
                                chars.next();
                            }
                        }
                    } else {
                        break;
                    }
                }

                tokens.push(DewSchemaLanguageToken::Number(number));
            }

            // identifiers
            c if c.is_alphabetic() || c == '_' || c == '$' => {
                let mut identifier = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2.is_alphanumeric() || c2 == '_' || c2 == '$' {
                        identifier.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(DewSchemaLanguageToken::Identifier(identifier));
            }

            // string literals
            '"' => {
                chars.next(); // consume opening "
                let mut literal = String::new();
                let mut escaped = false;

                while let Some(c2) = chars.next() {
                    if escaped {
                        // handle escaped characters
                        literal.push(c2);
                        escaped = false;
                    } else if c2 == '\\' {
                        escaped = true;
                    } else if c2 == '"' {
                        // end of string
                        break;
                    } else {
                        literal.push(c2);
                    }
                }

                tokens.push(DewSchemaLanguageToken::StringLiteral(literal));
            }

            // punctuation
            '.' => {
                chars.next();
                tokens.push(DewSchemaLanguageToken::Dot);
            }
            ',' => {
                chars.next();
                tokens.push(DewSchemaLanguageToken::Comma);
            }
            '(' => {
                chars.next();
                tokens.push(DewSchemaLanguageToken::LeftParenthesis);
            }
            ')' => {
                chars.next();
                tokens.push(DewSchemaLanguageToken::RightParenthesis);
            }

            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let input = r#"foo.bar(123, "hello")"#;
        let expected = vec![
            DewSchemaLanguageToken::Identifier("foo".into()),
            DewSchemaLanguageToken::Dot,
            DewSchemaLanguageToken::Identifier("bar".into()),
            DewSchemaLanguageToken::LeftParenthesis,
            DewSchemaLanguageToken::Number("123".into()),
            DewSchemaLanguageToken::Comma,
            DewSchemaLanguageToken::StringLiteral("hello".into()),
            DewSchemaLanguageToken::RightParenthesis,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_with_whitespace() {
        let input = r#"foo(
    bar(
        "Hello, World!"
    , 42)
)"#;
        let expected = vec![
            DewSchemaLanguageToken::Identifier("foo".into()),
            DewSchemaLanguageToken::LeftParenthesis,
            DewSchemaLanguageToken::Identifier("bar".into()),
            DewSchemaLanguageToken::LeftParenthesis,
            DewSchemaLanguageToken::StringLiteral("Hello, World!".into()),
            DewSchemaLanguageToken::Comma,
            DewSchemaLanguageToken::Number("42".into()),
            DewSchemaLanguageToken::RightParenthesis,
            DewSchemaLanguageToken::RightParenthesis,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_simple_number() {
        let input = r#"add(42, 38)"#;
        let expected = vec![
            DewSchemaLanguageToken::Identifier("add".into()),
            DewSchemaLanguageToken::LeftParenthesis,
            DewSchemaLanguageToken::Number("42".into()),
            DewSchemaLanguageToken::Comma,
            DewSchemaLanguageToken::Number("38".into()),
            DewSchemaLanguageToken::RightParenthesis,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_simple_negative_number() {
        let input = r#"-42.sub(-20)"#;
        let expected = vec![
            DewSchemaLanguageToken::Number("-42".into()),
            DewSchemaLanguageToken::Dot,
            DewSchemaLanguageToken::Identifier("sub".into()),
            DewSchemaLanguageToken::LeftParenthesis,
            DewSchemaLanguageToken::Number("-20".into()),
            DewSchemaLanguageToken::RightParenthesis,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_complex_numbers() {
        let input = r#"-3.2e3.mul(1e20)"#;
        let expected = vec![
            DewSchemaLanguageToken::Number("-3.2e3".into()),
            DewSchemaLanguageToken::Dot,
            DewSchemaLanguageToken::Identifier("mul".into()),
            DewSchemaLanguageToken::LeftParenthesis,
            DewSchemaLanguageToken::Number("1e20".into()),
            DewSchemaLanguageToken::RightParenthesis,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenize_string_with_escape() {
        let input = r#"alert("hello, \"world\"!")"#;
        let expected = vec![
            DewSchemaLanguageToken::Identifier("alert".into()),
            DewSchemaLanguageToken::LeftParenthesis,
            DewSchemaLanguageToken::StringLiteral(r#"hello, "world"!"#.into()),
            DewSchemaLanguageToken::RightParenthesis,
        ];
        let result = tokenize(input).unwrap();
        assert_eq!(result, expected);
    }
}
