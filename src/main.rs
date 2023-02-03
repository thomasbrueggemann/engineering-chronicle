use hyper::Client;
use hyper_tls::HttpsConnector;
use opml::OPML;

#[derive(Clone)]
struct Feed {
    pub title: String,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let feeds = get_feeds().await?;

    Ok(())
}

async fn get_feeds() -> Result<Vec<Feed>, Box<dyn std::error::Error + Send + Sync>> {
    let rss_feeds_uri = "https://raw.githubusercontent.com/kilimchoi/engineering-blogs/master/engineering_blogs.opml".parse()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let resp = client.get(rss_feeds_uri).await?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let rss_feeds_opml = String::from_utf8(body_bytes.to_vec()).unwrap();

    let document = OPML::from_str(&rss_feeds_opml).unwrap();

    let outline = document.body.outlines.first().unwrap().to_owned();

    let result = outline
        .outlines
        .into_iter()
        .filter_map(|outline| {
            if let (Some(text), Some(url)) = (outline.title, outline.url) {
                Some(Feed { title: text, url })
            } else {
                None
            }
        })
        .collect();

    return Ok(result);
}
