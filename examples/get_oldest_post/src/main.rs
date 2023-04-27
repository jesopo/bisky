use bisky::atproto::{Client, UserSession};
use bisky::{bluesky::Bluesky, storage::{File, Storage}};
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

    let storage = File::<UserSession>::new(args.storage);
    
    // Create Client from Storage if tokens are not found.
    // TODO: Check if tokens are expired 
    // if storage.get().await.is_err() {
        let mut client = Client::builder().storage(storage).build();
        client.login(&args.service, &args.username, &args.password)
        .await
        .unwrap();
    // }

    let mut bsky = Bluesky::new(client);
    println!("Client\n{:#?}", bsky);
    let mut user = bsky.user(args.username).unwrap();
    println!("User\n{:#?}", user);
    let posts = user.list_posts().await.unwrap();
    println!("Posts\n{:#?}", posts);
    println!("oldest post: {:#?}", posts.last().unwrap());
}
