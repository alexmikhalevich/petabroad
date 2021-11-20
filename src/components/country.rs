use wasm_bindgen::JsCast;
use yew::{
    html,
    web_sys::{EventTarget, SvgElement, SvgPathElement},
    Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender,
};

pub enum Msg {
    Click(MouseEvent),
    Highlight(MouseEvent),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub path: String,
    pub onclick: Option<Callback<String>>,
    pub onhover: Option<Callback<SvgElement>>,
}

pub struct CountryComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for CountryComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CountryComponent { props, link }
    }

    fn view(&self) -> Html {
        let onmouseenter = self.link.callback(|e: MouseEvent| Msg::Highlight(e));
        let onclick = self.link.callback(|e: MouseEvent| Msg::Click(e));
        html! {
            <path class="country" id={self.props.id.clone()} name={self.props.name.clone()} d={self.props.path.clone()}
                  onmouseenter={onmouseenter} onclick={onclick}>
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
            Msg::Highlight(e) => {
                let target = e
                    .target()
                    .expect("Unable to get EventTarget in CountryComponent update")
                    .dyn_into::<SvgElement>()
                    .expect("Unable to convert target country SVG to SvgElement");
                self.props.onhover.as_ref().unwrap().emit(target);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
