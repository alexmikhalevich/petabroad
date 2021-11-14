use wasm_bindgen::JsCast;
use yew::{
    html, web_sys::Document, web_sys::SvgAnimatedLength, web_sys::SvgElement,
    web_sys::SvgsvgElement, Component, ComponentLink, Html, MouseEvent, ShouldRender, WheelEvent,
};
use std::cmp::{max, min};

use super::map_data::build_countries_list;

const MAP_ZOOM_STEP: i32 = 30;
const MAP_ZOOM_MIN: i32 = 500;
const MAP_ZOOM_MAX: i32 = 1500;

fn vec_to_string(v: Vec<i32>) -> String {
    v.into_iter()
        .map(|e| format!("{} ", e.to_string()))
        .collect::<String>()
        .trim()
        .to_string()
}

fn string_to_vec(s: String) -> Vec<i32> {
    s
        .split_whitespace()
        .map(|e| {
            e.parse::<i32>()
                .unwrap_or_else(|_| panic!("Unable to convert `{}` attribute to numbers vector", s))
        })
        .collect()
}

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
    fn get_map_element() -> SvgElement {
        web_sys::window()
            .expect("Global window does not exist")
            .document()
            .expect("Global document does not exist")
            .get_element_by_id("map")
            .expect("Element with id `map` not present")
            .unchecked_into::<SvgElement>()
    }
    fn set_map_attribute(map: &SvgElement, name: &str, value: &str) {
        map.set_attribute(name, value)
            .unwrap_or_else(|_| panic!("Unable to set `{}` attribute for the map", name));
    }
    fn get_map_attribute(map: &SvgElement, name: &str) -> String {
        map.get_attribute(name)
            .unwrap_or_else(|| panic!("Unable to get `{}` attribute for the map", name))
    }
    fn drag(e: MouseEvent) {
        if e.buttons() == 1 {
            let map_svg = MapComponent::get_map_element();
            let mut view_box_vec = string_to_vec(MapComponent::get_map_attribute(&map_svg, "viewBox"));
            view_box_vec[0] -= e.movement_x();
            view_box_vec[1] -= e.movement_y();
            MapComponent::set_map_attribute(&map_svg, "viewBox", &vec_to_string(view_box_vec));
        }
    }
    fn scroll(e: WheelEvent) {
        let map_svg = MapComponent::get_map_element();
        let mut view_box_vec = string_to_vec(MapComponent::get_map_attribute(&map_svg, "viewBox"));
        if e.delta_y() > 0.0 {
            view_box_vec[2] = max(MAP_ZOOM_MIN, view_box_vec[2] - MAP_ZOOM_STEP);
            view_box_vec[3] = max(MAP_ZOOM_MIN, view_box_vec[3] - MAP_ZOOM_STEP);
        } else if e.delta_y() < 0.0 {
            view_box_vec[2] = min(MAP_ZOOM_MAX, view_box_vec[2] + MAP_ZOOM_STEP);
            view_box_vec[3] = min(MAP_ZOOM_MAX, view_box_vec[3] + MAP_ZOOM_STEP);
        }
        MapComponent::set_map_attribute(&map_svg, "viewBox", &vec_to_string(view_box_vec));
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
        let onscroll = self.link.callback(|e: WheelEvent| MapComponent::scroll(e));
        html! {
            <svg baseprofile="tiny" fill="#ececec" stroke="black" viewBox="0 0 1500 1500"
                 width="100%" height="100%" stroke-linecap="round" stroke-linejoin="round"
                 stroke-width=".2" version="1.2" xmlns="http://www.w3.org/2000/svg"
                 style="border: 1px solid black" onmousemove={ondrag} onwheel={onscroll} id="map">
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
