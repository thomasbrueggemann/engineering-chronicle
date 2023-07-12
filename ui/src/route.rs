use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Overview,

    #[at("/topic/:id")]
    Topic { id: String },
}