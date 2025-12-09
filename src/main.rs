use dioxus::prelude::*;

use crate::dog_app::DogApp;
mod dog_app;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        DogApp{
            breed: "test".to_string()
        }
    }
}
