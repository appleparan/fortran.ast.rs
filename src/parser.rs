//! # Parser
//!
//! The parser is the second step of the compilation process.
//! It takes the output of the preprocessor and transforms it into a program.
use crate::{
    helpers::file::File,
    program::Program,
    tokens::Token,
    // variables::Variable,
};
use std::collections::HashMap;

/// This function splits the file into lines.
fn split_line(file: File) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let tmp_lines = file.get_content().split('\n');

    for line in tmp_lines.filter(|line| !(*line).is_empty()) {
        lines.push(line.to_string());
    }

    lines
}

/// This function returns the token corresponding to the word.
fn tokenize(word: String) -> Token {
    match word.to_ascii_uppercase().as_str() {
        "::" | "=" => Token::new(Token::Assign(word)),
        "+" | "-" | "*" | "/" => Token::new(Token::Operator(word)),
        "DO" => Token::new(Token::Do),
        "ELSE" => Token::new(Token::Else),
        "END" => Token::new(Token::End),
        "IF" => Token::new(Token::If),
        "PRINT" => Token::new(Token::Print),
        "PROGRAM" => Token::new(Token::Program),
        "REAL" | "INTEGER" | "CHARACTER" | "LOGICAL" => Token::new(Token::Type(word)),
        "RETURN" => Token::new(Token::Return),
        "THEN" => Token::new(Token::Then),
        "WHILE" => Token::new(Token::While),
        _ => Token::new(Token::Other(word)),
    }
}

/// This function parses a line.
fn parse_line(line: String, _pc: usize) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut tmp_word: String = String::new();
    let mut in_quote: bool = false;

    for index in 0..line.len() {
        let letter: char = line.chars().nth(index).unwrap();

        if letter == '\"' {
            in_quote = !in_quote;

            if !tmp_word.is_empty() {
                tokens.push(Token::new(Token::String(tmp_word)));
                tmp_word = String::new();
            }

            continue;
        }

        if in_quote {
            tmp_word.push(letter);
        } else if letter == ' ' || letter == ',' || index == line.len() - 1 {
            if index == line.len() - 1 {
                tmp_word.push(letter);
            }

            if tmp_word.is_empty() {
                continue;
            }

            let mut token: Token = tokenize(tmp_word.clone());

            if matches!(token, Token::Other(_)) {
                if !tokens.is_empty() && tokens.last().unwrap() == &Token::Program {
                    token = Token::new(Token::Identifier(tmp_word));
                } else if !tokens.is_empty()
                    && tokens.last().unwrap() == &Token::Assign("::".to_string())
                {
                    token = Token::new(Token::Variable(tmp_word.clone()));
                }
            }
            tokens.push(token);
            tmp_word = String::new();
        } else if letter == '!' {
            let mut comment: String = String::new();

            for index_rest in index..line.len() {
                let letter_rest: char = line.chars().nth(index_rest).unwrap();
                comment.push(letter_rest);
            }

            tokens.push(Token::new(Token::Comment(comment)));
            break;
        } else {
            tmp_word.push(letter);
        }
    }

    tokens
}

/// This function parses the file.
pub fn parser(file: File) -> Program {
    let tmp_lines: Vec<String> = split_line(file.clone());
    let mut lines: Vec<Vec<Token>> = Vec::new();

    for (index, line) in tmp_lines.iter().enumerate() {
        let parsed_line: Vec<Token> = parse_line(line.to_string(), index + 1);
        lines.push(parsed_line);
    }

    let name: String = lines[0][1].get_value();

    Program::new(name, lines, HashMap::new(), file.get_path().to_string())
}
