mod app;
pub mod components;
pub mod routes;

use app::App;

fn main() {
    yew::start_app::<App>();
}
