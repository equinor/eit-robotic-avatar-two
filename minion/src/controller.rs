use brain::{Head, Tracking};
use tokio::sync::watch::{channel, Receiver};

pub type Pilot = Receiver<Tracking>;

pub type HeadsetTracking = Receiver<Tracking>;

pub type Drive = Receiver<brain::Drive>;

pub type RoboticArm = Receiver<brain::Head>;

pub struct ControllerInputs {
    pub pilot: Pilot,
}

pub struct ControllerOutputs {
    pub drive: Drive,
    pub arm: RoboticArm,
}

pub fn controller(inputs: ControllerInputs) -> ControllerOutputs {
    let (drive_send, drive_receive) = channel(brain::Drive::default());
    let (arm_send, arm_receive) = channel(Head::default());
    let mut pilot = inputs.pilot;

    tokio::spawn(async move {
        while let Ok(()) = pilot.changed().await {
            let _ = drive_send.send(pilot.borrow().drive);
            let _ = arm_send.send(pilot.borrow().head);
        }
    });

    ControllerOutputs {
        drive: drive_receive,
        arm: arm_receive,
    }
}
