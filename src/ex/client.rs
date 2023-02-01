use yew::prelude::*;

#[function_component(Client)]
pub fn client() -> Html {
    let on_file_select = Callback::from(|_| {
        log::info!("test");
    });
    html! {
        <>
            <style>{"label, span { display: inline-block; width: 128px; }"}</style>
            <form action="" method="get">
                <div>
                    <label>{"string"}</label>
                    <input type="text" name="string" value="( . _ .)" required=true/>
                </div>
                <div>
                    <label>{"int"}</label>
                    <input type="number" name="int" value="66" min="1" max="123" required=true/>
                </div>
                <div>
                    <label>{"float"}</label>
                    <input type="number" name="float" value="3.14159" step="any" required=true/>
                </div>
                <div>
                    <span/>
                    <label class="file-select" style="width: auto">
                        {"[choose file]"}
                        <input onchange={on_file_select} type="file" style="display: none"/>
                    </label>
                </div>
                <div>
                    <button>{"[send]"}</button>
                </div>
            </form>
        </>
    }
}
