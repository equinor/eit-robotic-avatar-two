# Minion the destroyer of pi-es.

Minimal readme for now. Just copied the code over from the old [eit-robotic-avatar](https://github.com/equinor/eit-robotic-avatar) repo.

## Setup
### [Client software](./runbooks/client-setup.md)
### [Robot](./runbooks/robot-setup.md)

## [Cad drawings](./cad/readme.md)

## Configuration
The robot binary require 2 environment variables: `ROBOT_SERVER_URL` and `ROBOT_TOKEN`
.env file is supported.

### ROBOT_SERVER_URL
The url to the base path of the server. That robot should connect to for configuration and communication.

Debug only default: http://127.0.0.1:3000/

### ROBOT_TOKEN
A bearer token to be used to authenticate with server.

Debug only default is a token signed by an empty key. Only debug version of server can accept it as valid.
