# mc-server-pinger

## Description

Command line utility for pinging Minecraft servers via Server List Ping protocol.

# Usage

Short syntax is `mc-server-pinger <hostname>`.

The following options can be specified:

| Full name | Short name | Type                                                         | Default value (if available) | Description                                                 |
| --------- | ---------- | ------------------------------------------------------------ | ---------------------------- | ----------------------------------------------------------- |
| `flavor`  | `f`        | One of `statu`                                               | `status_code`                | Flavor of the output:                                       |
| `port`    | `p`        | Integer in range `[0; 65535]`                                | `25565`                      | Port of the server                                          |
| `timeout` | `t`        | `<value><units>` where `value` is an integer and `units` is one of `ns`, `us`, `ms`, `s` | `5s`                         | Timeout after which the request should be considered failed |