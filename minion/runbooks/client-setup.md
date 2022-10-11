# Build and Install Client

The robot avatar client software have 3 parts:
1. Camera browser app. That runs on a pi that is connected to 
1. VR browser app.
1. Server to host the app and talk to the ROS side.

## Server setup

### Prerequisite
1. Assume you already have pi running ubuntu 20.04.

### Install requirements (Untested)
1. Install node 16.
    1. `curl -fsSL https://deb.nodesource.com/setup_16.x | sudo -E bash -`
    1. `sudo apt-get install -y nodejs`
1. Install yarn.
    1. `curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor | sudo tee /usr/share/keyrings/yarnkey.gpg >/dev/null`
    1. `echo "deb [signed-by=/usr/share/keyrings/yarnkey.gpg] https://dl.yarnpkg.com/debian stable main" | sudo tee /etc/apt/sources.list.d/yarn.list`
    1. `sudo apt-get update && sudo apt-get install yarn`
1. Install Rust. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    1. `source "$HOME/.cargo/env"`


### Get repo (untested)
1. clone repo: `git clone https://github.com/equinor/eit-robotic-avatar.git`
1. Install node js dependencies: `yarn install`
1. Build browser app: `yarn parcel build`
1. Build and run server: `cargo run --release` (This step will only take forever :P )

## Camera browser setup:
### Prerequisite
* You have the ip of server above.

### Setup raspberry pi 4.
1. Flash a new pi with "Raspberry PI OS (64-bit)"
1. Do the stander pi install. Make sure the camera pi is on the same network as the VR headset will be. And have access to the server.
1. Connect 2 Logitech HD webcams to the blue usb ports.

### First stream:
1. Connect the pi to a screen and keyboard and mouse.
1. Have the ip from the server ready.
1. Open chromium and load `https//ip:3000/`
1. Accept the unsecure certificate.
1. Press `Start as source`
1. Accept the use of cameras
1. Refresh page.
1. Noe you should see a list of cameras with IDS listed.
1. Copy paste the correct id into the `Left Camera ID` and  `Right Camera ID`
1. Press `Start as source` and verify that it works.
1. You can now disconnect screen, keyboard and mouse.
1. You can now connect the VR headset
 
* The `Start as source NO VIEWPORT` will save som cpu on the PI

## Connect Oculus Quest 2
### Prerequisite
* You have the ip of server above.
* The Oculus Quest must be on the same network as the Camera Pi.
* The Oculus Quest must have the Quest Bowser installed.

### Connect to robot.
1. Open the quest browser and go to the same `https//ip:3000/`
1. Press `Start as receiver`
1. When you see picture you can press `Enter VR`
1. It works we hope.

## Reconnect procedure
When you need to reconnect the VR headset

1. On PI: Connect the pi to a screen and keyboard and mouse.
1. On PI: Refresh page.
1. On PI: Press `Start as source` or `Start as source NO VIEWPORT`
1. On VR: Quit VR.
1. On VR: Refresh page.
1. On VR: Press `Start as receiver`
1. On VR: When you see picture you can press `Enter VR`
1. On PI: You can now disconnect screen, keyboard and mouse.



