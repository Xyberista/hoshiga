use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::*;

#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}
