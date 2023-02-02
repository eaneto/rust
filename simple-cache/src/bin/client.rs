use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        response: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        response: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let task1 = tokio::spawn(async move {
        let (response_tx, response_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            response: response_tx,
        };

        tx.send(cmd).await.unwrap();

        let response = response_rx.await;
        println!("GOT = {:?}", response);
    });

    let task2 = tokio::spawn(async move {
        let (response_tx, response_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            response: response_tx,
        };

        tx2.send(cmd).await.unwrap();

        let response = response_rx.await;
        println!("GOT = {:?}", response);
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, response } => {
                    let res = client.get(&key).await;
                    let _ = response.send(res);
                }
                Set {
                    key,
                    value,
                    response,
                } => {
                    let res = client.set(&key, value).await;
                    let _ = response.send(res);
                }
            }
        }
    });

    task1.await.unwrap();
    task2.await.unwrap();
    manager.await.unwrap();
}
