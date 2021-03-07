// Copyright 2021 Petr Portnov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::process::exit;
use std::str::FromStr;

use craftping::{Error, Response};
use tokio::time::error::Elapsed;

/// Flavor of the output.
#[derive(Eq, PartialEq, Debug)]
pub enum OutputFlavor {
    /// Terminates with zero code if ping is successful and non-zero code otherwise.
    StatusCode,
}

impl OutputFlavor {
    pub fn handle_response(&self, response: Response) {
        match self {
            Self::StatusCode => exit(0),
        }
    }

    pub fn handle_error(&self, response: Error) {
        match self {
            Self::StatusCode => exit(1),
        }
    }

    pub fn handle_timeout(&self, response: Elapsed) {
        match self {
            Self::StatusCode => exit(1),
        }
    }
}

impl FromStr for OutputFlavor {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        // note: no need to convert non-ASCII characters as they won't get matched anyway
        match raw.to_ascii_lowercase().as_str() {
            "status_code" | "code" | "c" => Ok(Self::StatusCode),
            _ => Err(format!("unknown flavor identifier \"{}\"", raw)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //<editor-fold desc="Test implementation generator" defaultstate="collapsed">
    macro_rules! impl_test {
        ($name:ident: $raw:expr => !) => {
            #[test]
            fn $name() {
                assert!(OutputFlavor::from_str($raw).is_err());
            }
        };
        ($name:ident: $raw:expr => $parsed:ident) => {
            #[test]
            fn $name() {
                assert_eq!(OutputFlavor::from_str($raw), Ok(OutputFlavor::$parsed));
            }
        };
    }
    //</editor-fold>

    // Correct data tests
    impl_test!(can_parse_statuc_code: "status_code" => StatusCode);

    // Incorrect data tests
    impl_test!(cannot_parse_with_spaced: "status code" => !);
    impl_test!(cannot_parse_random_string: "hello_world" => !);
}
