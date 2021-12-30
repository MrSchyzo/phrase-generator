use std::str::FromStr;

use lazy_static::lazy_static;
use regex::{Captures, Match, Regex};

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

impl GrammarTokenReference {
    fn try_get_str_from_group<'r>(
        captures: &'r Captures<'r>,
        group: &str,
        original_string: &str,
    ) -> AppResult<&'r str> {
        Self::try_get_group(captures, group, original_string).map(|m| m.as_str())
    }

    fn try_get_i32_from_group(
        captures: &Captures,
        group: &str,
        original_string: &str,
    ) -> AppResult<i32> {
        let int = Self::try_get_str_from_group(captures, group, original_string)?;
        i32::from_str(int).map_err(|err| {
            AppError::for_number_parse_error(int.to_owned(), original_string.to_owned(), err)
        })
    }

    fn maybe_i32_from_group(
        captures: &Captures,
        group: &str,
        original_string: &str,
    ) -> AppResult<Option<i32>> {
        if let Some(int) = Self::maybe_group(captures, group) {
            let int = int.as_str();
            let result = i32::from_str(int).map_err(|err| {
                AppError::for_number_parse_error(int.to_owned(), original_string.to_owned(), err)
            })?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    fn try_capture(s: &str) -> AppResult<Captures> {
        PLACEHOLDER
            .captures(s)
            .ok_or_else(|| AppError::for_unrecognized_placeholder(s.to_owned()))
    }

    fn maybe_group<'r>(captures: &'r Captures<'r>, group: &str) -> Option<Match<'r>> {
        captures.name(group)
    }

    fn try_get_group<'r>(
        captures: &'r Captures<'r>,
        group: &str,
        original_string: &str,
    ) -> AppResult<Match<'r>> {
        captures.name(group).ok_or_else(|| {
            AppError::for_group_not_found(group.to_owned(), original_string.to_owned())
        })
    }
}

impl FromStr for GrammarTokenReference {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Self::try_capture(s)?;
        let g_dep_id = Self::maybe_i32_from_group(&captures, "g_dep_id", s)?;
        let s_dep_id = Self::maybe_i32_from_group(&captures, "s_dep_id", s)?;

        Ok(Self {
            id: Self::try_get_i32_from_group(&captures, "id", s)?,
            semantic_properties: PropagationProperties {
                can_propagate: Self::try_get_str_from_group(&captures, "s_prop", s)?.eq("T"),
                dependency: Dependency::from(
                    s_dep_id,
                    Self::try_get_str_from_group(&captures, "s_dep", s)?,
                )?,
            },
            grammar_properties: PropagationProperties {
                can_propagate: Self::try_get_str_from_group(&captures, "g_prop", s)?.eq("T"),
                dependency: Dependency::from(
                    g_dep_id,
                    Self::try_get_str_from_group(&captures, "g_dep", s)?,
                )?,
            },
            reference: Self::try_get_str_from_group(&captures, "reference", s)?.to_owned(),
        })
    }
}
