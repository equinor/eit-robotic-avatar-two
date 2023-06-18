# Robotic Avatar Visionary Architecture

## Main building block.
The avatar controller talks to its environment using device definitions.

Some devices are abstract where both the definition and implementation is in the controller module. Abstract devises often model more high level concepts like position, locomotion, piloting. And is often implemented using one or more existing devices.

Environments are optional modules that can be enabled at build time and can implement one or more device or abstract devices.

Controller and environments are configured at startup.

```mermaid
flowchart
    subgraph environment_a
        device_impl[Device implementation]
    end

    subgraph environment_b
        abstract_device_impl[Abstract device implementation]
    end

    subgraph core
        device_def[Device definition]
        abstract_device[Abstract device definition and implementation]
        core_logic[Controller]

        device_def <--> abstract_device <--> core_logic
    end

    device_impl -.-> device_def
    abstract_device_impl -.-> abstract_device
```

## "Current" devices:

Devices for how Robotic Avatar works right now. But not implemented as devices.

```mermaid
flowchart
    webcam[Webcam]
    camera[Abstract cameras]
    headset[VR headset]
    pilot[Abstract pilot]
    robot_arm[Robotic Arm]
    drive[Abstract drive]
    motors[Motor]
    controller

    webcam-->|2x|camera-->controller-->headset
    headset-->pilot-->controller-->drive-->|4x|motors
    controller-->robot_arm
```

## "Current" Environments

Current environments but not implemented as environments yet.

### ROS
Implements:
* Robotic Arm

### Arduino
Implements:
* Motor

### Web
Implements:
* VR headset
* Webcam

## Planned Environments

### Robot
Implements:
* Robotic Arm
* Motor
* Webcam

### Web
Implements:
* VR headset

## Unreal 
Implements:
* Robotic Arm
* Motor
* Webcam

## Mock
Implements:
* Robotic Arm
* Motor
* Webcam
* VR headset

