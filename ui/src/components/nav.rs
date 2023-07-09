use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    let modal_active = use_state(|| false);
    let storage = use_local_storage::<Vec<String>>("search_terms".to_string());
    let active_tab = use_state(|| "latest".to_string());
    let modal_search_term = use_state(|| String::new());

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
        let active_tab = active_tab.clone();
        let modal_search_term = modal_search_term.clone();

        Callback::from(move |_| {
            let t = modal_search_term.to_string();

            if let Some(value) = &*storage {
                let mut search_terms = value.clone();
                search_terms.push(t);

                storage.set(search_terms)
            } else {
                storage.set(vec![t]);
            }

            active_tab.set(modal_search_term.to_string());
            modal_search_term.set(String::new());

            modal_active.set(false)
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
            <div class="tabs is-large is-boxed">
                <ul>
                    <li class={active_tab_classes("latest", &*active_tab)}><a>{"Latest"}</a></li>
                    {
                        if let Some(value) = &*storage {
                            html! {
                                <>
                                    {
                                        for value.clone().iter().map(|s| {
                                            html! {
                                                <li class={active_tab_classes(&s, &*active_tab)}><a>{s}</a></li>
                                            }
                                        })
                                    }
                                </>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <li><a onclick={&on_open_modal}>{"+"}</a></li>
                </ul>
            </div>
        </>
    }
}

fn active_tab_classes(tab_name: &str, active_tab: &str) -> Classes {
    let is_active = tab_name.eq(active_tab);
    if is_active {
        classes!("is-active")
    }
    else {
        classes!()
    }
}