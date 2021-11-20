use wasm_bindgen::JsCast;
use yew::{
    html, utils::document, web_sys::HtmlDivElement, Component, ComponentLink, Html, Properties,
    ShouldRender,
};

use super::country::CountryComponent;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub path: String,
}

pub struct CountryViewComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl CountryViewComponent {
    fn toggle_visibility(&self, visible: bool) {
        document()
            .get_element_by_id("country_view")
            .and_then(|t| t.dyn_into::<HtmlDivElement>().ok())
            .map(|el| {
                el.set_hidden(!visible);
            });
    }
}

impl Component for CountryViewComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CountryViewComponent { props, link }
    }

    fn view(&self) -> Html {
        html! {
            <div id="country_view">
                <div id="country_view_country">
                    <svg baseprofile="tiny" fill="#ececec" stroke="red" viewBox="0 0 1000 1000"
                         stroke-linecap="round" stroke-linejoin="round" stroke-width=".2" version="1.2"
                         xmlns="http://www.w3.org/2000/svg" id="country_view_country_svg">
                         <CountryComponent id={self.props.id.clone()} name={self.props.name.clone()}
                                           path={self.props.path.clone()} />
                    </svg>
                </div>
                <div id="country_view_desc">
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.toggle_visibility(false);
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        /*
        document()
            .get_element_by_id("country_view_country_path")
            .and_then(|t| t.dyn_into::<SvgPathElement>().ok())
            .map(|el| {
                el.set_attribute(
                    "style",
                    format!(
                        "scale: {}, translate: ({}px, {}px)",
                        get_scale(),
                        get_translate_x(),
                        get_translate_y()
                    ),
                );
            });
        */
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            self.toggle_visibility(true);
            true
        } else {
            false
        }
    }
}
