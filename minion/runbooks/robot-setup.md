# Robot Setup

## Setup Raspberry Pi
1. Flash new SD card with Ubuntu Server 20.04 (64-bit)
1. Connect the SD card to an external PI and connect to internet via an Ethernet cable
1. Default username and password is: ubuntu and ubuntu
1. Find IP address with: `ip a`
1. Connect to over ssh using: `ssh ubuntu@<eth0 ip>`
1. Update date and time with:
    1. `sudo apt install ntpdate`
    1. `sudo ntpdate time.nist.gov`
1. Ubuntu server updates automatically
    * You will get an error `Waiting for cache lock: Could not get lock /var/lib/dpkg/lock-frontend. It is held by process 2794 (unattended-upgr)` while running apt commands. Wait it out.
    * Or you can kill it with `sudo pkill unattended-upgr` and do a normal `sudo apt update` and `sudo apt upgrade`
1. Connect PI to WIFI [Ubuntu Guide](https://ubuntu.com/core/docs/networkmanager/configure-wifi-connections)
    1. Install network manager with: `sudo apt install network-manager`
    1. Connect using: `sudo nmcli d wifi connect <wifi name> password <password>`
1. Move ssh session over to wifi and disconnect ethernet
    1. Find IP address with: `ip a`
    1. Exit ssh using: `exit`
    1. Connect to over ssh using: `ssh ubuntu@<wlan0 ip>`
        * You may need to remove the old known host

## Clone this project.
1. Clone repo using `https://github.com/equinor/eit-robotic-avatar.git`

## Install and Setup ROS
Based on: https://emanual.robotis.com/docs/en/platform/openmanipulator_x/quick_start_guide/
1. Run ROS 2 Foxy Install Script: 
    1. Install compiler using: `sudo apt install build-essential`
    1. `chmod 755 ~/eit-robotic-avatar/ros/install_ros2_foxy.sh`  
    1. `bash ~/eit-robotic-avatar/ros/install_ros2_foxy.sh`
1. Install dependent packages for OpenMANIPULATOR-X:  
    1. `sudo apt install ros-foxy-rqt* ros-foxy-joint-state-publisher`  
    1. `cd ~/colcon_ws/src/` 
    1. `git clone -b foxy-devel https://github.com/ROBOTIS-GIT/DynamixelSDK.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/dynamixel-workbench.git`  
    1. `git clone -b foxy-devel https://github.com/ROBOTIS-GIT/open_manipulator.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/open_manipulator_msgs.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/open_manipulator_dependencies.git`  
    1. `git clone -b ros2 https://github.com/ROBOTIS-GIT/robotis_manipulator.git`  


## Setup robot specific settings
1. Move package into ros: `cp -r ~/eit-robotic-avatar/ros/my_package .`
1. Build packages `cd ~/colcon_ws && colcon build --symlink-install`   
1. Set USB latency to 1 ms using: `ros2 run open_manipulator_x_controller create_udev_rules`
1. `sudo apt install python3-pip`
1. `sudo pip3 install gpiozero rpi-gpio`
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