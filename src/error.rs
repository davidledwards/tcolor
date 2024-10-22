//! Error types.
//!
//! Copyright 2024 David Edwards
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//! <https://www.apache.org/licenses/LICENSE-2.0>
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.mod ansi;

use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    UnexpectedArg(String),
    InvalidValue(String, String),
    ExpectedValue(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedArg(arg) => write!(f, "{arg}: unexpected argument"),
            Error::InvalidValue(arg, v) => write!(f, "{v}: invalid value following {arg}"),
            Error::ExpectedValue(arg) => write!(f, "{arg}: expecting value to follow"),
        }
    }
}
