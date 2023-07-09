use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    let modal_active = use_state(|| false);

    let on_open_modal = {
        let modal_active = modal_active.clone();
        Callback::from(move |_| modal_active.set(true))
    };

    let on_close_modal = {
        let modal_active = modal_active.clone();
        Callback::from(move |_| modal_active.set(false))
    };

    let storage = use_local_storage::<Vec<String>>("search_terms".to_string());

    let on_save_click = {
        let storage_search = storage.clone();
        Callback::from(move |_| {
            let mut search_terms = storage_search.unwrap_or(vec![]);
            storage.set(search_terms);
        })
    };

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
                                           <input class="input" type="text" placeholder="What are you interested in?" />
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
                    <li class="is-active"><a>{"Latest"}</a></li>
                    <li><a onclick={&on_open_modal}>{"+"}</a></li>
                </ul>
            </div>
        </>
    }
}
