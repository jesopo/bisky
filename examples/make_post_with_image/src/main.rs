use bisky::atproto::{Client, ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::{Post, Embeds, ImagesEmbed};
use bisky::lexicon::app::bsky::embed::{Image};

use bisky::storage::{File, Storage as _};
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::sync::Arc;
use std::fs;

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
    let mut bsky = Bluesky::new(client);
    let mut me = bsky.me().unwrap();
    
    let blob_output = me.upload_blob(&image, "image/jpeg").await.unwrap();
    println!("Blob: {:#?}", blob_output.blob);
    let image = Image{image:blob_output.blob, alt: "HONK WITH RUST".to_string()};
    let images_embed = ImagesEmbed{rust_type: "app.bsky.embed.images".to_string(), images: vec!(image)};

    // let embed = Some(Embeds::Images(images_embed));

    println!(
        "{:#?}",
        bsky
            .me()
            .unwrap()
            .post(Post {
                rust_type: Some("app.bsky.feed.post".to_string()),
                text: args.post_text,
                created_at: chrono::Utc::now(),
                embed: Some(images_embed),
            })
            .await
            .unwrap()
    );
}
