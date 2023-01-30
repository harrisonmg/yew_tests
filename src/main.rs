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
    html! {
        <>
            <div><Link<Route> to={Route::Embed { address: "testo".to_string() }}>{ "embed" }</Link<Route>></div>
            <div><Link<Route> to={Route::WsClient}>{ "client" }</Link<Route>></div>
            <div><Link<Route> to={Route::WsServer}>{ "server" }</Link<Route>></div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::NotFound => html! { <div>{ "404" }</div> },
        Route::Home => html! { <Home/> },
        Route::Embed { address } => html! { <ex::embed::Embed address={address}/> },
        Route::WsClient => html! { <ex::ws::Client/> },
        Route::WsServer => html! { <ex::ws::Server/> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    html! {
        <div id="wrapper">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}
