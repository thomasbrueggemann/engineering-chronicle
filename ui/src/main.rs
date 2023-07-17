use components::overview::Overview;
use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::components::topic::Topic;

mod components;
mod route;
mod repositories;
mod models;

fn routes(routes: Route) -> Html {
    match routes {
        Route::Overview => html! { <Overview /> },
        Route::Topic { id } => html! {
            <Topic id={id} />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container is-max-desktop">
                <center class="pb-6 pt-6">
                    <h1 class="is-size-1" style="font-family: 'Queen Love Stories free', sans-serif;">{"The Engineering Chronicle"}</h1>
                </center>
            
                <div class="columns">
                    <div class="column">
                        <Nav />
                    </div>
                    <div class="column is-10">
                        <Switch<Route> render={routes} />
                    </div>
                </div>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}