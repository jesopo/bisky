use bisky::atproto::{Client, Session};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::Post;
use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use url::Url;

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
    post_text: String,
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

    println!(
        "{:#?}",
        client
            .feed_post(
                &args.username,
                Post {
                    text: args.post_text,
                    created_at: chrono::Utc::now(),
                }
            )
            .await
            .unwrap()
    );
}
