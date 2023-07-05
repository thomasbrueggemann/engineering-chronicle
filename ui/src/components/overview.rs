use log::info;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{repositories::blog_posts_repo::BlogPostsRepository, models::blog_post::{BlogPost, Blog}};

#[function_component]
pub fn Overview() -> Html {
    let state = use_async_with_options(async move {
         let blog_posts_repo = BlogPostsRepository::new();
         info!("asd");
         let result = blog_posts_repo.get_latest().await;

         return result;
    }, UseAsyncOptions::enable_auto());

    if state.loading {
        return html! { <section class="section"><bold>{"Loading..."}</bold></section>}
    }

    if let Some(posts) = &state.data {

        let blog_posts: Vec<BlogPost> = posts.to_owned();

        html! {
            <>
                <section class="section">
                <h1 class="title">{"📣 The Engineering Chronicle"}</h1>
                {for blog_posts.iter().map(|post| {

                    let p = post.to_owned();
                    html! {
                        <div class="box">
                        <article class="media">
                        <div class="media-left">
                            <figure class="image is-64x64">
                            <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                            </figure>
                        </div>
                        <div class="media-content">
                            <div class="content">
                            <p>
                                <strong>{p.title}</strong> {" · "} {p.blog.title} {" · "} <small>{p.published.date.number_long}</small>
                                <br />
                                {p.content}
                            </p>
                            </div>
                            <nav class="level is-mobile">
                            <div class="level-left">
                                <a class="level-item" aria-label="reply">
                                <span class="icon is-small">
                                    <i class="fas fa-reply" aria-hidden="true"></i>
                                </span>
                                </a>
                                <a class="level-item" aria-label="retweet">
                                <span class="icon is-small">
                                    <i class="fas fa-retweet" aria-hidden="true"></i>
                                </span>
                                </a>
                                <a class="level-item" aria-label="like">
                                <span class="icon is-small">
                                    <i class="fas fa-heart" aria-hidden="true"></i>
                                </span>
                                </a>
                            </div>
                            </nav>
                        </div>
                        </article>
                    </div>
                    }
                })}
                </section>
            </>
        }
    }
    else {
        html! {
            <section class="section">{"😢 nothing to see here"}</section>
        }
    }
    
}