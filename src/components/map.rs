use wasm_bindgen::JsCast;
use yew::{
    html, web_sys::Document, web_sys::SvgAnimatedLength, web_sys::SvgElement,
    web_sys::SvgsvgElement, Component, ComponentLink, Html, MouseEvent, ShouldRender,
};

use super::map_data::build_countries_list;

pub struct Country {
    pub id: String,
    pub name: String,
    pub path: String,
}

impl Country {
    fn render(&self) -> Html {
        html! {
            <path class="country" id={self.id.clone()} name={self.name.clone()} d={self.path.clone()}></path>
        }
    }
}

pub struct MapComponent {
    link: ComponentLink<Self>,
}

impl MapComponent {
    fn drag(e: MouseEvent) {
        if e.buttons() == 1 {
            let map_svg = web_sys::window()
                .expect("Global window does not exist")
                .document()
                .expect("Global document does not exist")
                .get_element_by_id("map")
                .expect("Element with id `map` not present")
                .unchecked_into::<SvgElement>();
            let mut view_box_vec: Vec<i32> = map_svg
                .get_attribute("viewBox")
                .expect("Unable to get `viewBox` attribute from the map")
                .split_whitespace()
                .map(|e| {
                    e.parse::<i32>()
                        .expect("Unable to convert viewBox attribute to numbers vector")
                })
                .collect();
            view_box_vec[0] -= e.movement_x();
            view_box_vec[1] -= e.movement_y();
            let view_box_attr: String = view_box_vec
                .into_iter()
                .map(|e| format!("{} ", e.to_string()))
                .collect::<String>()
                .trim()
                .to_string();
            map_svg
                .set_attribute("viewBox", &view_box_attr)
                .expect("Unable to set `viewBox` attribute for the map");
        }
    }
}

impl Component for MapComponent {
    type Properties = ();
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MapComponent { link }
    }

    fn view(&self) -> Html {
        let ondrag = self.link.callback(|e: MouseEvent| MapComponent::drag(e));
        html! {
            <svg baseprofile="tiny" fill="#ececec" stroke="black" viewBox="0 0 1500 1500"
                 width="100%" height="100%" stroke-linecap="round" stroke-linejoin="round"
                 stroke-width=".2" version="1.2" xmlns="http://www.w3.org/2000/svg"
                 style="border: 1px solid black" onmousemove={ondrag} id="map">
                 { for build_countries_list().iter().map(|c| c.render()) }
            </svg>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }
}
