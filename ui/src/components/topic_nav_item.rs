use yew::prelude::*;
use yew_hooks::use_local_storage;
use yew_router::prelude::*;

use crate::{route::Route, models::search_term::SearchTerm};

#[derive(Properties, PartialEq)]
pub struct TopicNavItemProps {
    pub id: String,
    pub title: String
}


#[function_component]
pub fn TopicNavItem(props: &TopicNavItemProps) -> Html {

    let delete_armed = use_state(|| false);
    let storage = use_local_storage::<Vec<SearchTerm>>("search_terms".to_string());
    let navigator = use_navigator().unwrap();

    let on_delete_click = {
        let delete_state = delete_armed.clone();
        let id = props.id.clone();

        Callback::from(move |_| {
            if !*delete_state {
                delete_state.set(true);
            }
            else {
                if let Some(value) = &*storage {
                    let search_terms = value
                        .clone()
                        .into_iter()
                        .filter(|x| !x.id.eq(&id))
                        .collect();
                    
                    storage.set(search_terms);
                    navigator.push(&Route::Overview);
                }
            }
        })
    };

    let mut delete_classes = vec!["delete", "is-pulled-right", "is-small", "mt-1"];
    if *delete_armed {
        delete_classes.push("has-background-danger");
    }

    html! {
        <Link<Route> to={Route::Topic{ id: props.id.to_string()}} classes={classes!["pr-0"]}>
            {&props.title}
            <button onclick={on_delete_click} class={classes!(delete_classes)}></button>
        </Link<Route>>
    }
}