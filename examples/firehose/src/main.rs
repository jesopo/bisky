use bisky::firehose::cbor::Body as FirehoseBody;
use bisky::lexicon::app::bsky::feed::Post;
use futures::{SinkExt as _, StreamExt as _};
use std::io::Cursor;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

#[tokio::main]
async fn main() {
    let (mut socket, _response) = tokio_tungstenite::connect_async(
        Url::parse("wss://bsky.social/xrpc/com.atproto.sync.subscribeRepos").unwrap(),
    )
    .await
    .unwrap();

    while let Some(Ok(Message::Binary(message))) = socket.next().await {
        let (_header, body) = bisky::firehose::cbor::read(&message).unwrap();
        match body {
            FirehoseBody::Commit(commit) => {
                if commit.operations.is_empty() {
                    continue;
                }
                let operation = &commit.operations[0];
                if !operation.path.starts_with("app.bsky.feed.post/") {
                    continue;
                }
                if let Some(cid) = operation.cid {
                    let mut car_reader = Cursor::new(commit.blocks);
                    let _car_header = bisky::firehose::car::read_header(&mut car_reader).unwrap();
                    let car_blocks = bisky::firehose::car::read_blocks(&mut car_reader).unwrap();

                    let record_reader = Cursor::new(car_blocks.get(&cid).unwrap());
                    let post = ciborium::de::from_reader::<Post, _>(record_reader);
                    println!("{post:?}");
                }
            }
            _ => {}
        }
    }
}
