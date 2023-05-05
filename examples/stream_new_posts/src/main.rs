use bisky::atproto::{ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(index = 1)]
    storage: PathBuf,
    #[clap(index = 2)]
    service: Url,
    #[clap(index = 3)]
    username: String,
    #[clap(index = 4)]
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let storage = Arc::new(File::<UserSession>::new(args.storage));
    let mut client= ClientBuilder::default().session(None).storage(storage).build().unwrap();

    client.login(&args.service, &args.username, &args.password)
    .await
    .unwrap();

    let mut bsky = Bluesky::new(client);
    let mut profile = bsky.user(args.username).unwrap();
    let mut stream = profile.stream_posts().await.unwrap();

    while let Ok(record) = stream.next().await {
        println!("{:#?}", record);
    }
}
