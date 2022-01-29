use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[cfg(test)]
#[path = "./unit_tests/mod.rs"]
mod tests;

use self::parsing::TokenReference;
use crate::app_core::errors::{DataError, ProductionError};
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

pub struct ProductionBranch {
    sequence: Vec<PlaceholderReference>,
}

impl ProductionBranch {
    pub fn ordered_placeholder_references(&self) -> AppResult<Vec<&PlaceholderReference>> {
        let lookup: HashMap<i32, &PlaceholderReference> = self.sequence.iter().try_fold(
            HashMap::new(),
            |mut look: HashMap<i32, &PlaceholderReference>, placeholder| {
                if let Some(hit) = look.insert(placeholder.id(), placeholder) {
                    Err(AppError::for_production_id_clash(hit.id()))
                } else {
                    Ok(look)
                }
            },
        )?;

        let order = self.topologically_sorted(
            &lookup
                .iter()
                .map(|(&i, &reference)| (i, reference.dependencies()))
                .collect::<HashMap<i32, Vec<i32>>>(),
        )?;

        Ok(order
            .into_iter()
            .map(|i| {
                *(lookup
                    .get(&i)
                    .unwrap_or_else(|| panic!("Unexpectedly unable to find node in lookup {}", i)))
            })
            .collect::<Vec<_>>())
    }

    #[allow(unused)]
    pub fn placeholder_appearance_order_in_production(&self) -> Vec<i32> {
        self.sequence
            .iter()
            .map(PlaceholderReference::id)
            .collect_vec()
    }

    fn topologically_sorted(&self, graph: &HashMap<i32, Vec<i32>>) -> AppResult<Vec<i32>> {
        let mut yet_to_finalize: HashSet<i32> =
            graph.iter().map(|(&i, _)| i).collect::<HashSet<_>>();
        let mut finalized: HashSet<i32> = HashSet::new();
        let mut order: Vec<i32> = Vec::new();

        while !yet_to_finalize.is_empty() {
            let walk_root = *yet_to_finalize.iter().next().unwrap();
            let mut walk_stack: Vec<i32> = Vec::new();
            let mut visited: HashSet<i32> = HashSet::new();
            let mut walk: Vec<i32> = Vec::new();

            if !finalized.contains(&walk_root) {
                walk_stack.push(walk_root);
            }

            while let Some(current) = walk_stack.pop() {
                let empty = Vec::new();

                if finalized.contains(&current) {
                    continue;
                }

                if let Some(&node) = graph
                    .get(&current)
                    .unwrap_or(&empty)
                    .iter()
                    .find(|&other| !finalized.contains(other))
                {
                    walk_stack.push(node)
                } else {
                    order.push(current);
                    finalized.insert(current);
                    yet_to_finalize.remove(&current);
                }

                walk.push(current);

                if visited.contains(&current) {
                    return Err(AppError::Data(DataError::Production(
                        ProductionError::CycleDetected(walk),
                    )));
                }
                visited.insert(current);
            }
        }

        Ok(order)
    }
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
    NonTerminalSymbol(TokenReference),
    WordSelector(TokenReference),
}

impl PlaceholderReference {
    fn from_nts(captures: Captures) -> AppResult<Self> {
        captures
            .parse_on_match("content", Ok)
            .and_then(TokenReference::from_str)
            .map(Self::NonTerminalSymbol)
    }
    fn from_word_selector(captures: Captures) -> AppResult<Self> {
        captures
            .parse_on_match("content", Ok)
            .and_then(TokenReference::from_str)
            .map(Self::WordSelector)
    }
    fn dependencies(&self) -> Vec<i32> {
        match self {
            PlaceholderReference::NonTerminalSymbol(reference)
            | PlaceholderReference::WordSelector(reference) => vec![
                reference.grammar_dependency_on_other(),
                reference.semantic_dependency_on_other(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        }
    }
    pub fn id(&self) -> i32 {
        match self {
            PlaceholderReference::NonTerminalSymbol(reference)
            | PlaceholderReference::WordSelector(reference) => reference.id(),
        }
    }
}

impl FromStr for PlaceholderReference {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        IS_NTS.try_capture(s).and_then(Self::from_nts).or_else(|_| {
            IS_PLACEHOLDER
                .try_capture(s)
                .and_then(Self::from_word_selector)
        })
    }
}
