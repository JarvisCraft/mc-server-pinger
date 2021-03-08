# mc-server-pinger

[![License](https://img.shields.io/github/license/JarvisCraft/mc-server-pinger)](./LICENSE) [![Test](https://github.com/JarvisCraft/mc-server-pinger/actions/workflows/test.yml/badge.svg)](https://github.com/JarvisCraft/mc-server-pinger/actions/workflows/test.yml) [![Crates.io](https://img.shields.io/crates/v/mc-server-pinger)](https://crates.io/crates/mc-server-pinger) [![GitHub release (latest by date)](https://img.shields.io/github/v/release/JarvisCraft/mc-server-pinger)](https://github.com/JarvisCraft/mc-server-pinger/releases)

## Description

Command line utility for pinging Minecraft servers via Server List Ping protocol.

# Usage

Short syntax is `mc-server-pinger <hostname>`.

The following options can be specified:

| Full name | Short name | Type                                                         | Default value (if available) | Description                                                 |
| --------- | ---------- | ------------------------------------------------------------ | ---------------------------- | ----------------------------------------------------------- |
| `flavor`  | `f`        | One of `status_code`                                         | `status_code`                | Flavor of the output:                                       |
| `port`    | `p`        | Integer in range `[0; 65535]`                                | `25565`                      | Port of the server                                          |
| `timeout` | `t`        | `<value><units>` where `value` is an integer and `units` is one of `ns`, `us`, `ms`, `s` | `5s`                         | Timeout after which the request should be considered failed |