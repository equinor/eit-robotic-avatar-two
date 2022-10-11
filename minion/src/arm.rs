use common::Head;
use pyo3::{
    types::{PyDict, PyModule},
    Py, PyAny, PyResult, Python,
};

/// The python code to controll the arm compiled in.
const ARM_PY: &str = include_str!("./arm.py");

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
