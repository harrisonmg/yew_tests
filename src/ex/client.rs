use web_sys::{File, FormData, HtmlFormElement, HtmlInputElement};
use yew::prelude::*;

#[function_component(Client)]
pub fn client() -> Html {
    let file = use_state_eq::<Option<File>, _>(|| None);
    let filename = file.as_ref().map_or(String::new(), |file| file.name());

    let file_clone = file.clone();
    let on_file_input = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Some(file_list) = input.files() {
            if file_list.length() > 0 {
                file_clone.set(file_list.get(0));
            }
        }
    });

    //let file_clone = file.clone();
    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let form: HtmlFormElement = e.target_unchecked_into();
        let data = FormData::new_with_form(&form).unwrap();
        let string = data.get("string").as_string().unwrap();
        let int: u32 = data.get("int").as_string().unwrap().parse().unwrap();
        let float: f32 = data.get("float").as_string().unwrap().parse().unwrap();
        log::info!("{string}, {int}, {float}")
    });

    html! {
        <>
            <style>{"label { display: inline-block; width: 128px; }"}</style>
            <form method="get" onsubmit={on_submit}>
                <div>
                    <label>{"string"}</label>
                    <input
                        type="text"
                        name="string"
                        value="( . _ .)"
                        required=true
                    />
                </div>
                <div>
                    <label>{"int"}</label>
                    <input
                        type="number"
                        name="int"
                        value="66"
                        min="1"
                        max="123"
                        required=true
                    />
                </div>
                <div>
                    <label>{"float"}</label>
                    <input
                        type="number"
                        name="float"
                        value="3.14159"
                        step="any"
                        required=true
                    />
                </div>
                <div>
                    <span style="width: 128px"/>
                    <label for="image-input" class="file-select" style="width: auto">
                       {"[choose image]"}
                    </label>
                    <input
                        onchange={on_file_input}
                        id="image-input"
                        type="file"
                        accept="image/*"
                        style="width: 1px; height: 1px"
                        //required=true
                    />
                    <span>{filename}</span>
                </div>
                <div>
                    <button>{"[send]"}</button>
                </div>
            </form>
        </>
    }
}
