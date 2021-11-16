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

//! Type wrappers enabling [`clap`] support.

use std::{fmt::Debug, str::FromStr, time::Duration};

/// Wrapper around [`Duration`] which supports [parsing](std::str::FromStr).
#[derive(Eq, PartialEq, Debug)]
pub struct TimeoutDuration(Duration);

/// Units in which the duration can be measured. These correspond to [`Duration`]'s `from_` methods.
enum DurationUnits {
    /// Nanoseconds
    Nanos,
    /// Microseconds
    Micros,
    /// Milliseconds
    Millis,
    /// Seconds
    Seconds,
}

impl DurationUnits {
    /// Creates a [`Duration`] with from the given amount of this unit.
    ///
    /// # Arguments
    /// * `value` - amount of this unit in the created duration
    fn to_duration(&self, value: u64) -> Duration {
        match self {
            Self::Nanos => Duration::from_nanos(value),
            Self::Micros => Duration::from_micros(value),
            Self::Millis => Duration::from_millis(value),
            Self::Seconds => Duration::from_secs(value),
        }
    }
}

peg::parser! {
    /// Parser of [`Duration`] from [`str`].
    grammar duration_parser() for str {

        rule value() -> u64 = quiet! {
            value:$(['0'..='9']+) {?
                value.parse().map_err(|_| "duration's value is too big")
            }
        } / expected!("duration's value is too big")

        rule units() -> DurationUnits = quiet! {
            "ns" { DurationUnits::Nanos } / "us" { DurationUnits::Micros }
            / "ms" { DurationUnits::Millis } / "s" { DurationUnits::Seconds }
        } / expected!("duration's units should be one of \"ms\", \"ns\", \"s\", \"us\"")

        /// Parses [`Duration`] from the given [`str`].
        pub rule duration() -> Duration = value:value() units:units() { units.to_duration(value) }
    }
}

impl FromStr for TimeoutDuration {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        duration_parser::duration(raw)
            .map(Self)
            .map_err(|error| error.expected.to_string())
    }
}

impl From<TimeoutDuration> for Duration {
    fn from(duration: TimeoutDuration) -> Self {
        duration.0
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
                assert!(TimeoutDuration::from_str($raw).is_err());
            }
        };
        ($name:ident: $raw:expr => $parsed:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    TimeoutDuration::from_str($raw),
                    Ok(TimeoutDuration($parsed))
                );
            }
        };
    }
    //</editor-fold>

    // Correct data tests
    impl_test!(can_parse_100_milliseconds: "100ms" => Duration::from_millis(100));
    impl_test!(can_parse_1212_microseconds: "1212us" => Duration::from_micros(1212));
    impl_test!(can_parse_8800_nanoseconds: "8800ns" => Duration::from_nanos(8800));
    impl_test!(can_parse_12152_seconds: "12152s" => Duration::from_secs(12152));
    impl_test!(can_parse_013_seconds: "013s" => Duration::from_secs(13));

    // Incorrect data tests
    impl_test!(cannot_parse_with_incorrect_units: "12y" => !);
    impl_test!(cannot_parse_with_incorrect_units_looking_like_corrcet: "14mm" => !);
    impl_test!(cannot_parse_with_minus: "-123s" => !);
    impl_test!(cannot_parse_with_space: "456 s" => !);
    impl_test!(cannot_parse_with_huge_number: "19929192192919219291928 s" => !);
}
