use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::app_core::{errors::AppError, AppResult};

#[cfg(test)]
#[path = "./unit_tests/grammar.rs"]
mod test;

lazy_static! {
    static ref PLACEHOLDER: Regex = Regex::new(r"^(?P<id>\d|[1-9]\d*):(?P<g_dep>(?:N|C)|(?:O|CO)[(](?P<g_dep_id>\d|[1-9]\d*)[)]):(?P<g_prop>T|F):(?P<s_dep>(?:N|C)|(?:O|CO)[(](?P<s_dep_id>\d|[1-9]\d*)[)]):(?P<s_prop>T|F):(?P<reference>\w+)$").unwrap();
}

#[derive(PartialEq, Debug)]
pub struct GrammarTokenReference {
    pub id: i32,
    pub semantic_properties: PropagationProperties,
    pub grammar_properties: PropagationProperties,
    pub reference: String,
}

#[derive(PartialEq, Debug)]
pub struct PropagationProperties {
    pub can_propagate: bool,
    pub dependency: Dependency,
}

#[derive(PartialEq, Debug)]
pub enum Dependency {
    OnNothing,
    OnContext,
    On(i32),
    OnContextAnd(i32),
}

impl Dependency {
    fn from(maybe_id: Option<i32>, entire: &str) -> AppResult<Self> {
        match (maybe_id, entire) {
            (None, "N") => Ok(Self::OnNothing),
            (None, "C") => Ok(Self::OnContext),
            (Some(i), s) if s.starts_with("O(") => Ok(Self::On(i)),
            (Some(i), s) if s.starts_with("CO(") => Ok(Self::OnContextAnd(i)),
            _ => Err(AppError::for_unrecognized_dependency_marker(
                entire.to_owned(),
            )),
        }
    }
}

impl FromStr for GrammarTokenReference {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::utils::regex::{EnhancedCaptures, EnhancedRegex};

        let captures = PLACEHOLDER.try_capture(s)?;

        Ok(Self {
            id: captures.i32_from_group("id")?,
            semantic_properties: PropagationProperties {
                can_propagate: captures.parse_on_match("s_prop", Ok)?.eq("T"),
                dependency: Dependency::from(
                    captures.maybe_i32_from_group("s_dep_id")?,
                    captures.parse_on_match("s_dep", Ok)?,
                )?,
            },
            grammar_properties: PropagationProperties {
                can_propagate: captures.parse_on_match("g_prop", Ok)?.eq("T"),
                dependency: Dependency::from(
                    captures.maybe_i32_from_group("g_dep_id")?,
                    captures.parse_on_match("g_dep", Ok)?,
                )?,
            },
            reference: captures.parse_on_match("reference", Ok)?.to_owned(),
        })
    }
}
