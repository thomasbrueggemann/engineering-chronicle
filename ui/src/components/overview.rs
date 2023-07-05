use log::info;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::repositories::blog_posts_repo::BlogPostsRepository;

#[function_component]
pub fn Overview() -> Html {
    let state = use_async_with_options(async move {
         let blog_posts_repo = BlogPostsRepository::new();
         info!("asd");
         let result = blog_posts_repo.get_latest(100).await;

         return result;
    }, UseAsyncOptions::enable_auto());

    if state.loading {
        return html! { <bold>{"Loading..."}</bold>}
    }

    html! {
        <section class="section">{"hi"}</section>
    }
}