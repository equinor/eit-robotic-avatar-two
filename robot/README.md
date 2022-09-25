# Robotic Avatar Robot

This is both a simple virtual robot to help with development and a library for the other robots to use.  

## Status

The main focus now is:
- Infrastucture as code.
- Data streams beween robots and robots and users.

### Setup sequence
1. Short runbook to install OS and boot script (Not Implemented)
2. Boot script will download the latests binary of robot and run it. (Not Implemented)
    * May revert the filesystem to a clean slate (Not Implemented)
3. This robot software will install and setup the system and requirements based on info from server. (Not Implemented)
4. Then start running the main loop for that robot. (Not Implemented)
5. Shut everyting down cleanly if given the signal from server. (Not Implemented)

### Other features.
* Process ad hoc console commands from server. (Not Implemented)
* Create peer to peer linkes togehter with server. (Not Implemented)

## Development

For now run `cargo run` this directory or `cargo run --bin robot` from project root. 

There are no tests yet.

## Production
No production notes yet.

## Configuration
No configuration yet.