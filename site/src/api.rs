// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Each API endpoint has its own module. The modules contain Request and/or
//! Response structs; these contain the specifications for how to interact
//! with the API.
//!
//! The responses are calculated in the server.rs file.

use std::collections::BTreeSet;
use std::result::Result as StdResult;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "list", content = "content")]
pub enum List {
    All,
    List(BTreeSet<String>),
}

impl From<Vec<String>> for List {
    fn from(s: Vec<String>) -> List {
        List::List(s.into_iter().collect())
    }
}

impl List {
    pub fn contains(&self, item: &str) -> bool {
        match *self {
            List::All => true,
            List::List(ref x) => x.contains(item),
        }
    }

    pub fn into_set(&self, all: &BTreeSet<String>) -> BTreeSet<String> {
        match *self {
            List::All => all.clone(),
            List::List(ref x) => x.clone(),
        }
    }
}

pub type ServerResult<T> = StdResult<T, String>;

pub mod info {
    use date::Date;
    use std::collections::BTreeSet;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Response {
        /// Sorted vector of crate names
        pub crates: BTreeSet<String>,

        /// Sorted list of statistic names known
        pub stats: BTreeSet<String>,

        /// Chronologically last loaded run date.
        pub as_of: Date,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitResponse {
    pub commit: Option<String>,
}

pub mod data {
    use super::List;
    use server::DateData;
    use collector::Bound;
    use std::collections::BTreeSet;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,

        /// Which crates to return data for
        pub crates: List,

        /// Which statistic to return data for
        pub stat: String,
    }

    /// List of DateData's from oldest to newest
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub data: Vec<DateData>,
        pub crates: BTreeSet<String>,
    }
}

pub mod days {
    use super::List;
    use server::DateData;
    use collector::Bound;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,

        /// Which crates to return data for
        pub crates: List,
        pub stat: String,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub a: DateData,
        pub b: DateData,
    }
}
