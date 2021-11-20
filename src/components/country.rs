use wasm_bindgen::JsCast;
use yew::{
    html,
    web_sys::{EventTarget, SvgElement, SvgPathElement},
    Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender,
};

pub enum Msg {
    Click(MouseEvent),
    HighlightOn(MouseEvent),
    HighlightOff(MouseEvent),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub path: String,
    pub onclick: Option<Callback<String>>,
    pub onhover: Option<Callback<EventTarget>>,
}

pub struct CountryComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl CountryComponent {
    fn update_props(&self, target: EventTarget, color: &str, class: &str) {
        // set props to highlight countries
        let svg_el = target
            .dyn_into::<SvgElement>()
            .expect("Unable to convert target country SVG to SvgElement");
        svg_el
            .set_attribute("fill", color)
            .expect("Unable to set style attribute for the country path");
        svg_el
            .set_attribute("class", class)
            .expect("Unable to set filter attribute for the country path");
    }
}

impl Component for CountryComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CountryComponent { props, link }
    }

    fn view(&self) -> Html {
        let onmouseenter = self.link.callback(|e: MouseEvent| Msg::HighlightOn(e));
        let onmouseleave = self.link.callback(|e: MouseEvent| Msg::HighlightOff(e));
        let onclick = self.link.callback(|e: MouseEvent| Msg::Click(e));
        html! {
            <path class="country" id={self.props.id.clone()} name={self.props.name.clone()} d={self.props.path.clone()}
                  onmouseenter={onmouseenter} onmouseleave={onmouseleave} onclick={onclick}>
            </path>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(e) => {
                let country_id = e
                    .target()
                    .expect("Unable to get click event target")
                    .dyn_into::<SvgPathElement>()
                    .expect("Unable to convert target country element to SvgElement")
                    .id()
                    .to_string();
                self.props.onclick.as_ref().unwrap().emit(country_id);
                false
            }
            Msg::HighlightOn(e) => {
                let target = e
                    .target()
                    .expect("Unable to get EventTarget in CountryComponent update");
                self.update_props(target.clone(), "#fcecec", "country shadow");
                self.props.onhover.as_ref().unwrap().emit(target);
                true
            }
            Msg::HighlightOff(e) => {
                let target = e
                    .target()
                    .expect("Unable to get EventTarget in CountryComponent update");
                self.update_props(target, "#ececec", "country");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
