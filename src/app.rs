use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <p class="text-indigo-600">{ "Hello World" }</p>
            <p class="text-sky-600">{ "Hello World" }</p>
        </div>
    }
}
