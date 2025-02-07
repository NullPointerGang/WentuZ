use crate::components::{Hero, Footer};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Footer {}
    }
}
