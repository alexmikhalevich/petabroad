use std::{
    cmp::{max, min},
};
use wasm_bindgen::JsCast;
use yew::{
    html,
    utils::document,
    web_sys::{EventTarget, Node, SvgElement},
    Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender, WheelEvent,
};

use super::country::CountryComponent;
use super::map_data::{get_countries_borders, get_countries_names};

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

pub enum Msg {
    CountryClick(String),
    CountryHover(SvgElement),
    Drag(MouseEvent),
    Scroll(WheelEvent),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub oncountryclick: Option<Callback<String>>,
}

pub struct MapComponent {
    props: Props,
    link: ComponentLink<Self>,
    map_html: Html,
}

impl MapComponent {
    fn get_map_element(&self) -> SvgElement {
        document()
            .get_element_by_id("map")
            .expect("Element with id `map` not present")
            .unchecked_into::<SvgElement>()
    }
    fn drag(&self, e: MouseEvent) {
        let map_svg = self.get_map_element();
        let mut view_box_vec = string_to_vec(get_svg_attribute(&map_svg, "viewBox"));
        view_box_vec[0] -= e.movement_x();
        view_box_vec[1] -= e.movement_y();
        set_svg_attribute(&map_svg, "viewBox", &vec_to_string(view_box_vec));
    }
    fn scroll(&self, e: WheelEvent) {
        let map_svg = self.get_map_element();
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
    fn build_map_html(link: &ComponentLink<Self>) -> Html {
        let oncountryclick = link.callback(|id: String| Msg::CountryClick(id));
        let oncountryhover = link.callback(|n: SvgElement| Msg::CountryHover(n));
        let ondrag = link.callback(|e: MouseEvent| Msg::Drag(e));
        let onscroll = link.callback(|e: WheelEvent| Msg::Scroll(e));
        html! {
            <svg baseprofile="tiny" fill="#ececec" stroke="black" viewBox="0 0 1500 1500"
                 width="100%" height="100%" stroke-linecap="round" stroke-linejoin="round"
                 stroke-width=".2" version="1.2" xmlns="http://www.w3.org/2000/svg"
                 onmousemove={ondrag} onwheel={onscroll} id="map">
                 {
                     for get_countries_names().iter().map(|(id, name)| {
                         let path = get_countries_borders()
                            .get(id)
                            .unwrap_or_else(|| panic!("Mismatch in countries list"))
                            .to_string();
                         html!{
                             <CountryComponent id={id.to_string()} name={name.to_string()} path={path}
                                               onclick={oncountryclick.clone()} onhover={oncountryhover.clone()} />
                         }
                     })
                 }
            </svg>
        }
    }
}

impl Component for MapComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let map_html = MapComponent::build_map_html(&link);
        MapComponent {
            props,
            link,
            map_html,
        }
    }

    fn view(&self) -> Html {
        self.map_html.clone()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Drag(e) => {
                if e.buttons() == 1 {
                    self.drag(e);
                    return true;
                }
                false
            }
            Msg::Scroll(e) => {
                self.scroll(e);
                true
            }
            Msg::CountryClick(id) => {
                self.props.oncountryclick.as_ref().unwrap().emit(id);
                false
            }
            Msg::CountryHover(n) => {
                // move selected country to the DOM top to make shades render properly
                let target_node = n
                    .dyn_into::<Node>()
                    .expect("Unable to convert target country SVG to Node");
                let map_node = self
                    .get_map_element()
                    .dyn_into::<Node>()
                    .expect("Unable to convert root map SVG to Node");
                map_node
                    .remove_child(&target_node)
                    .expect("Unable to remove selected SVG Node");
                map_node
                    .append_child(&target_node)
                    .expect("Unable to re-append selected SVG Node");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
