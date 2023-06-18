# EIT Robotic Avatar Rebirth

Robotic Avatar is a study to see if we can use the human mind as part of a robotic control system via VR.  
The goal is to give a human operator a natural control of a remote robot as if it was his own body.

## [Future vision](./vision.md)

## Main features and goals
* Use web technologies to allow peer to peer connection between robots and robot and user. We use web technologies because it works everywhere. (Not Implemented)
* Allow for remote installation of robots, as the robots may be somewhat remote. (Not Implemented)
* Create connections between sensors and VR equipment over the internet. (Not Implemented)

## Quick Start

1. To start everything then run `docker compose up` in the project root.
1. For the web ui open: `http://127.0.0.1:8080/`
1. Login with the pin: `00000`
1. There should be a virtual robot you can play with, ready to go. (Not Implemented)

For the other robots look into there respective readmes for install instructions.

## Development 

This project use dev containers and everything should just work without any more user config.

To start the server run `cargo run --bin server`. You should have this running as other component talks to the server.

For the ui run `trunk serve` and open http://127.0.0.1:8080/ in the browser.

For the virtual robot `cargo run --bin robot`

## System components

### [Server](./server/README.md)
Common point of contact between robot to robot and between robots and users. 

### [Client](./client/README.md)
Ui for monitoring and controlling robots. Also driver for any visuals needed. VR Headset over WebXR

### [Robot](.robot/README.md)
A base library for robots and virtual robot to help development of core functionality

### [Common](./common/README.md)
Small library that defines common data definitions sent between components

## Robots

### [Minion](./minion/readme.md) (Not implemented)
Small robot car for testing remote sight, hearing and speech.

### Rocky (Not implemented)
Robot arm for testing hand and arm movement with haptic feedback.

## License and contribution
This project is licensed under the MIT License se [LICENSE](./LICENSE) file.

This repo hosts an experiment and will be archived after the trails are concluded. So we are not looking for outside contributions. If you find this repo interesting feel free to fork.
