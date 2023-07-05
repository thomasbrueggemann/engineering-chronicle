use components::overview::Overview;
use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod route;

fn routes(routes: Route) -> Html {
    match routes {
        Route::Overview => html! { <Overview /> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={routes} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}