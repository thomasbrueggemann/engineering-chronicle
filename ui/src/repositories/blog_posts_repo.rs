use realm_web_rs::{Client, Collection, bson::doc};
use std::{env, time::Duration};

use super::models::blog_post::BlogPost;

pub struct BlogPostsRepository {
    client: Client,
    collection: Collection,
    http_client: reqwest::Client
}

impl BlogPostsRepository {
    
    pub fn new() -> BlogPostsRepository {

        let mongodb_api_key = env::var("MONGODB_API_KEY")
            .expect("MongoDB api key not set via env var MONGODB_API_KEY!");

        BlogPostsRepository {
            client: Client { 
                application_id: "data-ooknz".to_string(), 
                api_token: mongodb_api_key, 
                api_version: realm_web_rs::ApiVersion::v1,
                deployment_region: Some("westeurope.azure".to_string()),
            },
            collection: Collection { 
                data_source: "EngineeringChronicle".to_string(),
                database: "engineeringchronicle".to_string(), 
                collection: "blogposts".to_string() 
            },
            http_client: reqwest::Client::builder()
                .build()
                .unwrap()
        }
    }

    pub async fn get_latest(&self, limit: i32) -> Result<Vec<BlogPost>, anyhow::Error> {
        let results = self.client.find(
            self.collection.clone(), 
            Some(doc!{}), 
            Some(doc! {
                "title": 1
            }),
            Some(doc! {
                "published": -1
            }),
            Some(limit), 
            Some(0), 
            &self.http_client).await;

        match results {
            Ok(blog_posts) => {
                Ok(vec![])
            },
            Err(e) => Ok(vec![])
        }
    }
}