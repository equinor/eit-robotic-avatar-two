use anyhow::Result;
use common::Tracking;
use robot::Server;
use tokio::sync::watch::{channel, Receiver, Sender};

pub fn tracking(server: Server) -> Receiver<Tracking> {
    let (sender, receiver) = channel(Tracking::default());
    tokio::spawn(async move {
        loop {
            if let Err(error) = get_tracking(&server, &sender).await {
                eprintln!("Server request error: {}", error)
            }
        }
    });
    receiver
}

async fn get_tracking(server: &Server, send: &Sender<Tracking>) -> Result<()> {
    let tracking = server
        .get("/api/minion/tracking")
        .send()
        .await?
        .json()
        .await?;
    println!("Tracking: {:?}", tracking);
    send.send(tracking)?;
    Ok(())
}
