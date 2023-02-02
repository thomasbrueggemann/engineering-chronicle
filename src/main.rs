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
    Ok(())
}

async fn get_feeds() -> Result<Option<&Vec<Feed>>, Box<dyn std::error::Error + Send + Sync>> {
    let rss_feeds_uri = "https://raw.githubusercontent.com/kilimchoi/engineering-blogs/master/engineering_blogs.opml".parse()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let resp = client.get(rss_feeds_uri).await?;
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let rss_feeds_opml = String::from_utf8(body_bytes.to_vec()).unwrap();

    let document = OPML::from_str(&rss_feeds_opml).unwrap();

    for outline in document.body.outlines {
        for feed in outline.outlines {
            println!("{}", feed.title.unwrap());
        }

        let feeds = &outline
            .outlines
            .into_iter()
            .map(|feed| {
                return Feed {
                    title: feed.title.unwrap().to_owned(),
                    url: feed.url.unwrap().to_owned(),
                };
            })
            .collect::<Vec<Feed>>();

        return Ok(Some(feeds));
    }

    Ok(None)
}
