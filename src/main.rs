use dioxus::prelude::*;

use dioxus_desktop::LogicalSize;
use views::{Home};
use dioxus::desktop::tao;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    // #[route("/blog/:id")]
    // Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    let window = tao::window::WindowBuilder::new()
    .with_resizable(true)
    .with_title("Wentu")
    .with_min_inner_size(LogicalSize::new(900, 400))
    .with_inner_size(LogicalSize::new(900, 700))
    .with_focused(true);

    dioxus::LaunchBuilder::new().with_cfg(dioxus::desktop::Config::new().with_window(window)).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        Router::<Route> {}
    }
}
