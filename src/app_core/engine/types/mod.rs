use std::str::FromStr;

#[cfg(test)]
#[path = "./unit_tests/mod.rs"]
mod tests;

use self::parsing::GrammarTokenReference;
use crate::{
    app_core::{errors::AppError, AppResult},
    utils::regex::{EnhancedCaptures, EnhancedRegex},
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub mod parsing;

lazy_static! {
    static ref IS_NTS: Regex = Regex::new(r"^[{](?P<content>[^{}]+)[}]$").unwrap();
    static ref IS_PLACEHOLDER: Regex = Regex::new(r"^[<](?P<content>[^<>]+)[>]$").unwrap();
}

#[allow(dead_code)]
pub struct ProductionBranch {
    sequence: Vec<PlaceholderReference>,
}

impl FromStr for ProductionBranch {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (successes, failures): (Vec<_>, Vec<_>) = s
            .split(' ')
            .into_iter()
            .map(|str| str.trim())
            .map(PlaceholderReference::from_str)
            .into_iter()
            .partition(Result::is_ok);

        if failures.is_empty() {
            Ok(Self {
                sequence: successes.into_iter().flat_map(Result::ok).collect(),
            })
        } else {
            Err(AppError::for_multiple_errors(
                failures.into_iter().flat_map(Result::Err).collect(),
            ))
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PlaceholderReference {
    NonTerminalSymbol(GrammarTokenReference),
    WordSelector(GrammarTokenReference),
}

impl PlaceholderReference {
    fn into_nts(captures: Captures) -> AppResult<Self> {
        captures
            .parse_on_match("content", Ok)
            .and_then(GrammarTokenReference::from_str)
            .map(Self::NonTerminalSymbol)
    }
    fn into_word_selector(captures: Captures) -> AppResult<Self> {
        captures
            .parse_on_match("content", Ok)
            .and_then(GrammarTokenReference::from_str)
            .map(Self::WordSelector)
    }
}

impl FromStr for PlaceholderReference {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IS_NTS.try_capture(s).and_then(Self::into_nts).or_else(|_| {
            IS_PLACEHOLDER
                .try_capture(s)
                .and_then(Self::into_word_selector)
        })
    }
}
