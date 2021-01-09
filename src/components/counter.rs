use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Counter {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub children: Children,
}

impl Component for Counter {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div style="color: blue;">
                {self.props.children.clone()}
            </div>
        }
    }
}
