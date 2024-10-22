//! ANSI control sequences.
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

const CSI: &str = "\x1b[";

pub fn set_fg(color: u8) -> String {
    format!("{CSI}38;5;{color}m")
}

pub fn set_bg(color: u8) -> String {
    format!("{CSI}48;5;{color}m")
}

pub fn set_fg_rgb(rgb: (u8, u8, u8)) -> String {
    let (r, g, b) = rgb;
    format!("{CSI}38;2;{r};{g};{b}m")
}

pub fn set_bg_rgb(rgb: (u8, u8, u8)) -> String {
    let (r, g, b) = rgb;
    format!("{CSI}48;2;{r};{g};{b}m")
}

pub fn reset() -> String {
    format!("{CSI}m")
}
