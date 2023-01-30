use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

mod ex;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/")]
    Home,

    #[at("/ex/embed/:address")]
    Embed { address: String },

    #[at("/ex/ws-client")]
    WsClient,

    #[at("/ex/ws-server")]
    WsServer,
}

#[function_component(Home)]
fn home() -> Html {
    let nav = Rc::new(use_navigator().unwrap());

    let embed_nav = nav.clone();
    let to_embed = Callback::from(move |_| {
        embed_nav.push(&Route::Embed {
            address: "testo".to_string(),
        })
    });

    let client_nav = nav.clone();
    let to_client = Callback::from(move |_| client_nav.push(&Route::WsClient));

    let server_nav = nav.clone();
    let to_server = Callback::from(move |_| server_nav.push(&Route::WsServer));

    html! {
        <>
            <button onclick={to_embed}>{"[embed]"}</button>
            <button onclick={to_client}>{"[client]"}</button>
            <button onclick={to_server}>{"[server]"}</button>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::NotFound => html! { <div>{"404"}</div> },
        Route::Home => html! { <Home/> },
        Route::Embed { address } => html! { <ex::embed::Embed address={address}/> },
        Route::WsClient => html! { <ex::ws::Client/> },
        Route::WsServer => html! { <ex::ws::Server/> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}
