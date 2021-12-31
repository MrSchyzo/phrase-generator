use regex::{Captures, Match, Regex};

use crate::app_core::{errors::AppError, AppResult};

pub trait EnhancedRegex {
    fn try_capture<'r>(&self, s: &'r str) -> AppResult<Captures<'r>>;
}

impl EnhancedRegex for Regex {
    fn try_capture<'r>(&self, s: &'r str) -> AppResult<Captures<'r>> {
        self.captures(s)
            .ok_or_else(|| AppError::for_regex_did_not_recognize(s.to_owned()))
    }
}

pub trait EnhancedCaptures<'r> {
    fn entire_match(&self) -> &str;

    fn named_group(&self, group: &str) -> AppResult<Match>;

    fn maybe_i32_from_group(&self, group: &str) -> AppResult<Option<i32>>;

    fn i32_from_group(&self, group: &str) -> AppResult<i32>;

    fn parse_on_match<OUT: 'r, F: FnOnce(&'r str) -> AppResult<OUT>>(
        &'r self,
        group: &'r str,
        parser: F,
    ) -> AppResult<OUT>;
}

impl<'r> EnhancedCaptures<'r> for Captures<'r> {
    fn entire_match(&self) -> &str {
        self.get(0)
            .expect("Capture without group number 0!")
            .as_str()
    }

    fn named_group(&self, group: &str) -> AppResult<Match> {
        self.name(group).ok_or_else(|| {
            AppError::for_group_not_found(group.to_owned(), self.entire_match().to_owned())
        })
    }

    fn maybe_i32_from_group(&self, group: &str) -> AppResult<Option<i32>> {
        match self.name(group) {
            Some(int_match) => int_match.parsed_as_i32().map(Some),
            None => Ok(None),
        }
    }

    fn parse_on_match<OUT: 'r, F: FnOnce(&'r str) -> AppResult<OUT>>(
        &'r self,
        group: &'r str,
        parser: F,
    ) -> AppResult<OUT> {
        self.named_group(group)?.parsed_with(parser)
    }

    fn i32_from_group(&self, group: &str) -> AppResult<i32> {
        self.named_group(group).and_then(|m| m.parsed_as_i32())
    }
}

/*
  I have no idea what I am doing with lifetime annotations...
*/
pub trait EnhancedMatch<'out, 'input: 'out> {
    fn parsed_with<OUT: 'out, F: FnOnce(&'input str) -> AppResult<OUT>>(
        &self,
        parser: F,
    ) -> AppResult<OUT>;

    fn parsed_as_i32(&self) -> AppResult<i32>;
}

impl<'r: 'input, 'input: 'out, 'out> EnhancedMatch<'out, 'input> for Match<'r> {
    fn parsed_with<OUT: 'out, F: FnOnce(&'input str) -> AppResult<OUT>>(
        &self,
        parser: F,
    ) -> AppResult<OUT> {
        parser(self.as_str())
    }

    fn parsed_as_i32(&self) -> AppResult<i32> {
        use std::str::FromStr;

        self.parsed_with(|int| {
            i32::from_str(int).map_err(|err| AppError::for_number_parse_error(int.to_owned(), err))
        })
    }
}
