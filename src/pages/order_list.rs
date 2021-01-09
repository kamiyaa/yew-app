use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct OrderList {
    link: ComponentLink<Self>,
}

impl Component for OrderList {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{ "Order List" }</h2>
                <ul>
                    <li>{"Order #1"}</li>
                    <li>{"Order #2"}</li>
                </ul>
            </div>
        }
    }
}
