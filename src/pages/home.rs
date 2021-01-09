use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::components::counter::Counter;
use crate::app_router::{AppRoute, Link};

pub struct Home {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
    SubtractOne,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::SubtractOne => self.value -= 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::SubtractOne)>{ "-1" }</button>
                <Counter>
                    <p>{ self.value }</p>
                </Counter>
                <Link route=AppRoute::OrderList>{"Order List"}</Link>
                <Link route=AppRoute::Order(self.value)>{"Order #"}{ self.value }</Link>
            </div>
        }
    }
}
