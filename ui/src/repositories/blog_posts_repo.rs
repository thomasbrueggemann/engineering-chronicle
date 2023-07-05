use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::models::blog_post::BlogPost;

pub struct BlogPostsRepository {
    application_id: String,
    api_key: String,
    deployment_region: String,
    data_source: String,
    database: String,
    collection: String,
    http_client: reqwest::Client
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct AuthenticationResponse {
    access_token: String,
    refresh_token: String,
    user_id: String
}

impl BlogPostsRepository {
    
    pub fn new() -> BlogPostsRepository {
        BlogPostsRepository {
            application_id: "data-ooknz".to_string(),
            api_key: "s0IOoAn5zoYqZPY4qCQdmrB5At3oLuNK6IyTU29wdj8fUzYzBFsQL19dAH2ZpS2V".to_string(),
            deployment_region: "westeurope.azure".to_string(),
            data_source: "EngineeringChronicle".to_string(),
            database: "engineeringchronicle".to_string(), 
            collection: "blogposts".to_string(),
            http_client: reqwest::Client::new()
        }
    }

    pub async fn get_access_token(&self) -> Result<String, String> {
        let url = format!("https://realm.mongodb.com/api/client/v2.0/app/myapp-abcde/auth/providers/api-key/login");

        let mut body = HashMap::new();
        body.insert("key", &self.api_key);

        let result = self.http_client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap()
            .json::<AuthenticationResponse>()
            .await
            .unwrap();

        Ok(result.access_token)
    }

    pub async fn get_latest(&self, limit: i32) -> Result<Vec<BlogPost>, String> {

        let access_token = self.get_access_token().await.unwrap();

    /*     let results = self.client.find(
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
            Err(_e) => Ok(vec![])
        }*/

        Ok(vec![])
    }
}