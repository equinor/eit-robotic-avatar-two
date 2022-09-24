# Robotic Avatar Server

The servers main job is to be a common point of contact between robots and robot and users.

### Features for robots:
* Config store. (Not Implemented)
* Target to send logs. (Not Implemented)
* Assists with self update (Not Implemented)
* Facilitate the creations of pear to pear links. WebRTC (Not Implemented)

### Features for users:
* See the status of robots. (Not Implemented)
* Configure robots. (Not Implemented)
* Send ad hock console command to a robot. (Not Implemented)
* Host user facing applications. (Not Implemented)

## Development

Run `cargo run` this directory or `cargo run --bin server` from project root.

There are no tests yet.

## Production

You may only deploy this server behind a HTTPS proxy.
You should not use .env files in production. 

## Configuration
The server is configured using environment variables and/or .env files.
With hardcoded default to make development easier.

Some settings are debug only and will only work on debug builds. They are often very unsafe settings that makes local development easier.

### AVATAR_BIND_ADDRESS
Define both the IPv4 address and port in the format `<address>:<port>`. That the server will bind for http traffic.

* Type: String
* Default value: 127.0.0.1:3000
* Debug only: false

### RUST_LOG
Logging configuration. For a simple case just put in the log level. `error`, `warn`, `info`, `debug` or `trace`. 

For more complicated configuration look at: https://docs.rs/env_logger/0.9.1/env_logger/#enabling-logging

* Type: Comma-separated list of logging directives.
* Default value: error
* Debug only: false

