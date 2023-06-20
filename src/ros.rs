use std::sync::{Arc, Mutex};

use brain::Head;
use pyo3::{
    types::{PyDict, PyModule},
    Py, PyAny, PyResult, Python,
};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tokio::{sync::watch::Receiver, task};

/// The python code to controll the arm compiled in.
const ARM_PY: &str = include_str!("./ros/arm.py");

pub struct Arm {
    module: Py<PyModule>,
    object: Py<PyAny>,
}

pub fn arm_start() -> Arm {
    Python::with_gil(|py| -> PyResult<Arm> {
        // compile and run python code.
        let arm_module = PyModule::from_code(py, ARM_PY, "arm.py", "arm")?;

        // run the arm_start function.
        let arm_object = arm_module.getattr("arm_start")?.call0()?;

        Ok(Arm {
            module: arm_module.into(),
            object: arm_object.into(),
        })
    })
    .unwrap()
}

pub fn arm_run(arm: &Arm, data: Head) {
    println!("Head: {:?}", data);
    Python::with_gil(|py| -> PyResult<()> {
        // Copy data into PyDict
        let py_data = PyDict::new(py);
        py_data.set_item("rx", data.rx)?;
        py_data.set_item("ry", data.ry)?;
        py_data.set_item("rz", data.rz)?;

        // Call the run function
        arm.module
            .getattr(py, "arm_run")?
            .call1(py, (&arm.object, py_data))?;
        Ok(())
    })
    .unwrap()
}

#[derive(Default)]
struct RoboticArm;

#[async_trait::async_trait]
impl Actor for RoboticArm {
    type Msg = brain::Head;
    type State = Arc<Mutex<Arm>>;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        let arm = task::spawn_blocking(arm_start).await.unwrap();
        Ok(Arc::new(Mutex::new(arm)))
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        data: Self::Msg,
        arm: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let arm = arm.clone();
        task::spawn_blocking(move || {
            if let Ok(arm) = arm.try_lock() {
                arm_run(&arm, data)
            }
        });
        Ok(())
    }
}

pub fn register(mut arm_receive: Receiver<brain::Head>) {
    task::spawn(async move {
        let (arm, _) = RoboticArm::spawn(None, RoboticArm, ()).await.unwrap();
        arm.send_message(*arm_receive.borrow()).unwrap();
        while let Ok(()) = arm_receive.changed().await {
            arm.send_message(*arm_receive.borrow()).unwrap();
        }
    });
}
