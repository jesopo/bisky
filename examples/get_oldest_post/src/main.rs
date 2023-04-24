use bisky::atproto::{Client, Session};
use bisky::bluesky::Bluesky;
use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// A file to store JSON Web Tokens in
    #[clap(index = 1)]
    storage: PathBuf,
    /// Which atproto service to connect to
    #[clap(index = 2)]
    service: Url,
    /// Username to log in with
    #[clap(index = 3)]
    username: String,
    /// Password to log in with
    #[clap(index = 4)]
    password: String,
    /// Username to get oldest post for
    #[clap(index = 5)]
    query: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let mut storage = File::<Session>::new(args.storage);
    if storage.get().await.is_err() {
        Client::login(&args.service, &args.username, &args.password, &mut storage)
            .await
            .unwrap();
    }

    let mut client = Bluesky::new(Client::new(args.service, storage).await.unwrap());
    let mut user = client.user(args.query);
    let posts = user.list_posts().await.unwrap();
    println!("oldest post: {:#?}", posts.last().unwrap());
}
