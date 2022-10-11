use rust_gpiozero::{Motor, OutputDevice};

pub struct Drive {
    front_left: Wheel,
    front_right: Wheel,
    back_left: Wheel,
    back_right: Wheel,
}

impl Drive {
    fn new() -> Self {
        // Setting up the GPIO pins for the wheels.
        Self {
            front_left: Wheel::new(20, 16, 21),
            front_right: Wheel::new(13, 19, 26),
            back_left: Wheel::new(23, 24, 25),
            back_right: Wheel::new(17, 27, 22),
        }
    }

    fn set_speed(&mut self, left: f64, right: f64) {
        self.front_left.set_speed(left);
        self.front_right.set_speed(right);
        self.back_left.set_speed(left);
        self.back_right.set_speed(right);
    }
}

struct Wheel {
    motor: Motor,
    _enable: OutputDevice,
}

impl Wheel {
    fn new(forward_pin: u8, backward_pin: u8, enable_pin: u8) -> Self {
        let mut enable = OutputDevice::new(enable_pin);
        enable.on();
        Self {
            motor: Motor::new(forward_pin, backward_pin),
            _enable: enable,
        }
    }

    fn set_speed(&mut self, speed: f64) {
        self.motor.set_speed(f64::abs(speed));
        if speed.signum() == 1.0 {
            self.motor.forward();
        } else {
            self.motor.backward();
        }
    }
}

pub fn drive_start() -> Drive {
    println!("Starting drive");
    Drive::new()
}

pub fn drive_run(drive: &mut Drive, data: common::Drive) {
    let y = data.speed;
    let x = data.turn;

    let w = (1.0 - f64::abs(y)) * (x) + x;
    let v = (1.0 - f64::abs(x)) * (y) + y;

    let left = -(v - w) / 2.0;
    let right = -(v + w) / 2.0;

    drive.set_speed(left, right);
}
