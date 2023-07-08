use yew::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <div class="tabs is-large is-boxed">
        <ul>
            <li class="is-active"><a>{"Latest"}</a></li>
            <li><a>{"+"}</a></li>
        </ul>
        </div>
    }
}
