use regex::{Captures, Match, Regex};

use crate::class::location::Location;
use crate::class::token::{Token, TokenType, TOKEN_REGEXES};
use crate::compiler::CompilerError;

pub fn tokenize_code_file(file: &str) -> Result<Vec<Token>, CompilerError> {
    let code: String = match std::fs::read_to_string(file) {
        Ok(string) => string,
        Err(error) => return Err(CompilerError::IOError(error)),
    };
    Ok(tokenize_code(&code, Some(file.to_string())))
}

pub fn tokenize_code(code: &str, code_file: Option<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut row: usize = 1;
    let mut column: usize = 1;
    let mut cursor: usize = 0;
    loop {
        let token: Option<Token> = get_next_token(
            &code,
            code_file.clone(),
            &mut cursor,
            &mut row,
            &mut column
        );
        if token.is_none() {
            break;
        }
        tokens.push(token.unwrap());
    }
    return tokens;
}

fn get_next_token(
    code: &str,
    code_file: Option<String>,
    cursor: &mut usize,
    row: &mut usize,
    column: &mut usize,
) -> Option<Token> {
    if *cursor >= code.len() {
        return None;
    }

    // Test if the remaining code matches with any Token regex
    let unparsed_code: &str = code.split_at(*cursor).1;
    for (regex, token_type) in TOKEN_REGEXES.entries() {
        let captures: Option<Captures> = Regex::new(regex).unwrap().captures(unparsed_code);
        if !captures.is_none() {
            // Take match from capture group if it is explicitly specified
            let whole_match: Option<Match> = captures.as_ref().unwrap().get(0);
            let mut token_match: Option<Match> = captures.unwrap().get(1);
            if token_match.is_none() {
                token_match = whole_match;
            }

            // Save the old row and column
            let token_row: usize = *row;
            let token_column: usize = *column;

            // Calculate the new row and column after the string
            let match_str = token_match.unwrap().as_str();
            let newline_count = match_str.matches("\n").count();
            if newline_count > 0 {
                *column = match_str.len() - match_str.rfind("\n").unwrap_or(0);
            } else {
                *column += match_str.len();
            }
            *row += newline_count;

            // Move cursor to the end of the parsed Token
            *cursor += whole_match.unwrap().end();

            // Token should be skipped, e.g. whitespace or comment
            if token_type == &TokenType::None {
                return get_next_token(code, code_file, cursor, row, column);
            }
            return Some(Token {
                value: match_str.to_string(),
                typ: get_token_type(match_str),
                location: Location::new(token_row, token_column, code_file),
            });
        }
    }

    // TODO: Enhance error reporting
    panic!(
        "Unknown Token at the start of the following code:\n{}",
        unparsed_code
    )
}

pub fn get_token_type(token: &str) -> TokenType {
    for (regex, token_type) in TOKEN_REGEXES.entries() {
        // Take match from capture group if it is explicitly specified
        let is_match: bool = Regex::new(regex).unwrap().is_match(token);
        if is_match {
            return token_type.clone();
        }
    }
    panic!("Did not get TokenType for '{}'", token);
}

#[cfg(test)]
mod tests {
    use strum::{EnumCount, IntoEnumIterator};

    use super::*;
    use crate::{
        class::token::{Delimiter, Keyword},
        constant::TEST_FOLDER,
        data_types::{ChunkSize, DataType},
    };

    #[test]
    fn lex_nonexistent_file() {
        match tokenize_code_file("nonexistent.rot") {
            Ok(_) => panic!("Lexer should not be able to tokenize nonexistent file"),
            Err(error) => match error {
                CompilerError::IOError(error) => {
                    let expected_error_kind = std::io::ErrorKind::NotFound;
                    if error.kind() != expected_error_kind {
                        panic!(
                            "Unexpected IO error '{}'. Expected error to be '{}'",
                            error.kind(),
                            expected_error_kind
                        )
                    }
                }
                _ => panic!("Expected IO error"),
            },
        }
    }

    #[test]
    fn lex_empty_file() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/lex_empty.rot")).unwrap();
        assert!(tokens.is_empty())
    }

    #[test]
    fn lex_whitespace() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/lex_whitespace.rot")).unwrap();
        assert!(tokens.is_empty())
    }

    #[test]
    fn lex_comments() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/lex_comments.rot")).unwrap();
        assert!(tokens.is_empty())
    }

    #[test]
    fn lex_delimiters() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/lex_delimiters.rot")).unwrap();
        // Are all Delimiters taken into account in the test file
        assert_eq!(tokens.len(), Delimiter::COUNT);
        // Are tokens lexed correctly as certain delimiters
        for (i, delimiter) in Delimiter::iter().enumerate() {
            assert_eq!(TokenType::Delimiter(delimiter), tokens[i].typ)
        }
    }

    #[test]
    fn lex_literals() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/lex_literals.rot")).unwrap();
        // Are all DataTypes taken into account in the test file
        assert_eq!(tokens.len(), DataType::COUNT);
        // Are tokens lexed correctly as literal with certain type
        for (i, data_type) in DataType::iter().enumerate() {
            // Pointer literals do not exist
            if data_type == DataType::Pointer || i >= DataType::iter().len() - 1 {
                continue;
            }
            assert_eq!(TokenType::Literal(data_type), tokens[i].typ)
        }
    }

    #[test]
    fn lex_keywords() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/lex_keywords.rot")).unwrap();
        // Are all keyword operators taken into account in the test file
        assert_eq!(tokens.len(), Keyword::COUNT);
        // Are tokens lexed correctly as certain keywords
        for (i, keyword) in Keyword::iter().enumerate() {
            assert_eq!(TokenType::Keyword(keyword), tokens[i].typ)
        }
    }
}
