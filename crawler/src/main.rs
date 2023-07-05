mod blogs;

use anyhow::Result;
use blogs::{get_blogs, parse_blog};
use futures::stream::FuturesUnordered;
use mongodb::{
    options::{ClientOptions, InsertManyOptions},
    Client,
};
use shared::blog_post::BlogPost;
use stopwatch::Stopwatch;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    let mut watch = Stopwatch::start_new();

    println!("Start reading enginnering blogs... ");

    let blogs = get_blogs().await?;

    println!("✅ {} identified engineering blogs [{}s]", blogs.len(), watch.elapsed_ms() / 1000);

    let mongo_connection_string = env::var("MONGODB_CONNECTION_STRING")
        .expect("MongoDB connection string not set via env var MONGODB_CONNECTION_STRING!");

    let mongo_client_options = ClientOptions::parse(mongo_connection_string).await?;
    let mongo_client = Client::with_options(mongo_client_options)?;
    let db = mongo_client.database("engineeringchronicle");
    let blog_posts_col = db.collection::<BlogPost>("blogposts");

    println!("Start downloading blog feeds into memory... ");
    watch.restart();

    let parse_all_blogs_tasks: FuturesUnordered<_> = blogs
        .into_iter()
        .map(|blog| tokio::spawn(parse_blog(blog)))
        .collect();

    let blog_posts = futures::future::join_all(parse_all_blogs_tasks).await;
    println!("✅ {} blog feeds downloaded [{}s]", blog_posts.len(), watch.elapsed_ms() / 1000);

    println!("Start inserting blog posts into database...");
    watch.restart();

    for blog_post in blog_posts {
        if let Ok(posts) = blog_post.unwrap() {
            let _ = blog_posts_col
                .insert_many(
                    posts,
                    InsertManyOptions::builder().ordered(false).build(),
                )
                .await;
        }
    }

    println!("✅ Done inserting [{}s]", watch.elapsed_ms() / 1000);
    watch.stop();

    Ok(())
}
