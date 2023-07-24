use log::info;
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::{models::search_term::SearchTerm, route::Route, components::topic_nav_item::TopicNavItem};

#[function_component]
pub fn Nav() -> Html {
    let modal_active = use_state(|| false);
    let storage = use_local_storage::<Vec<SearchTerm>>("search_terms".to_string());
    
    let modal_search_term = use_state(|| String::new());
    let navigator = use_navigator().unwrap();
    let current_route = use_route::<Route>();

    let on_open_modal = {
        let modal_active = modal_active.clone();
        Callback::from(move |_| modal_active.set(true))
    };

    let on_close_modal = {
        let modal_active = modal_active.clone();
        Callback::from(move |_| modal_active.set(false))
    };

    let on_save_click = {
        let storage = storage.clone();
        let modal_active = modal_active.clone();
        let modal_search_term = modal_search_term.clone();

        Callback::from(move |_| {
            let t = &modal_search_term;
            let new_search_term = SearchTerm::new(t);
            let new_search_term_id = new_search_term.id.clone();

            if let Some(value) = &*storage {
                let mut search_terms = value.clone();
                search_terms.push(new_search_term);

                storage.set(search_terms)
            } else {
                storage.set(vec![new_search_term]);
            }

            modal_search_term.set(String::new());
            modal_active.set(false);

            navigator.push(&Route::Topic { id: new_search_term_id });
        })
    };

    let oninput = Callback::from({
        let modal_search_term = modal_search_term.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();

            modal_search_term.set(target.value());
        }
    });

    let modal_class = if *modal_active {
        classes!("modal", "is-active")
    } else {
        classes!("modal")
    };

    html! {
        <>
            <div class={modal_class}>
                <div class="modal-background" onclick={&on_close_modal}></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Add search term"}</p>
                        <button class="delete" aria-label="close" onclick={&on_close_modal}></button>
                    </header>
                    <section class="modal-card-body">
                        <div class="field">
                            <div class="field-body">
                                <div class="field">
                                    <p class="control">
                                        <input class="input" type="text" {oninput} placeholder="What are you interested in?" />
                                    </p>
                                </div>
                            </div>
                        </div>

                        <div class="field">
                            <div class="field-body">
                                <div class="field">
                                    <p class="control">
                                        <button class="button is-success" onclick={&on_save_click}>{"Save"}</button>
                                    </p>
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </div>

            <aside class="menu">
                <p class="menu-label">
                    {"General"}
                </p>
                <ul class="menu-list">
                    <li>
                        <Link<Route> to={Route::Overview} classes={active_tab_classes(Route::Overview, &current_route)}>
                            {"Latest"}
                        </Link<Route>>
                    </li>
                </ul>
                <p class="menu-label mt-6">
                    {"Topics"}
                </p>
                <ul class="menu-list">
                    {
                        if let Some(value) = &*storage {
                            html! {
                                <>
                                    {
                                        for value.clone().iter().map(|s| {
                                            html! {
                                                <li>
                                                    <TopicNavItem 
                                                        title={s.search_term.clone()} 
                                                        is_active={is_active(Route::Topic { id: s.search_term.clone() }, &current_route)} 
                                                        id={s.id.clone()} />     
                                                </li>
                                            }
                                        })
                                    }
                                </>
                            }
                        } else {
                            html! {}
                        }
                    }

                    <li><button class="mt-3 button is-primary is-fullwidth is-outlined" onclick={&on_open_modal}>{"+ add"}</button></li>
                </ul>
            </aside>
        </>
    }
}

fn active_tab_classes(nav_route: Route, current_route: &Option<Route>) -> Classes {
    if is_active(nav_route, current_route) {
        classes!("is-active")
    } 
    else {
        classes!("")
    }
}

fn is_active(nav_route: Route, current_route: &Option<Route>) -> bool {
    match current_route {
        Some(route) => {

            info!("{:?}", current_route);
            let active_route = route.clone();

            if nav_route == active_route {
                info!("{:?} = {:?}", nav_route, active_route);
                true
            }
            else {
                false
            }
        },
        None => false
    }
}