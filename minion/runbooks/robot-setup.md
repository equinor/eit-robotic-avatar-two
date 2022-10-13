# Minion Robot Setup

Minion uses a [LattePanda 3 Delta](https://www.lattepanda.com/lattepanda-3-delta).  Raspberry Pi was not powerful enough for 2 video streams.

## Setup LattePanda 3 Delta
1. Download and flash Ubuntu 20.04 (64-bit) to a USB Stick
1. Install Ubuntu 20.04 (64-bit) to LattePanda
    * Both desktop and server will work. I used a minimal desktop install.
1. Make sure its online and the normal peripherals like keyboard mouse and screen works.
    * The robot needs Wifi working before it can drive around.
1. Update packages with `sudo apt update` and `sudo apt upgrade`

## Setup SSH
1. Install ssh server with `sudo apt install openssh-server`
    * Now you can connect over ssh for the rest of the guide.

## Download the robotic avatar project
1. Install git with `sudo apt install git`
1. Clone the repo using one of:
    * Using http: `git clone https://github.com/equinor/eit-robotic-avatar-two.git`
    * Using ssh:`git clone git@github.com:equinor/eit-robotic-avatar-two.git`

## Install Rust
1. Install curl using: `sudo apt install curl`
1. Install compiler using: `sudo apt install build-essential`
1. Download and run the Rust install script using: `curl https://sh.rustup.rs -sSf | sh`
    * Configure current shell with `source "$HOME/.cargo/env"`
1. This project have some dependencies to build:
    * libssl-dev: `sudo apt install libssl-dev`
    * python3-dev: `sudo apt install python3-dev`
1. Run `cargo build` to see if everything is in order.

## Install Rust web dev (Dev only)
You only need this if you plan todo development locally. 
1. Install web assembly target: `rustup target add wasm32-unknown-unknown`
1. Setup the trunk builder: `cargo install --locked trunk`
1. `trunk serve` to build the client locally.

## Install ROS
Based on: https://emanual.robotis.com/docs/en/platform/openmanipulator_x/quick_start_guide/
1. Download ROS install script: `wget https://raw.githubusercontent.com/ROBOTIS-GIT/robotis_tools/master/install_ros2_foxy.sh`
1. Set execution bits: `chmod 755 ./install_ros2_foxy.sh`
1. Run the script `bash ./install_ros2_foxy.sh` and pray.

## Setup OpenMANIPULATOR-X
1. Install ROS dependencies:`sudo apt install ros-foxy-rqt* ros-foxy-joint-state-publisher` 
1. Download OpenMANIPULATOR-X source:
    1. `cd ~/colcon_ws/src/` 
    1. `git clone -b foxy-devel https://github.com/ROBOTIS-GIT/DynamixelSDK.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/dynamixel-workbench.git`  
    1. `git clone -b foxy-devel https://github.com/ROBOTIS-GIT/open_manipulator.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/open_manipulator_msgs.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/open_manipulator_dependencies.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/robotis_manipulator.git`
1. Build and install the arm software with:
    1. `cd ~/colcon_ws`
    1. `colcon build --symlink-install`
1. You need to open a new terminal. Don't ask.
1. Set USB latency to 1 ms using: `ros2 run open_manipulator_x_controller create_udev_rules`
1. You can test the arm with `ros2 launch open_manipulator_x_controller open_manipulator_x_controller.launch.py`
    * The robot will be rigid while the script is running.

# Old Setup

## Setup robot specific settings
1. `sudo chown root:$USER /dev/gpiomem`
1. `sudo chmod g+rw /dev/gpiomem`

## Setup networking fallback
Based on (https://ubuntu.com/server/docs/network-configuration)  

1. `cd /etc/netplan/`  
1. `sudo nano 99_config.yaml` with:
``` yaml
network:  
  version: 2  
  renderer: networkd  
  ethernets:  
    eth0:  
      addresses:  
        - 192.168.0.1/24
``` 
1. After making the changes, apply them in terminal: `sudo netplan apply`

## Setup Client
1. Run the [Client install runbook](./client-setup.md)

## Start robot.
1. Move SD card to robot and boot up
1. SSH into the robot over Wi-Fi
1. Turn on VR headseat and enter VR
1. In one terminal write: `ros2 run my_package my_client`
1. In anoher terminal write: `ros2 launch open_manipulator_x_controller open_manipulator_x_controller.launch.py`
1. The robot should now be fully funcional. 
