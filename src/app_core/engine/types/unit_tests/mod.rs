use crate::app_core::engine::types::ProductionBranch;
use itertools::Itertools;
use std::str::FromStr;

use crate::app_core::errors::{AppError, DataError, ParseError, ProductionError};

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
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ));
}

#[test]
fn wrong_delimiters_cannot_be_recognized() {
    let str = "(1:N:T:C:F:prova)";
    let result = PlaceholderReference::from_str(str);

    assert!(matches!(
        result,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ));
}

#[test]
fn wrong_content_cannot_be_recognized() {
    let str = "{1#N:T:CO(1000):F:pr!ova}";
    let result = PlaceholderReference::from_str(str);

    assert!(matches!(
        result,
        Err(AppError::Data(DataError::GrammarParse(ParseError::RegexDidNotRecognize(s)))) if s.eq(str)
    ));
}

#[test]
fn parsed_production_branch_returns_topologically_sorted_placeholder_references() {
    let str = "{0:O(0):T:O(2):F:some} {1:CO(2):T:O(0):F:some} {2:CO(2):T:O(2):F:some} {3:CO(1):T:O(2):F:some} {4:CO(2):T:O(5):F:some} {5:CO(3):T:O(0):F:some}";
    let result = ProductionBranch::from_str(str)
        .unwrap()
        .ordered_placeholder_references()
        .unwrap()
        .into_iter()
        .map(PlaceholderReference::id)
        .collect_vec();

    assert_eq!(result, [2, 0, 1, 3, 5, 4]);
}

#[test]
fn parsed_production_branch_returns_detected_cycle_error() {
    let str = "{0:O(0):T:O(2):F:some} {1:CO(2):T:O(0):F:some} {2:CO(5):T:O(2):F:some} {3:CO(1):T:O(2):F:some} {4:CO(2):T:O(5):F:some} {5:CO(3):T:O(0):F:some}";
    let branch = ProductionBranch::from_str(str).unwrap();
    let result = branch.ordered_placeholder_references();

    assert!(matches!(
        result,
        Err(AppError::Data(DataError::Production(
            ProductionError::CycleDetected(_)
        )))
    ));
}

#[test]
fn parsed_production_branch_returns_detected_cycle_error_of_expected_length() {
    let str = "{0:O(0):T:O(1):F:some} {1:CO(2):T:O(1):F:some} {2:CO(3):T:O(2):F:some} {3:CO(4):T:O(4):F:some} {4:CO(4):T:O(5):F:some} {5:CO(0):T:O(5):F:some}";
    let branch = ProductionBranch::from_str(str).unwrap();

    match branch.ordered_placeholder_references() {
        Err(AppError::Data(DataError::Production(ProductionError::CycleDetected(cycle)))) => {
            assert_eq!(
                cycle.len(),
                (&[0, 1, 2, 3, 4, 5, 0]).len(),
                "Unit test correctly spotted, but the returned cycle has not the same length"
            )
        }

        _ => {
            panic!("Unit test was expecting an error, none was returned")
        }
    }
}
