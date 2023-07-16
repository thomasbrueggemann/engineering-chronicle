use crate::models::blog_post::BlogPost;

pub struct BlogPostsRepository {
    http_client: reqwest::Client
}

impl BlogPostsRepository {
    
    pub fn new() -> BlogPostsRepository {
        BlogPostsRepository {
            http_client: reqwest::Client::new()
        }
    }

    pub async fn get_latest(&self) -> Result<Vec<BlogPost>, String> {
        let blog_posts = self.http_client.get("https://engineering-chronicle.thomasbrueggemann.com/latest")
            .send()
            .await
            .unwrap()
            .json::<Vec<BlogPost>>()
            .await
            .unwrap();

        Ok(blog_posts)
    }

    pub async fn get_topic(&self, search_term: String) -> Result<Vec<BlogPost>, String> {
        let url = format!("https://engineering-chronicle.thomasbrueggemann.com/topic/{}", &search_term);
        let blog_posts = self.http_client.get(url)
            .send()
            .await
            .unwrap()
            .json::<Vec<BlogPost>>()
            .await
            .unwrap();

        Ok(blog_posts)
    }
}