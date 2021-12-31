use std::str::FromStr;

use crate::app_core::errors::{AppError, DataError, ParseError};

use super::PlaceholderReference;

#[test]
fn nts_recognized() {
    let str = "{1:N:T:C:F:prova}";
    let result = PlaceholderReference::from_str(str).unwrap();

    assert!(matches!(result, PlaceholderReference::NonTerminalSymbol(_)));
}

#[test]
fn word_selector_recognized() {
    let str = "<1:N:T:C:F:prova>";
    let result = PlaceholderReference::from_str(str).unwrap();

    assert!(matches!(result, PlaceholderReference::WordSelector(_)));
}

#[test]
fn mixed_delimiters_cannot_be_recognized() {
    let str = "<1:N:T:C:F:prova}";
    let result = PlaceholderReference::from_str(str);

    assert!(matches!(
        result,
        Err(AppError::Data(DataError::GrammarParseError(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ));
}

#[test]
fn wrong_delimiters_cannot_be_recognized() {
    let str = "(1:N:T:C:F:prova)";
    let result = PlaceholderReference::from_str(str);

    assert!(matches!(
        result,
        Err(AppError::Data(DataError::GrammarParseError(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ));
}

#[test]
fn wrong_content_cannot_be_recognized() {
    let str = "{1#N:T:CO(1000):F:pr!ova}";
    let result = PlaceholderReference::from_str(str);

    assert!(matches!(
        result,
        Err(AppError::Data(DataError::GrammarParseError(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ));
}
