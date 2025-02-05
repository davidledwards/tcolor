//! Displays ANSI colors on the terminal.
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

mod ansi;
mod error;
mod opt;

use crate::error::Result;
use crate::opt::Options;
use std::process::ExitCode;

/// Usage documentation for display to terminal.
const USAGE: &str = include_str!("include/usage.in");

// Version and build information.
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_HASH: &str = env!("BUILD_HASH");
const BUILD_DATE: &str = env!("BUILD_DATE");

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            println!("{e}");
            println!("use --help for options");
            ExitCode::from(1)
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

fn run() -> Result<()> {
    let opts = Options::parse(std::env::args().skip(1))?;
    if opts.help {
        println!("{USAGE}");
    } else if opts.version {
        println!("{PACKAGE_NAME} {PACKAGE_VERSION} ({BUILD_HASH} {BUILD_DATE})");
    } else {
        run_opts(&opts)
    }
    Ok(())
}

fn run_opts(opts: &Options) {
    const BLANKS: &str = "                               ";

    if let Some(rgb) = opts.rgb {
        let (r, g, b) = rgb;
        println!(
            "rgb({r},{g},{b}) {}{BLANKS}{}",
            ansi::set_bg_rgb(rgb),
            ansi::reset(),
        );
    } else if let Some(blend) = opts.blend {
        let (fg, bg) = blend;
        println!(
            "{}  foreground: {fg}, background: {bg}  {}",
            ansi::set_color(fg, bg),
            ansi::reset()
        );
    } else {
        let colors = if let Some(color) = opts.color {
            color..=color
        } else if opts.standard {
            0..=15
        } else {
            0..=255
        };
        for color in colors {
            println!(
                "{color:<3} {}{BLANKS}{}",
                ansi::set_bg(color),
                ansi::reset()
            );
        }
    }
}
