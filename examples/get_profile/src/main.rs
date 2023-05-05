use bisky::atproto::{Client, ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::Post;
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
    #[clap(index = 5)]
    query: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let storage = Arc::new(File::<UserSession>::new(args.storage));

    let mut client= ClientBuilder::default().session(None).storage(storage).build().unwrap();
    client.login(&args.service, &args.username, &args.password).await;
    let mut bsky = Bluesky::new(client);
    let mut user = bsky.user(&args.query).unwrap();
    let profile = user.get_profile().await.unwrap();
    println!("Profile: {:#?}", profile);
    let likes = user.get_likes(100, None).await.unwrap();
    println!("Likes: {:#?}", likes);
    let follows = user.get_follows(100, None).await.unwrap();
    println!("Follows: {:#?}", follows);
    let followers = user.get_followers(100, None).await.unwrap();
    println!("Followers: {:#?}", followers);


}
