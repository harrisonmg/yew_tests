use yew::prelude::*;

#[function_component(Client)]
pub fn client() -> Html {
    html! {
        <>
        <style>{ "label { display: inline-block; width: 50px; }" }</style>
        <form action="" method="get">
            <div>
                <label>{ "string" }</label>
                    <input type="text" name="string" value="( . _ .)"/>
            </div>
            <div>
                <label>{ "int" }</label>
                <input type="number" name="int" value="8675209"/>
            </div>
            <div>
                <label>{ "float" }</label>
                <input type="number" name="float" value="3.14159"/>
            </div>
            <div>
                //<input type="sumbit" value="send"/>
                <button>{ "send" }</button>
            </div>
        </form>
        </>
    }
}

#[function_component(Server)]
pub fn server() -> Html {
    html! {
        <div>{ "server" }</div>
    }
}
