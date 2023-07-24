use log::info;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{
    models::blog_post::BlogPost,
    repositories::blog_posts_repo::BlogPostsRepository, components::post::Post,
};

#[function_component]
pub fn Overview() -> Html {
    let state = use_async_with_options(
        async move {
            let blog_posts_repo = BlogPostsRepository::new();
            info!("asd");
            let result = blog_posts_repo.get_latest().await;

            return result;
        },
        UseAsyncOptions::enable_auto(),
    );

    if state.loading {
        return html! { <section class="section"><bold>{"Loading..."}</bold></section>};
    }

    if let Some(posts) = &state.data {
        let blog_posts: Vec<BlogPost> = posts.to_owned();

        html! {
            <>
                {for blog_posts.iter().map(|post| {
                    html! { <Post post={post.to_owned()} /> }
                })}
            </>
        }
    } else {
        html! {
            <section class="section">{"😢 nothing to see here"}</section>
        }
    }
}