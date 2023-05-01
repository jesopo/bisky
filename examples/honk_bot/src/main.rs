use bisky::atproto::{Client, ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::{Post, Embeds, ImagesEmbed};
use bisky::lexicon::app::bsky::embed::{Image};
use bisky::lexicon::app::bsky::notification::{Notification, NotificationRecord, ListNotificationsOutput};

use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::sync::Arc;
use std::fs;
use futures::{future, future::BoxFuture, stream, FutureExt, StreamExt}; // 0.3.13
use std::time::{Duration, Instant};
use tokio::time; // 1.3.0

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
    #[clap(index = 6)]
    image_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let storage = Arc::new(File::<UserSession>::new(args.storage));
    let image = fs::read(&args.image_path).unwrap();

    let mut client= ClientBuilder::default().session(None).storage(storage).build().unwrap();
    client.login(&args.service, &args.username, &args.password).await;
    // let mut bsky = Bluesky::new(client);
    // let mut me = bsky.me().unwrap();
    let good_text: ListNotificationsOutput<NotificationRecord> = serde_json::from_str("{\"notifications\":[{\"uri\":\"at://did:plc:kzgjymzlya5hezpidllt5tfm/app.bsky.feed.post/3juojivxzg225\",\"cid\":\"bafyreiex6b4cbm5ybrbcuucihan2vrsmxoqmfm46geadwa5v5ap5fkf3yu\",\"author\":{\"did\":\"did:plc:kzgjymzlya5hezpidllt5tfm\",\"handle\":\"mxkoder.bsky.social\",\"displayName\":\"Agnes 🦄🌈👩\u{200d}💻\",\"description\":\"Junior Software Engineer, UK public sector | @commandshift grad / #queertech / like to make things & PC build / opinions my own\\nPronouns: they/them\\n\\nHere for the tech + community 🙌🏻🦄🌈\",\"avatar\":\"https://cdn.bsky.social/imgproxy/9IFcsE__tMbIhAe-kz4_g2hU1HrrMRtzvTa74aQGj68/rs:fill:1000:1000:1:0/plain/bafkreic756qailqftgurfp67or4sgn4xldculpq53hvjacnhtqqvtm2boi@jpeg\",\"indexedAt\":\"2023-04-25T23:23:13.234Z\",\"viewer\":{\"muted\":false,\"blockedBy\":false,\"followedBy\":\"at://did:plc:kzgjymzlya5hezpidllt5tfm/app.bsky.graph.follow/3jua7ddgujt2r\"},\"labels\":[]},\"reason\":\"reply\",\"reasonSubject\":\"at://did:plc:4jfck5rfg4vhzfq5kt2z5wis/app.bsky.feed.post/3juoh4xodkf2n\",\"record\":{\"text\":\"I'm in a wok-obsessed phase, enjoy the cooking!\",\"$type\":\"app.bsky.feed.post\",\"reply\":{\"root\":{\"cid\":\"bafyreicv4ftonbyfxi4u76wp5uup5kmfag7zom3zjo7pxw6kgxzet3kdka\",\"uri\":\"at://did:plc:4jfck5rfg4vhzfq5kt2z5wis/app.bsky.feed.post/3juoh4xodkf2n\"},\"parent\":{\"cid\":\"bafyreicv4ftonbyfxi4u76wp5uup5kmfag7zom3zjo7pxw6kgxzet3kdka\",\"uri\":\"at://did:plc:4jfck5rfg4vhzfq5kt2z5wis/app.bsky.feed.post/3juoh4xodkf2n\"}},\"createdAt\":\"2023-05-01T15:56:30.346Z\"},\"isRead\":true,\"indexedAt\":\"2023-05-01T15:56:30.481Z\",\"labels\":[]}
    ]}").unwrap();
    println!("GOOD TEXT");

    let badd_text: ListNotificationsOutput<NotificationRecord> = serde_json::from_str("{\"notifications\":[{\"uri\":\"at://did:plc:xxfojudungpdlyjq3lvrx7by/app.bsky.feed.post/3jukkog7vn322\",\"cid\":\"bafyreia2ebngobny7zaqbd64sgtplozadhmdrxjsfc2v2iscrlymeromle\",\"author\":{\"did\":\"did:plc:xxfojudungpdlyjq3lvrx7by\",\"handle\":\"sleepytrekkie.bsky.social\",\"displayName\":\"Sleepy's Trek 🖖 🫡\",\"description\":\"TW: twitter.com/sleepytrekkie\\nYT: youtube.com/@thesleepycraftsman\\nYT: youtube.com/@thesleepytrekkie \\nFB: facebook.com/TheSleepyCraftsman\\n• Star Trek\\n• SciFi\\n• Technology\\n• Politics\\n• DIY\\n• Xennial\",\"avatar\":\"https://cdn.bsky.social/imgproxy/BQGdAeaj01I2v2WuNh4yf1RrYtUBV6KO4lj0ZavMv2U/rs:fill:1000:1000:1:0/plain/bafkreieiltn6spfdoquueojar6qxrjqe4574qcbgn4q4eeb2ubt4ntdy2u@jpeg\",\"indexedAt\":\"2023-04-30T15:07:02.725Z\",\"viewer\":{\"muted\":false,\"blockedBy\":false},\"labels\":[]},\"reason\":\"reply\",\"reasonSubject\":\"at://did:plc:4jfck5rfg4vhzfq5kt2z5wis/app.bsky.feed.post/3jukkixvlsm2s\",\"record\":{\"text\":\"\",\"$type\":\"app.bsky.feed.post\",\"embed\":{\"$type\":\"app.bsky.embed.images\",\"images\":[{\"alt\":\"\",\"image\":{\"ref\":{\"$link\":\"bafkreiauiod2um35p3q7k6sr6t4qjehuz7fclqcmy76qsvrsjlakzjsiha\"},\"size\":257449,\"$type\":\"blob\",\"mimeType\":\"image/jpeg\"}}]},\"reply\":{\"root\":{\"cid\":\"bafyreig7ox2h5kmcmjukbxfpopy65ggd2ymhbnldcu3fx72ij3c22ods3i\",\"uri\":\"at://did:plc:nx3kofpg4oxmkonqr6su5lw4/app.bsky.feed.post/3juhgsu4tpi2e\"},\"parent\":{\"cid\":\"bafyreiccsmpcpkkdodj4ig2itxq3gazi7idi4ahpuq4cuvmtbekj64jgk4\",\"uri\":\"at://did:plc:4jfck5rfg4vhzfq5kt2z5wis/app.bsky.feed.post/3jukkixvlsm2s\"}},\"createdAt\":\"2023-04-30T02:06:49.728Z\"},\"isRead\":true,\"indexedAt\":\"2023-04-30T02:06:50.059Z\",\"labels\":[]}],\"cursor\":\"1682820410059::bafyreia2ebngobny7zaqbd64sgtplozadhmdrxjsfc2v2iscrlymeromle\"}]}").unwrap();
    println!("BAD TEXT");
    let now = Instant::now();
    let forever = stream::unfold((), |()| async {
        eprintln!("Bisky Bluesky Bot Starting at {:?}", Instant::now());

        let poll = bot(client.clone());

        // Resolves when both the bot() function and a delay of 15 second is done
        future::join(poll, time::sleep(Duration::from_secs(15))).await;
        
        Some(((), ()))
    });

    /// The command that does everything the bot needs
    async fn bot(client: Client){
        println!("Running test");
        let mut bsky = Bluesky::new(client);
        let mut me = bsky.me().unwrap();
        let notification_count = me.get_notification_count(None).await.unwrap();
        println!("Notification Count: {:#?}", notification_count);
        let notifications = me.list_notifications(45).await.unwrap();
        let mentions =  notifications.into_iter().filter(|n| n.reason == "mention").collect::<Vec<Notification<NotificationRecord>>>();
        println!("Mentions\n{:#?}", mentions);
        // println!("Notifications\n{:#?}", notifications.into_iter().filter(|n| n.reason == "follow").collect::<Vec<Notification<NotificationRecord>>>());
        me.update_seen().await.unwrap();
     }

    forever.take(15).for_each(|_| async {}).await;
    eprintln!("Took {:?}", now.elapsed());
    
    // let blob_output = me.upload_blob(&image, "image/jpeg").await.unwrap();
    // println!("Blob: {:#?}", blob_output.blob);
    // let image = Image{image:blob_output.blob, alt: "HONK WITH RUST".to_string()};
    // let images_embed = ImagesEmbed{rust_type: "app.bsky.embed.images".to_string(), images: vec!(image)};
   
    // bsky
    //     .me()
    //     .unwrap()
    //     .post(Post {
    //         rust_type: Some("app.bsky.feed.post".to_string()),
    //         text: args.post_text,
    //         created_at: chrono::Utc::now(),
    //         embed: Some(images_embed),
    //     })
    //     .await
    //     .unwrap()
    
}
