# Robotic Avatar Web Client

The web clients main task is letting users manamge robots that is part of Robotic Avatar. And also host the ui for any screens needed by robotic avatar. VR Headsets and monetoring displays.

### Features:
* See the status of robots. (Not Implemented)
* Configure robots. (Not Implemented)
* Send ad hock console command to a robot. (Not Implemented)
* VR App (Not Implemented)

## Development

To run the dev server use `trunk serve` from the project root folder.

If you are not using the dev container you need to install some dependensies:

* Web asembely build target with: `rustup target add wasm32-unknown-unknown`
* And install trunk using: `cargo install --locked trunk`

There are no tests yet.

## Production

There are no spesial production notes yet.

## Configuration
Being a web client it do not have a any configuration.

