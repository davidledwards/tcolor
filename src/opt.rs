//! Options parser.
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

use crate::error::{Error, Result};

pub struct Options {
    pub help: bool,
    pub version: bool,
    pub standard: bool,
    pub extended: bool,
    pub color: Option<u8>,
    pub rgb: Option<(u8, u8, u8)>,
    pub blend: Option<(u8, u8)>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            help: false,
            version: false,
            standard: false,
            extended: true,
            color: None,
            rgb: None,
            blend: None,
        }
    }
}

impl Options {
    pub fn parse<T>(args: T) -> Result<Options>
    where
        T: IntoIterator<Item = String>,
    {
        let mut opts = Options::default();
        let mut it = args.into_iter();
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--standard" | "-s" => opts.standard = true,
                "--extended" | "-e" => opts.extended = true,
                "--color" | "-c" => opts.color = Some(parse_color(&arg, it.next())?),
                "--rgb" | "-r" => {
                    opts.rgb = Some((
                        parse_color(&arg, it.next())?,
                        parse_color(&arg, it.next())?,
                        parse_color(&arg, it.next())?,
                    ))
                }
                "--blend" | "-b" => {
                    opts.blend =
                        Some((parse_color(&arg, it.next())?, parse_color(&arg, it.next())?))
                }
                "--help" | "-h" => opts.help = true,
                "--version" | "-v" => opts.version = true,
                _ => return Err(Error::UnexpectedArg(arg)),
            };
        }
        Ok(opts)
    }
}

fn parse_color(arg: &str, next_arg: Option<String>) -> Result<u8> {
    next_arg
        .ok_or_else(|| Error::ExpectedValue(arg.to_string()))
        .and_then(|v| {
            v.parse::<u8>()
                .map_err(|_| Error::InvalidValue(arg.to_string(), v.to_string()))
        })
}
