use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::app_core::{errors::AppError, AppResult};

#[cfg(test)]
#[path = "./unit_tests/parsing.rs"]
mod tests;

lazy_static! {
    static ref PLACEHOLDER_DEFINITION: Regex = Regex::new(r"^(?P<id>\d|[1-9]\d*):(?P<g_dep>(?:N|C)|(?:O|CO)[(](?P<g_dep_id>\d|[1-9]\d*)[)]):(?P<g_prop>T|F):(?P<s_dep>(?:N|C)|(?:O|CO)[(](?P<s_dep_id>\d|[1-9]\d*)[)]):(?P<s_prop>T|F):(?P<reference>\w+)$").unwrap();
}

#[derive(PartialEq, Debug)]
pub struct TokenReference {
    id: i32,
    semantic_properties: PropagationProperties,
    grammar_properties: PropagationProperties,
    reference: String,
}

impl TokenReference {
    pub fn new_trivial_reference(reference: String) -> Self {
        Self {
            id: 0i32,
            semantic_properties: PropagationProperties::empty(),
            grammar_properties: PropagationProperties::empty(),
            reference,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn grammar_dependency_on_other(&self) -> Option<i32> {
        self.grammar_properties
            .dependency_to_other()
            .filter(|other| other.ne(&self.id))
    }
    pub fn grammar_depends_on_context(&self) -> bool {
        self.grammar_properties.depends_on_context()
    }
    pub fn grammar_can_propagate(&self) -> bool {
        self.grammar_properties.can_propagate()
    }

    pub fn semantic_dependency_on_other(&self) -> Option<i32> {
        self.semantic_properties
            .dependency_to_other()
            .filter(|other| other.ne(&self.id))
    }
    pub fn semantic_depends_on_context(&self) -> bool {
        self.semantic_properties.depends_on_context()
    }
    pub fn semantic_can_propagate(&self) -> bool {
        self.semantic_properties.can_propagate()
    }
}

#[derive(PartialEq, Debug)]
pub struct PropagationProperties {
    can_propagate: bool,
    dependency: Dependency,
}

impl PropagationProperties {
    fn empty() -> Self {
        Self {
            can_propagate: false,
            dependency: Dependency::OnNothing,
        }
    }
    fn dependency_to_other(&self) -> Option<i32> {
        self.dependency.dependency_to_other()
    }
    fn depends_on_context(&self) -> bool {
        self.dependency.depends_on_context()
    }
    fn can_propagate(&self) -> bool {
        self.can_propagate
    }
}

#[allow(clippy::enum_variant_names)]
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
    fn dependency_to_other(&self) -> Option<i32> {
        match self {
            Dependency::OnNothing | Dependency::OnContext => None,
            Dependency::On(id) | Dependency::OnContextAnd(id) => Some(*id),
        }
    }
    fn depends_on_context(&self) -> bool {
        matches!(self, Dependency::OnContextAnd(_) | Dependency::OnContext)
    }
}

impl FromStr for TokenReference {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::utils::regex::{EnhancedCaptures, EnhancedRegex};

        let captures = PLACEHOLDER_DEFINITION.try_capture(s)?;

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
