use std::time::Duration;

use anyhow::{anyhow, Result};
use chrono::Utc;
use dissolve::strip_html_tags;
use feed_rs::parser;
use opml::OPML;
use reqwest;
use shared::blog_post::{Blog, BlogPost};

pub async fn parse_blog(blog: Blog) -> Result<Vec<BlogPost>> {
    let content = download_content(&blog.url).await?;
    let rss = parser::parse(content.as_bytes())?;

    let blog_posts = rss
        .entries
        .into_iter()
        .filter_map(|entry| {
            if let Some(title) = entry.title {
                let content = match entry.summary {
                    Some(summary) => summary.content,
                    None => entry.content?.body.unwrap_or_default(),
                };

                let categories = entry.categories.into_iter().map(|c| c.term).collect();

                Some(BlogPost {
                    url: entry.id,
                    content: strip_html_tags(&content).join(""),
                    title: title.content,
                    blog: blog.clone(),
                    published: entry.published.or(Some(Utc::now()))?,
                    categories
                })
            } else {
                None
            }
        })
        .collect();

    Ok(blog_posts)
}

pub async fn get_blogs() -> Result<Vec<Blog>> {
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
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client.get(url).send().await?;

    if response.status() != 200 {
        return Err(anyhow!("Status code {}", response.status()));
    }

    let content = response.text().await?;

    Ok(content)
}
