use anyhow::{anyhow, Result};
use dissolve::strip_html_tags;
use feed_rs::parser;
use mongodb::{
    bson::doc,
    options::{ClientOptions, InsertManyOptions},
    Client,
};
use opml::OPML;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Blog {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BlogPost {
    pub url: String,
    pub title: String,
    pub content: String,
    pub blog: Blog,
}

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

async fn parse_blog(blog: &Blog) -> Result<Vec<BlogPost>> {
    let content = download_content(&blog.url).await?;
    let rss = parser::parse(content.as_bytes())?;

    let blog_posts = rss
        .entries
        .into_iter()
        .filter_map(|entry| {
            if let Some(title) = entry.title {
                let content = match entry.summary {
                    Some(summary) => summary.content,
                    None => entry.content.unwrap().body.unwrap_or_default(),
                };

                Some(BlogPost {
                    url: entry.id,
                    content: strip_html_tags(&content).join(""),
                    title: title.content,
                    blog: blog.clone(),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(blog_posts)
}

async fn get_blogs() -> Result<Vec<Blog>> {
    let rss_feeds_uri = "https://raw.githubusercontent.com/kilimchoi/engineering-blogs/master/engineering_blogs.opml";
    let rss_feeds_opml = download_content(rss_feeds_uri).await?;

    let document = OPML::from_str(&rss_feeds_opml)?;

    match document.body.outlines.first() {
        Some(outline) => {
            let blogs = outline
                .clone()
                .outlines
                .into_iter()
                .filter_map(|outline| {
                    if let Some(url) = outline.xml_url {
                        Some(Blog {
                            title: outline.text,
                            url,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            Ok(blogs)
        }
        None => Ok(vec![]),
    }
}

async fn download_content(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;

    if response.status() != 200 {
        return Err(anyhow!("Status code {}", response.status()));
    }

    let content = response.text().await?;

    Ok(content)
}
