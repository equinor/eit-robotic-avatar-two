use futures::join;
use web_sys::MediaStream;
use weblog::console_log;

use crate::services::Media;

pub async fn load_cams(left: &str, right: &str) -> (MediaStream, MediaStream) {
    let media = Media::new();

    // log list of devices to console.
    let devices = media.list_devices().await;
    for device in devices.iter() {
        console_log!(format!(
            "{:?}: {} id = {}",
            device.kind(),
            device.label(),
            device.device_id()
        ));
    }

    let left = media.get_user_video(left);
    let right = media.get_user_video(right);

    join!(left, right)
}
