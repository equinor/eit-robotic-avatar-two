# Robotic Avatar Web Client

The web clients main task is letting users manage robots that is part of Robotic Avatar.  
It should host the ui for any screens required for the robotic avatar, including support for VR Headsets and monitoring displays.

### Features:
* See the status of robots. (Not Implemented)
* Configure robots. (Not Implemented)
* Send ad hock console command to a robot. (Not Implemented)
* VR App (Not Implemented)

## Development

To run the dev server use `trunk serve` from the project root folder.

If you are not using the dev container then you need to install some dependencies:

* Web asembely build target with: `rustup target add wasm32-unknown-unknown`
* And install trunk using: `cargo install --locked trunk`

There are no tests yet.

## Production

There are no special production notes yet.

## Configuration
Being a web client it does not have a any configuration.

