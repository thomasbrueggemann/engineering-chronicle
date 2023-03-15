mod blogs;
mod templates;

use anyhow::{anyhow, Result};
use blogs::{get_blogs, parse_blog, Blog, BlogPost};
use chrono::{DateTime, Utc};
use mongodb::{
    bson::doc,
    options::{ClientOptions, InsertManyOptions},
    Client,
};
use std::env;
use templates::render_html;

#[tokio::main]
async fn main() -> Result<()> {
    let blogs = get_blogs().await?;

    println!("{} blogs identified", blogs.len());

    let mongo_connection_string = env::var("MONGODB_CONNECTION_STRING")
        .expect("MongoDB connection string not set via environment variable!");

    let mongo_client_options = ClientOptions::parse(mongo_connection_string).await?;
    let mongo_client = Client::with_options(mongo_client_options)?;
    let db = mongo_client.database("thefabricatedfeed");
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

    Ok(())
}
