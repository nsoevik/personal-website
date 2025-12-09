use dioxus::{logger::tracing, prelude::*};

#[derive(PartialEq, Props, Clone)]
pub struct DogAppProps {
    pub breed: String,
}

#[component]
pub fn DogApp(props: DogAppProps) -> Element {
    tracing::info!("rendered");
    rsx! { div { "Dog breed: {props.breed}" } }
}
