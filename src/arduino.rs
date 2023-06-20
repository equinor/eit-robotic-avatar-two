use std::{
    io,
    sync::{Arc, Mutex},
};

use ractor::{Actor, ActorProcessingErr, ActorRef};
use serialport::SerialPort;
use tokio::{sync::watch::Receiver, task};

struct DriveInner {
    serial: Box<dyn SerialPort>,
}

impl DriveInner {
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

fn drive_start() -> DriveInner {
    println!("Starting drive");
    DriveInner::new()
}

fn drive_run(drive: &mut DriveInner, data: brain::Drive) {
    let y = data.speed;
    let x = data.turn;

    let w = (1.0 - f64::abs(y)) * (x) + x;
    let v = (1.0 - f64::abs(x)) * (y) + y;

    let left = -(v - w) / 2.0;
    let right = -(v + w) / 2.0;

    drive.set_speed(left, right);
}

#[derive(Default)]
struct Drive;

#[async_trait::async_trait]
impl Actor for Drive {
    type Msg = brain::Drive;
    type State = Arc<Mutex<DriveInner>>;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        let drive = task::spawn_blocking(drive_start).await.unwrap();
        Ok(Arc::new(Mutex::new(drive)))
    }

    // This is our main message handler
    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        data: Self::Msg,
        drive: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let drive = drive.clone();
        task::spawn_blocking(move || {
            if let Ok(mut drive) = drive.try_lock() {
                drive_run(&mut drive, data)
            }
        });
        Ok(())
    }
}

pub fn register(mut drive_receive: Receiver<brain::Drive>) {
    task::spawn(async move {
        let (drive, _) = Drive::spawn(None, Drive, ()).await.unwrap();
        drive.send_message(*drive_receive.borrow()).unwrap();
        while let Ok(()) = drive_receive.changed().await {
            drive.send_message(*drive_receive.borrow()).unwrap();
        }
    });
}
