use std::time::Duration;

use common::{Drive, Head, RtcIce};
use parking_lot::Mutex;
use time::Instant;

const TIMEOUT: Duration = Duration::from_millis(500);

pub struct Minion {
    movement: Mutex<Movement>,
    offer: Mutex<String>,
    answer: Mutex<String>,
    ice: RtcIce,
}

impl Minion {
    pub fn new(ice: RtcIce) -> Self {
        Self {
            movement: Mutex::new(Movement {
                head: Head::default(),
                drive: Drive::default(),
                age: Instant::now(),
            }),
            offer: Default::default(),
            answer: Default::default(),
            ice,
        }
    }

    pub fn movement(&self) -> (Head, Drive) {
        let movement = self.movement.lock();
        if movement.age.elapsed() < TIMEOUT {
            (movement.head, movement.drive)
        } else {
            (movement.head, Drive::default())
        }
    }

    pub fn movement_set(&self, head: Head, drive: Drive) {
        *self.movement.lock() = Movement {
            head,
            drive,
            age: Instant::now(),
        }
    }

    pub fn offer(&self) -> String {
        let offer = self.offer.lock();
        if offer.is_empty() {
            "{}".to_string()
        } else {
            offer.clone()
        }
    }

    pub fn set_offer(&self, offer: String) {
        {
            self.answer.lock().clear();
        }
        *self.offer.lock() = offer;
    }

    pub fn answer(&self) -> String {
        let answer = self.answer.lock();
        if answer.is_empty() {
            "{}".to_string()
        } else {
            answer.clone()
        }
    }

    pub fn set_answer(&self, answer: String) {
        {
            self.offer.lock().clear();
        }
        *self.answer.lock() = answer;
    }

    pub fn ice(&self) -> RtcIce {
        self.ice.clone()
    }
}

struct Movement {
    head: Head,
    drive: Drive,
    age: Instant,
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn movement() {
        let minion = Minion::new(RtcIce::default());

        let test_data = (
            Head {
                rx: 0.1,
                ry: 0.2,
                rz: 0.3,
            },
            Drive {
                speed: 1.0,
                turn: 1.0,
            },
        );
        minion.movement_set(test_data.0, test_data.1);

        assert_eq!(minion.movement(), test_data);
    }

    #[test]
    fn movement_timeout() {
        let minion = Minion::new(RtcIce::default());

        minion.movement_set(
            Head::default(),
            Drive {
                speed: 1.0,
                turn: 1.0,
            },
        );

        thread::sleep(Duration::from_secs(1));

        assert_eq!(minion.movement(), Default::default());
    }
}
