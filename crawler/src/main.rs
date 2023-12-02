mod blogs;

use anyhow::Result;
use blogs::{get_blogs, parse_blog};
use futures::stream::FuturesUnordered;
use shared::blog_post::BlogPost;
use stopwatch::Stopwatch;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {

    let mut watch = Stopwatch::start_new();

    println!("Start reading engineering blogs... ");

    let blogs = get_blogs().await?;

    println!("✅ {} identified engineering blogs [{}s]", blogs.len(), watch.elapsed_ms() / 1000);
    println!("Start downloading blog feeds into memory... ");
    watch.restart();

    let parse_all_blogs_tasks: FuturesUnordered<_> = blogs
        .into_iter()
        .map(|blog| tokio::spawn(parse_blog(blog)))
        .collect();

    let blog_post_results = futures::future::join_all(parse_all_blogs_tasks).await;
    println!("✅ {} blog feeds downloaded [{}s]", blog_post_results.len(), watch.elapsed_ms() / 1000);

    println!("Start inserting blog posts into database...");
    watch.restart();

    let blog_posts: Vec<BlogPost> = blog_post_results
        .into_iter()
        .filter_map(|blog_post_result| blog_post_result.ok())
        .flatten()
        .flatten()
        .collect();

    let json_str = serde_json::to_string_pretty(&blog_posts)?;
    let mut file = File::create("blog_posts.json")?;
    file.write_all(json_str.as_bytes())?;

    println!("✅ Done crawling [{}s]", watch.elapsed_ms() / 1000);
    watch.stop();

    Ok(())
}