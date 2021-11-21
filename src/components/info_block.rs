use wasm_bindgen::JsCast;
use yew::{
    html,
    utils::document,
    web_sys::HtmlDivElement,
    Component, ComponentLink, Html, Properties, ShouldRender,
};

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
}

pub struct InfoBlockComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for InfoBlockComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InfoBlockComponent { props, link }
    }

    fn view(&self) -> Html {
        html! {
            <div class="country_info_block">
                <div class ="country_info_block_icon">
                    <img src="http://cdn.onlinewebfonts.com/svg/img_319799.png" class ="country_info_block_icon" />
                </div>
                <div class="country_info_block_text">
                {
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
                    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam."
                }
                </div>
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
