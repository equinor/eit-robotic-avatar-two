use gloo_storage::{LocalStorage, Storage};
use yew::Callback;

pub struct MinionModel {
    on_change: Callback<()>,
    cam_id: (String, String),
}

impl MinionModel {
    pub fn new(on_change: Callback<()>) -> MinionModel {
        let cam_id = LocalStorage::get("minion_cam_id").unwrap_or_default();

        MinionModel { on_change, cam_id }
    }

    pub fn action(&mut self, action: MinionAction) {
        match action {
            MinionAction::LeftCamChange(id) => {
                self.cam_id.0 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
                self.on_change.emit(());
            }
            MinionAction::RightCamChange(id) => {
                self.cam_id.1 = id;
                LocalStorage::set("minion_cam_id", self.cam_id.clone()).unwrap();
                self.on_change.emit(());
            }
        }
    }

    pub fn state(&self) -> MinionState {
        MinionState {
            cam_id: self.cam_id.clone(),
        }
    }
}

pub enum MinionAction {
    LeftCamChange(String),
    RightCamChange(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinionState {
    pub cam_id: (String, String),
}
