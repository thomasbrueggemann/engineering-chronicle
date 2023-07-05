#[macro_use]
extern crate rocket;

use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::{options::ClientOptions, Client};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{Request, Response};
use shared::blog_post::BlogPost;
use std::env;

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
    let connection_string = env::var("MONGODB_CONNECTION_STRING").unwrap();
    let opts = ClientOptions::parse(connection_string).await.unwrap();

    let mongo_client = Client::with_options(opts).unwrap();
    let db = mongo_client.database("engineeringchronicle");
    let blog_posts_col = db.collection::<BlogPost>("blogposts");

    let find_options = FindOptions::builder()
        .limit(100)
        .sort(doc!{"published": -1})
        .build();

    let cursor = blog_posts_col.find(doc! {}, find_options).await.unwrap();
    let blog_posts: Vec<BlogPost> = cursor.try_collect().await.unwrap();

    let trunc: Vec<BlogPost> = blog_posts.into_iter().map(|mut post| {
        if post.content.len() > 250 {
            post.content = format!("{}[…]", post.content[..250].to_string());
        }

        post
    }).collect();

    Json(trunc)
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
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, latest_posts])
}