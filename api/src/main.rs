#[macro_use]
extern crate rocket;

use std::env;

use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::Database;
use mongodb::{options::ClientOptions, Client};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{Request, Response};
use shared::blog_post::BlogPost;
use tokio::sync::OnceCell;
use unicode_truncate::UnicodeTruncateStr;

static DB: OnceCell<Database> = OnceCell::new();

#[get("/")]
fn index() -> &'static str {
    "The Engineering Chronicle API v1.0.0"
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BlogPostsResponse {
    title: String,
}

#[get("/latest")]
async fn latest_posts() -> Json<Vec<BlogPost>> {
    let blog_posts_col = DB.get().unwrap().collection::<BlogPost>("blogposts");

    let find_options = FindOptions::builder()
        .limit(100)
        .sort(doc! {"published": -1})
        .build();

    let cursor = blog_posts_col.find(doc! {}, find_options).await.unwrap();
    let blog_posts: Vec<BlogPost> = cursor.try_collect().await.unwrap();

    let trunc: Vec<BlogPost> = blog_posts
        .into_iter()
        .map(|mut post| {
            if post.content.len() > 250 {
                let (content_truncated, _) = post.content.unicode_truncate(250);
                post.content = format!("{}[…]", content_truncated);
            }

            post
        })
        .collect();

    Json(trunc)
}

#[get("/topic/<search_term>")]
async fn topic(search_term: String) -> Json<Vec<BlogPost>> {
    let blog_posts_col = DB.get().unwrap().collection::<BlogPost>("blogposts");

    let pipeline = vec![
        doc! {
            "$search": {
                "index": "search",
                "highlight": {
                    "path": ["title", "content"]
                },
                "count": {
                    "type": "lowerBound"
                },
                "scoreDetails": true,
                "sort": {
                    "published": -1
                },
                "tracking": {
                    "searchTerms": search_term.to_string()
                }
            }
        },
        doc! {
            "$facet": {
                "docs": [],
                "meta": [
                {"$replaceWith": "$$SEARCH_META"}
                ]
            }
        }
    ];

    let cursor = blog_posts_col.aggregate(pipeline, None).await. unwrap();
    let search_result = cursor.try_collect().await.unwrap();

    Json(search_result)
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    let connection_string = env::var("MONGODB_CONNECTION_STRING").unwrap();
    let opts = ClientOptions::parse(connection_string).await.unwrap();

    let mongo_client = Client::with_options(opts).unwrap();
    DB.set(mongo_client.database("engineeringchronicle"));

    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, latest_posts])
}
