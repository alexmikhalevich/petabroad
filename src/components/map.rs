use std::cmp::{max, min};
use wasm_bindgen::JsCast;
use yew::{
    html, web_sys::Document, web_sys::Node, web_sys::SvgAnimatedLength, web_sys::SvgElement,
    web_sys::SvgPathElement, web_sys::SvgsvgElement, Callback, Component, ComponentLink, Html,
    MouseEvent, ShouldRender, WheelEvent,
};

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
    s.split_whitespace()
        .map(|e| {
            e.parse::<i32>()
                .unwrap_or_else(|_| panic!("Unable to convert `{}` attribute to numbers vector", s))
        })
        .collect()
}

fn set_svg_attribute(svg: &SvgElement, name: &str, value: &str) {
    svg.set_attribute(name, value)
        .unwrap_or_else(|_| panic!("Unable to set `{}` attribute for the {:?}", name, svg));
}

fn get_svg_attribute(svg: &SvgElement, name: &str) -> String {
    svg.get_attribute(name)
        .unwrap_or_else(|| panic!("Unable to get `{}` attribute for the {:?}", name, svg))
}

fn get_document() -> Document {
    web_sys::window()
        .expect("Global window does not exist")
        .document()
        .expect("Global document does not exist")
}

pub struct Country {
    pub id: String,
    pub name: String,
    pub path: String,
}

impl Country {
    fn toggle_highlight(e: MouseEvent, enable: bool) {
        let style = if enable { "#fcecec" } else { "#ececec" };
        let filter = if enable { "country shadow" } else { "country" };
        // set props to highlight countries
        e.target()
            .and_then(|t| t.dyn_into::<SvgElement>().ok())
            .map(|el| {
                el.set_attribute("fill", style)
                    .expect("Unable to set style attribute for the country path");
                el.set_attribute("class", filter)
                    .expect("Unable to set filter attribute for the country path");
            });
        // move selected country to the DOM top to make shades render properly
        let target_node = e
            .target()
            .expect("Unable to get EventTarget")
            .dyn_into::<Node>()
            .expect("Unable to convert target country SVG to Node");
        let map_node = MapComponent::get_map_element()
            .dyn_into::<Node>()
            .expect("Unable to convert root map SVG to Node");
        map_node
            .remove_child(&target_node)
            .expect("Unable to remove selected SVG Node");
        map_node
            .append_child(&target_node)
            .expect("Unable to re-append selected SVG Node");
    }

    fn render(&self) -> Html {
        let onmouseenter = Callback::from(|e: MouseEvent| Country::toggle_highlight(e, true));
        let onmouseleave = Callback::from(|e: MouseEvent| Country::toggle_highlight(e, false));
        html! {
            <path class="country" id={self.id.clone()} name={self.name.clone()} d={self.path.clone()}
                  onmouseenter={onmouseenter} onmouseleave={onmouseleave}>
            </path>
        }
    }
}

pub struct MapComponent {
    link: ComponentLink<Self>,
}

impl MapComponent {
    fn get_map_element() -> SvgElement {
        get_document()
            .get_element_by_id("map")
            .expect("Element with id `map` not present")
            .unchecked_into::<SvgElement>()
    }
    fn drag(e: MouseEvent) {
        if e.buttons() == 1 {
            let map_svg = MapComponent::get_map_element();
            let mut view_box_vec = string_to_vec(get_svg_attribute(&map_svg, "viewBox"));
            view_box_vec[0] -= e.movement_x();
            view_box_vec[1] -= e.movement_y();
            set_svg_attribute(&map_svg, "viewBox", &vec_to_string(view_box_vec));
        }
    }
    fn scroll(e: WheelEvent) {
        let map_svg = MapComponent::get_map_element();
        let mut view_box_vec = string_to_vec(get_svg_attribute(&map_svg, "viewBox"));
        if e.delta_y() > 0.0 {
            view_box_vec[2] = max(MAP_ZOOM_MIN, view_box_vec[2] - MAP_ZOOM_STEP);
            view_box_vec[3] = max(MAP_ZOOM_MIN, view_box_vec[3] - MAP_ZOOM_STEP);
        } else if e.delta_y() < 0.0 {
            view_box_vec[2] = min(MAP_ZOOM_MAX, view_box_vec[2] + MAP_ZOOM_STEP);
            view_box_vec[3] = min(MAP_ZOOM_MAX, view_box_vec[3] + MAP_ZOOM_STEP);
        }
        set_svg_attribute(&map_svg, "viewBox", &vec_to_string(view_box_vec));
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
                 style="border: 1px solid red" onmousemove={ondrag} onwheel={onscroll} id="map">
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
