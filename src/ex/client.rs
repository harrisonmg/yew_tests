use web_sys::{FormData, HtmlFormElement, HtmlInputElement};
use yew::prelude::*;

#[function_component(Client)]
pub fn client() -> Html {
    let file = use_state_eq(|| None);

    let file_clone = file.clone();
    let on_file_select = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Some(file_list) = input.files() {
            if file_list.length() > 0 {
                if let Some(file) = file_list.get(0) {
                    file_clone.set(Some(file));
                }
            }
        }
    });

    let filename = file.as_ref().map_or(String::new(), |file| file.name());

    //let file_clone = file.clone();
    let on_submit = Callback::from(move |e: SubmitEvent| {
        let form: HtmlFormElement = e.target_unchecked_into();
        let data = FormData::new_with_form(&form).unwrap();
        log::info!("{}", data.to_string());
        // prints but then leaves page from submission
    });

    html! {
        <>
            <style>{"label { display: inline-block; width: 128px; }"}</style>
            <form action="" method="get" onsubmit={on_submit}>
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
                    <span style="width: 128px"/>
                    <label class="file-select" style="width: auto">
                       {"[choose image]"}
                       <input onchange={on_file_select} type="file" accept="image/*" style="display: none" required=true/>
                    </label>
                    <span>{" "}</span>
                    <span>{filename}</span>
                </div>
                <div>
                    <button type="submit" formaction="">{"[send]"}</button>
                </div>
            </form>
        </>
    }
}
