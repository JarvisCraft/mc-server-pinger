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

use clap::Clap;
use craftping::{Error, Response};
use craftping::tokio::ping;
use tokio::time::error::Elapsed;
use tokio::time::timeout;

use crate::duration::TimeoutDuration;
use crate::output_flavor::OutputFlavor;

mod duration;
mod output_flavor;

/// Options of the command line utility.
#[derive(Clap)]
struct CommandLineOptions {
    /// Hostname of the server
    hostname: String,

    /// Port of the server
    #[clap(short, long, default_value = "25565")]
    port: u16,

    /// Timeout after which the request should be considered failed
    #[clap(short, long, default_value = "5s")]
    timeout: TimeoutDuration,

    /// Flavor of the output
    #[clap(short, long, default_value = "status_code")]
    flavor: OutputFlavor,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let options: CommandLineOptions = CommandLineOptions::parse();
    match timeout(
        *options.timeout,
        ping(options.hostname.as_str(), options.port),
    )
        .await
    {
        Ok(Ok(response)) => options.flavor.handle_response(response),
        Ok(Err(error)) => options.flavor.handle_error(error),
        Err(timeout) => options.flavor.handle_timeout(timeout),
    };
}
