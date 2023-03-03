//use futures::{SinkExt, StreamExt};
use yew::{platform::spawn_local, prelude::*};

#[function_component(Server)]
pub fn server() -> Html {
    //use_effect_with_deps(
    //    |_| {
    //        log::info!("what");
    //        let sock = WebSocket::open("wss://127.0.0.1:7777").unwrap();
    //        let (mut _tx, mut rx) = sock.split();

    //        spawn_local(async move {
    //            while let Some(msg) = rx.next().await {
    //                log::info!("{msg:?}");
    //                //tx.send(msg).await.unwrap();
    //            }
    //            log::info!("ws closed");
    //        })
    //    },
    //    (),
    //);
    html! {
        <div>{"server"}</div>
    }
}
