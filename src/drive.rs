use std::io;

use serialport::SerialPort;

pub struct Drive {
    serial: Box<dyn SerialPort>,
}

impl Drive {
    fn new() -> Self {
        let serial = serialport::new("/dev/ttyACM0", 115_200)
            .open()
            .expect("Failed to open port to arduino");
        Self { serial }
    }

    fn set_speed(&mut self, left: f64, right: f64) {
        let left = speed_to_bytes(left);
        let right = speed_to_bytes(right);

        let motor_buffer = [
            left[0], left[1], // front_left motor
            right[0], right[1], // front_right motor
            left[0], left[1], // back_left motor
            right[0], right[1], // back_right motor
        ];

        match self.send_buffer(&motor_buffer) {
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(err) => {
                println!("Failed write to arduino: {} ", err);
            }
            Ok(_) => {
                //println!("Drive write: {:?}", motor_buffer);
            }
        }
    }

    fn send_buffer(&mut self, buffer: &[u8]) -> Result<(), io::Error> {
        self.serial.write_all(buffer)?;
        let mut buffer = [0; 64];
        let _ = self.serial.read(&mut buffer[..])?;
        //self.serial.flush()?;
        Ok(())
    }
}

fn speed_to_bytes(speed: f64) -> [u8; 2] {
    let direction = if speed.signum() == 1.0 { 1 } else { 0 };
    let speed = (f64::abs(speed) * 256.0).round() as u8;

    [direction, speed]
}

pub fn drive_start() -> Drive {
    println!("Starting drive");
    Drive::new()
}

pub fn drive_run(drive: &mut Drive, data: brain::Drive) {
    let y = data.speed;
    let x = data.turn;

    let w = (1.0 - f64::abs(y)) * (x) + x;
    let v = (1.0 - f64::abs(x)) * (y) + y;

    let left = -(v - w) / 2.0;
    let right = -(v + w) / 2.0;

    drive.set_speed(left, right);
}
