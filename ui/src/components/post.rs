use yew::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::models::blog_post::BlogPost;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub post: BlogPost,
}

#[function_component]
pub fn Post(props: &PostProps) -> Html {

    let post = props.post.clone();

    html!  {
        <div class="box">
            <article class="media">
                <div class="media-left">
                    <figure class="image is-64x64">
                    <img src="https://bulma.io/images/placeholders/128x128.png" alt="Image" />
                    </figure>
                </div>
                <div class="media-content">
                    <div class="content">
                        <a href={post.url} target="_blank"><strong>{post.title}</strong></a>
                        {" · "}
                        {post.blog.title}
                        {" · "}
                        <small>{readable_date(post.published.date.number_long)}</small>
                        <br />
                        {post.content}
                        {
                            if props.post.categories.len() > 0 {
                                html! {
                                    <>
                                        <br />
                                        <br />
                                        <div class="tags">
                                            {for props.post.categories.iter().map(|cat| {
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
}

fn readable_date(timestamp: String) -> String {
    let timestamp_parsed = timestamp.parse::<i64>().unwrap() / 1000;

    let parsed_date = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(timestamp_parsed, 0).unwrap(),
        Utc,
    );

    parsed_date.format("%d.%m.%Y %H:%Mh").to_string()
}
