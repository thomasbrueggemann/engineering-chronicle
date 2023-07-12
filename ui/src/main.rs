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
            <div class="container">
                <section class="section">
                    <center class="mb-6">
                        <h1 class="is-size-1" style="font-family: 'Queen Love Stories free', sans-serif;">{"📣 The Engineering Chronicle"}</h1>
                    </center>
                </section>
            
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