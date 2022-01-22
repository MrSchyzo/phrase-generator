use std::str::FromStr;

use crate::app_core::errors::AppError;
use crate::app_core::errors::DataError;
use crate::app_core::errors::ParseError;

use super::Dependency;
use super::PropagationProperties;
use super::TokenReference;

#[test]
fn grammar_token_reference_trivial_case() {
    let str = "0:N:T:C:F:Nome";
    let grammar_token_reference = TokenReference::from_str(str).unwrap();

    assert_eq!(
        grammar_token_reference,
        TokenReference {
            id: 0i32,
            semantic_properties: PropagationProperties {
                can_propagate: false,
                dependency: Dependency::OnContext,
            },
            grammar_properties: PropagationProperties {
                can_propagate: true,
                dependency: Dependency::OnNothing,
            },
            reference: "Nome".to_owned(),
        }
    );
}

#[test]
fn grammar_token_reference_other_trivial_case() {
    let str = "120:C:T:C:T:regex";
    let grammar_token_reference = TokenReference::from_str(str).unwrap();

    assert_eq!(
        grammar_token_reference,
        TokenReference {
            id: 120i32,
            semantic_properties: PropagationProperties {
                can_propagate: true,
                dependency: Dependency::OnContext,
            },
            grammar_properties: PropagationProperties {
                can_propagate: true,
                dependency: Dependency::OnContext,
            },
            reference: "regex".to_owned(),
        }
    );
}

#[test]
fn grammar_token_reference_non_trivial_grammar_dependency_definition() {
    let str = "110:O(200):F:N:T:non_trivial";
    let grammar_token_reference = TokenReference::from_str(str).unwrap();

    assert_eq!(
        grammar_token_reference,
        TokenReference {
            id: 110i32,
            semantic_properties: PropagationProperties {
                can_propagate: true,
                dependency: Dependency::OnNothing,
            },
            grammar_properties: PropagationProperties {
                can_propagate: false,
                dependency: Dependency::On(200i32),
            },
            reference: "non_trivial".to_owned(),
        }
    );
}

#[test]
fn grammar_token_reference_non_trivial_semantic_dependency_definition() {
    let str = "65:C:T:CO(120):F:anotherOne";
    let grammar_token_reference = TokenReference::from_str(str).unwrap();

    assert_eq!(
        grammar_token_reference,
        TokenReference {
            id: 65i32,
            semantic_properties: PropagationProperties {
                can_propagate: false,
                dependency: Dependency::OnContextAnd(120i32),
            },
            grammar_properties: PropagationProperties {
                can_propagate: true,
                dependency: Dependency::OnContext,
            },
            reference: "anotherOne".to_owned(),
        }
    );
}

#[test]
fn grammar_token_reference_non_trivial_case() {
    let str = "12:CO(1):T:O(2):F:lastOne11";
    let grammar_token_reference = TokenReference::from_str(str).unwrap();

    assert_eq!(
        grammar_token_reference,
        TokenReference {
            id: 12i32,
            semantic_properties: PropagationProperties {
                can_propagate: false,
                dependency: Dependency::On(2i32),
            },
            grammar_properties: PropagationProperties {
                can_propagate: true,
                dependency: Dependency::OnContextAnd(1i32),
            },
            reference: "lastOne11".to_owned(),
        }
    );
}

#[test]
fn grammar_token_reference_with_negative_number_cannot_be_recognized() {
    let str = "-12:CO(1):T:O(2):F:lastOne11";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_with_spaces_cannot_be_recognized() {
    let str = "12 : CO ( 1 ) : T : O ( 2 ) : F : lastOne11";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_with_wrong_order_cannot_be_recognized() {
    let str = "12:T:N:T:N:lastOne";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_with_wrong_dependency_definition_cannot_be_recognized() {
    let str = "12:N(10):T:N:T:lastOne";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_with_wrong_separators_cannot_be_recognized() {
    let str = "12#N#T#N#T#lastOne";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_with_non_alphanumerical_name_cannot_be_recognized() {
    let str = "12:O(10):T:N:F:last-One";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_with_empty_dependency_identifier_cannot_be_recognized() {
    let str = "12:O():T:N:F:lastOne";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_with_colons_only_cannot_be_recognized() {
    let str = ":::::";
    let grammar_token_reference = TokenReference::from_str(str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ))
}

#[test]
fn grammar_token_reference_cannot_be_provided_with_numbers_that_overflow_32_bits() {
    let gigantic = "999999999999";
    let str = format!("12:O({}):T:N:F:lastOne", gigantic);
    let grammar_token_reference = TokenReference::from_str(&str);
    assert!(matches!(
        grammar_token_reference,
        Err(AppError::Data(DataError::GrammarParse(ParseError::CannotParseToNumber(number, error))))
        if number.eq(gigantic) && error.kind().eq(&std::num::IntErrorKind::PosOverflow)
    ))
}
