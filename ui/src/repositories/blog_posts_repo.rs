use realm_web_rs::{Client, Collection, bson::{doc, self, Bson}};
use std::{env, time::Duration};

use super::models::blog_post::BlogPost;

pub struct BlogPostsRepository {
    client: Client,
    collection: Collection,
    http_client: reqwest::Client
}

impl BlogPostsRepository {
    
    pub fn new() -> BlogPostsRepository {
        BlogPostsRepository {
            client: Client { 
                application_id: "data-ooknz".to_string(), 
                api_token: "s0IOoAn5zoYqZPY4qCQdmrB5At3oLuNK6IyTU29wdj8fUzYzBFsQL19dAH2ZpS2V".to_string(), 
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

    pub async fn get_latest(&self, limit: i32) -> Result<Vec<BlogPost>, String> {
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
            Ok(documents) => {
                let blog_posts: Vec<BlogPost> = documents.documents
                    .unwrap()
                    .into_iter()
                    .map(|doc| bson::from_bson(Bson::Document(doc)))
                    .filter(|blog_post| blog_post.is_ok())
                    .map(|blog_post| blog_post.unwrap())
                    .collect::<Vec<BlogPost>>();

                Ok(blog_posts)
            },
            Err(e) => Ok(vec![])
        }
    }
}