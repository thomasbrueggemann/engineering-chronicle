use chrono::{DateTime, NaiveDateTime, Utc};
use log::info;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::{
    components::nav::Nav,
    models::blog_post::{Blog, BlogPost},
    repositories::blog_posts_repo::BlogPostsRepository,
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
                <section class="section">
                <center class="mb-6">
                    <h1 class="is-size-1" style="font-family: 'Queen Love Stories free', sans-serif;">{"📣 The Engineering Chronicle"}</h1>
                </center>

                <Nav />

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
                                <a href={p.url} target="_blank"><strong>{p.title}</strong></a>
                                {" · "}
                                {p.blog.title}
                                {" · "}
                                <small>{readable_date(p.published.date.number_long)}</small>
                                <br />
                                {p.content}
                                {
                                    if p.categories.len() > 0 {
                                        html! {
                                            <>
                                                <br />
                                                <br />
                                                <div class="tags">
                                                    {for p.categories.iter().map(|cat| {
                                                        html! {
                                                            <span class="tag is-primary">{cat}</span>
                                                        }
                                                    })}
                                                </div>
                                            </>
                                        }
                                    }
                                    else {
                                        html! {}
                                    }
                                }
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
    } else {
        html! {
            <section class="section">{"😢 nothing to see here"}</section>
        }
    }
}

fn readable_date(timestamp: String) -> String {
    let timestamp_parsed = timestamp.parse::<i64>().unwrap() / 1000;

    let parsed_date = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp_parsed, 0).unwrap(),
        Utc,
    );

    parsed_date.format("%d.%m.%Y %H:%Mh").to_string()
}
