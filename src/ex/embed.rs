use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmbedProps {
    pub address: String,
}

#[function_component(Embed)]
pub fn embed(props: &EmbedProps) -> Html {
    html! {
        <div>{ &props.address }</div>
    }
}
