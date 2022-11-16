# Robotic Avatar Server

The main job of the server is to be common point of contact between robot to robot and between robots and users

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
* Required: No

### AVATAR_TOKEN_KEY (SECRET)
A string to derive the singing key for bearer tokens. Should contain at least 256 bits of entropy. Must be at least 32 bytes longs.

* Type: String
* Default value: "" (Only debug have empty string as default.)
* Required: Yes for release.

### RUST_LOG
Logging configuration. For a simple case just put in the log level. `error`, `warn`, `info`, `debug` or `trace`. 

For more complicated configuration look at: https://docs.rs/env_logger/0.9.1/env_logger/#enabling-logging

Note: In dev container RUST_LOG is set to debug.

* Type: Comma-separated list of logging directives.
* Default value: error
* Required: No

### AVATAR_AZURE_AD_ID
The Client ID for a Azure AD App. It it not set it will disable Azure AD authentication. 

* Type: String
* Default value: 
* Required: No

### AVATAR_AZURE_AD_SECRET (SECRET)
The Client Secret for a Azure AD App. 

* Type: String
* Default value: 
* Required: Yes if AVATAR_AZURE_AD_ID is set.

### AVATAR_AZURE_AD_URL
OpenID Connect base url. Will look something like this: `https://login.microsoftonline.com/<Azure AD tenant ID>/v2.0`

* Type: Url
* Default value: 
* Required: Yes if AVATAR_AZURE_AD_ID is set.

### AVATAR_AZURE_AD_BASE_URL
The public url this application will have.

Will try to build a value from environment if not set.
1. `https://RADIX_PUBLIC_DOMAIN_NAME/`
2. `http://localhost:8080/` Most likely wrong

* Type: Url
* Default value: Auto built see notes.
* Required: No

### AVATAR_ICE
A comma delimitated list of urls to used and STUN and TURN server for WebRTC.

You can set username and passwords inside the url and they will be used for authentication with the TURN server.

* Type: Urls
* Default value: 
* Required: No
