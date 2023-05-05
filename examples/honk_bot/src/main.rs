use bisky::atproto::{Client, ClientBuilder, UserSession};
use bisky::bluesky::Bluesky;
use bisky::lexicon::app::bsky::feed::{ImagesEmbed, ReplyRef};
use bisky::lexicon::app::bsky::embed::{Image};
use bisky::lexicon::app::bsky::notification::{Notification, NotificationRecord};
use bisky::lexicon::com::atproto::repo::StrongRef;
use bisky::lexicon::app::bsky::feed::Post;
use bisky::lexicon::app::bsky::notification::NotificationRecord::Post as NotificationPost;
use bisky::lexicon::app::bsky::feed::Embeds;
use bisky::storage::File;
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::sync::Arc;
use std::fs;
use futures::{future, stream, StreamExt};
use std::time::{Duration, Instant};
use tokio::time;
use rand::Rng;

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

    let now = Instant::now();
    let forever = stream::unfold((), |()| async {
        eprintln!("Bisky Bluesky Bot Starting at {:?}", Instant::now());

        let poll = bot(client.clone(), image.clone());

        // Resolves when both the bot() function and a delay of 15 second is done
        future::join(poll, time::sleep(Duration::from_secs(15))).await;
        
        Some(((), ()))
    });

    /// The command that does everything the bot needs
    async fn bot(client: Client, img: Vec<u8>){
        let mut bsky = Bluesky::new(client);
        let mut me = bsky.me().unwrap();
        let notifications = me.list_notifications(10).await.unwrap();
        me.update_seen().await.unwrap();
        let mentions =  notifications.into_iter().filter(|n| (n.reason == "mention" && n.is_read == false)).collect::<Vec<Notification<NotificationRecord>>>();
        if !mentions.is_empty(){
            println!("Mentions\n{:#?}", mentions);
        }

        for mention in mentions{
            let uri = mention.uri;
            let cid = mention.cid;
            let  (text, reply) = match mention.record{
                NotificationRecord::Post(p) => (p.text, p.reply),
                _ => panic!("What are you feeding me Seymore?"),
            };
            println!("POSTText {:#?}",text);
            
            if text.contains("@benwis.bsky.social /honk"){
                println!("HONK");

                // As I understand this, if there is no parent(it is the root), then both parent and root uri/cid can be set to the ones in the root of the mention
                // If reply is Some(), then we'll need the uri/cid of the post from the root for parent, and the uri/cid from reply: root for the root
                let mut resp_reply_ref = ReplyRef{
                    parent: StrongRef{uri: uri.clone(), cid: cid.clone()}, 
                    root: StrongRef{uri, cid},
                };

                match reply{
                    Some(r) => {
                        resp_reply_ref.root.uri = r.root.uri;
                        resp_reply_ref.root.cid = r.root.cid;
                        },
                    None => ()
                };

                let blob_output = me.upload_blob(&img, "image/jpeg").await.unwrap();
                println!("Blob: {:#?}", blob_output.blob);
                let image = Image{image:blob_output.blob, alt: "HONK".to_string()};
                let images_embed = ImagesEmbed{images: vec!(image)};
                let embed = Embeds::Images(images_embed);
        
                me.post(Post {
                    rust_type: Some("app.bsky.feed.post".to_string()),
                    text: "HONK".to_string(),
                    created_at: chrono::Utc::now(),
                    embed: Some(embed),
                    reply: Some(resp_reply_ref),
                })
                .await
                .unwrap();

            }
            else if text.contains("@benwis.bsky.social /d20"){
                println!("D20");

                // As I understand this, if there is no parent(it is the root), then both parent and root uri/cid can be set to the ones in the root of the mention
                // If reply is Some(), then we'll need the uri/cid of the post from the root for parent, and the uri/cid from reply: root for the root
                let mut resp_reply_ref = ReplyRef{
                    parent: StrongRef{uri: uri.clone(), cid: cid.clone()}, 
                    root: StrongRef{uri, cid},
                };

                match reply{
                    Some(r) => {
                        resp_reply_ref.root.uri = r.root.uri;
                        resp_reply_ref.root.cid = r.root.cid;
                        },
                    None => ()
                };

                let roll = rand::thread_rng().gen_range(1..21);

                let msg = match roll{
                    i32::MIN..=0 => "You have broken the laws of physics! A D20 cannot be negative or 0!",
                    1 => "You stand in befuddlement, a great deal of time has passed, and you're no closer to accomplishing your goals. You become disheartened, and vow never to attempt it again!",
                    2..=5 => "You tried your hardest, and with great effort, suddenly fall flat on your face! A small child points and laughs, \"HA HA!\"",
                    6..=10 => "You see somebody walk by, see you, and then shake their head. They turn around and leave in the opposite direction. You faintly hear them mutter, \"I remember when people had standards...\"",
                    11..=15 => "A bead of sweat rolls down your forehead, as you behold the fruits of your labor. That'll do, you think to yourself. That'll do.",
                    16..=19 => "You have skill, and with little effort, accomplish everything you set out to do. In fact, you have time to do more things. Why can't more things go like this?",
                    20 => "It as if the gods themselves guide your endeavours. Songs could be sung about this work. You have brought great honor to yourself, your family, and your friends. It becomes a legend amonst your friends and community, ",
                    21_i32..=i32::MAX => "This cannot be! These acts befit a god more than a mortal",
                };
        
                me.post(Post {
                    rust_type: Some("app.bsky.feed.post".to_string()),
                    text: format!("You rolled a {roll}.\n{msg}"),
                    created_at: chrono::Utc::now(),
                    embed: None,
                    reply: Some(resp_reply_ref),
                })
                .await
                .unwrap();

            }
        }
     }
     forever.for_each(|_| async {}).await;  
}
