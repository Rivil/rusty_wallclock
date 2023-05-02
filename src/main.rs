use yew::prelude::*;


pub struct AsyncComponent {
    clock: Option<AttrValue>,
}
mod components;

pub enum Msg {
    UpdateClock(AttrValue),
}


#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button onclick={onclick}>{ "Increment" }</button>
            <p>{ *counter }</p>
        <clock/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
