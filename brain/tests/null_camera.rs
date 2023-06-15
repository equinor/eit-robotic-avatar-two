use std::time::Duration;

use brain::null_camera;
use tokio::time::timeout;

#[tokio::test]
#[ignore = "Not implemented yet"]
async fn will_not_send_new_picture() {
    let mut camera = null_camera();
    let result = timeout(Duration::from_secs(1), camera.changed()).await;

    assert!(
        result.is_err(),
        "Did not timeout resulted in: {:?}",
        result.unwrap()
    );
}
