mod blogs;

use anyhow::Result;
use blogs::{get_blogs, parse_blog, BlogPost};
use mongodb::{
    options::{ClientOptions, InsertManyOptions},
    Client,
};
use std::{env, process};

#[tokio::main]
async fn main() -> Result<()> {
    let blogs = get_blogs().await?;

    println!("{} blogs identified", blogs.len());

    let mongo_connection_string = env::var("MONGODB_CONNECTION_STRING")
        .expect("MongoDB connection string not set via environment variable!");

    let mongo_client_options = ClientOptions::parse(mongo_connection_string).await?;
    let mongo_client = Client::with_options(mongo_client_options)?;
    let db = mongo_client.database("engineeringchronicle");
    let blog_posts_col = db.collection::<BlogPost>("blogposts");

    for blog in blogs {
        if let Ok(blog_posts) = parse_blog(&blog).await {
            println!("{} posts for blog {}", blog_posts.len(), blog.url);

            let _ = blog_posts_col
                .insert_many(
                    blog_posts,
                    InsertManyOptions::builder().ordered(false).build(),
                )
                .await;
        }
    }

    println!("Done");
    process::exit(0);
}
