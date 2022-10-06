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

use {
    crate::{clap_support::TimeoutDuration, output_flavor::OutputFlavor},
    clap::Parser,
    craftping::tokio::ping,
    tokio::{net::TcpStream, time::timeout as tokio_timeout},
};

mod clap_support;
mod output_flavor;

/// Options of the command line utility.
#[derive(Parser)]
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
    let CommandLineOptions {
        hostname,
        port,
        timeout,
        flavor,
    } = CommandLineOptions::parse();

    let hostname = hostname.as_str();

    match TcpStream::connect((hostname, port)).await {
        Ok(mut stream) => {
            match tokio_timeout(timeout.into(), ping(&mut stream, hostname, port)).await {
                Ok(Ok(response)) => flavor.handle_response(response),
                Ok(Err(error)) => flavor.handle_ping_error(error),
                Err(timeout) => flavor.handle_timeout(timeout),
            }
        }
        Err(io_error) => flavor.handle_io_error(io_error),
    }
}
