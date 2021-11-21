use wasm_bindgen::JsCast;
use yew::{
    html,
    utils::document,
    web_sys::HtmlDivElement,
    Component, ComponentLink, Html, Properties, ShouldRender,
};
use super::info_block::InfoBlockComponent;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
    pub name: String,
}

pub struct CountryInfoComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for CountryInfoComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CountryInfoComponent { props, link }
    }

    fn view(&self) -> Html {
        html! {
            <div id="country_view_desc">
                <h1 class="country_info_caption">{self.props.name.clone()}</h1>
                <InfoBlockComponent id={self.props.id.clone()} />
                <InfoBlockComponent id={self.props.id.clone()} />
                <InfoBlockComponent id={self.props.id.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
