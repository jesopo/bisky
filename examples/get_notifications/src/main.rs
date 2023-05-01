use bisky::atproto::{ClientBuilder, UserSession};
use bisky::{bluesky::Bluesky, storage::File};
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::sync::Arc;
use bisky::lexicon::app::bsky::notification::{Notification, NotificationRecord};

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
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let storage = Arc::new(File::<UserSession>::new(args.storage));
    
    // Create Client from Storage if tokens are not found.
    // TODO: Check if tokens are expired 
        // let mut client = ClientBuilder::default().session_from_storage(None, storage).await.build().unwrap();
        let mut client= ClientBuilder::default().session(None).storage(storage).build().unwrap();

        client.login(&args.service, &args.username, &args.password)
        .await
        .unwrap();

    let mut bsky = Bluesky::new(client);
    let mut me = bsky.me().unwrap();
    let notification_count = me.get_notification_count(None).await.unwrap();
    println!("Notif Count: {:#?}", notification_count);
    let notifications = me.list_notifications(30).await.unwrap();
    // // println!("Notifications\n{:#?}", notifications);
    // println!("Notifications\n{:#?}", notifications.into_iter().filter(|n| n.reason == "follow").collect::<Vec<Notification<NotificationRecord>>>());
    me.update_seen().await.unwrap();
}
