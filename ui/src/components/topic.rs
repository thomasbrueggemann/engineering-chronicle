use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions, use_local_storage};

use crate::{models::{blog_post::BlogPost, search_term::SearchTerm}, components::post::Post, repositories::blog_posts_repo::BlogPostsRepository};

#[derive(Properties, PartialEq)]
pub struct TopicProps {
    pub id: String,
}

#[function_component]
pub fn Topic(props: &TopicProps) -> Html {

    let storage = use_local_storage::<Vec<SearchTerm>>("search_terms".to_string());
    let id = props.id.clone();

    let state = use_async_with_options(
        async move {
            let blog_posts_repo = BlogPostsRepository::new();

            if let Some(value) = &*storage {
                let all_stored_search_terms = value.clone();
                let search_term_for_id = all_stored_search_terms
                    .into_iter()
                    .find(|item| item.id.eq(&id));

                if let Some(search_term) = search_term_for_id {
                    let result = blog_posts_repo.get_topic(search_term.search_term).await;
                    return result;
                }
            }

            return Err("".to_string())
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