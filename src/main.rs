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
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <div><Link<Route> to={Route::Embed { address: "testo".to_string() }}>{ "EMBED" }</Link<Route>></div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::NotFound => html! { <div>{ "404" }</div> },
        Route::Home => html! { <Home/> },
        Route::Embed { address } => html! { <ex::embed::Embed address={address}/> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}
