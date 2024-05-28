use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_websockets::Message;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(256);
    let rx_jh = tokio::spawn(rx_task(rx));
    let tx_jh = tokio::spawn(tx_task(tx));

    tx_jh.await.unwrap().unwrap();
    rx_jh.await.unwrap();
}

// This fails
// use std::error::Error;
// async fn tx_task(tx: Sender<Message>) -> Result<(), Box<dyn Error>> {

// So does this (as does using eyre::Result)
// use eyre::Report;
// async fn tx_task(tx: Sender<Message>) -> Result<(), Report> {

// But this works ok
use tokio::sync::mpsc::error::SendError;
async fn tx_task(tx: Sender<Message>) -> Result<(), SendError<Message>> {
    for i in 0..10 {
        let message = Message::text(format!("test: {i}"));
        tx.send(message).await?;
    }

    Ok(())
}

async fn rx_task(rx: Receiver<Message>) {
    let mut rx = rx;

    while let Some(message) = rx.recv().await {
        if let Some(text_message) = message.as_text() {
            println!("Received: {text_message}");
        }
    }
}
