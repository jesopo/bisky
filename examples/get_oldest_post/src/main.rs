use bisky::atproto::{Client, UserSession};
use bisky::bluesky::Bluesky;
use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(index = 1)]
    service: Url,
    #[clap(index = 2)]
    username: String,
    #[clap(index = 3)]
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    // let mut storage = File::<Session>::new(args.storage);
    
    // Create Client from Storage if tokens are not found.
    // TODO: Check if tokens are expired 
    // if storage.get().await.is_err() {
    let mut client = Client::login(&args.service, &args.username, &args.password)
        .await
        .unwrap();
    // }

    let mut bsky = Bluesky::new(client);
    println!("Client\n{:#?}", bsky);
    let mut user = bsky.user(args.username);
    println!("User\n{:#?}", user);
    let posts = user.list_posts().await.unwrap();
    println!("Posts\n{:#?}", posts);
    println!("oldest post: {:#?}", posts.last().unwrap());
}
