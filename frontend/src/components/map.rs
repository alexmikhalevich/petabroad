use std::cmp::{max, min};
use wasm_bindgen::JsCast;
use yew::{
    html,
    utils::document,
    web_sys::{Node, SvgElement},
    Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender, WheelEvent,
};

use super::country::CountryComponent;
use super::map_data::{get_countries_borders, get_countries_names};
use crate::utils::viewbox::{Point, ViewBox};

const MAP_ZOOM_STEP: f32 = 0.05;
pub const MAP_ZOOM_MIN: u32 = 2000;
const MAP_ZOOM_MAX: u32 = 300;

pub enum Msg {
    CountryClick(String),
    CountryHover(SvgElement),
    Drag(MouseEvent),
    Scroll(WheelEvent),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub oncountryclick: Option<Callback<String>>,
    pub viewbox_width: u32,
    pub viewbox_height: u32,
}

pub struct MapComponent {
    props: Props,
    link: ComponentLink<Self>,
    map_html: Html,
    viewbox: ViewBox,
}

impl MapComponent {
    fn get_map_element(&self) -> SvgElement {
        document()
            .get_element_by_id("map")
            .expect("Element with id `map` not present")
            .unchecked_into::<SvgElement>()
    }
    fn build_map_html(link: &ComponentLink<Self>) -> Html {
        let oncountryclick = link.callback(|id: String| Msg::CountryClick(id));
        let oncountryhover = link.callback(|n: SvgElement| Msg::CountryHover(n));
        html! {
            {
                 for get_countries_names().iter().map(|(id, name)| {
                     let path = get_countries_borders()
                        .get(id)
                        .unwrap_or_else(|| panic!("Mismatch in countries list"))
                        .to_string();
                     html!{
                         <CountryComponent id={id.to_string()} name={name.to_string()} path={path}
                                           onclick={oncountryclick.clone()} onhover={oncountryhover.clone()}
                                           translate_x=0 translate_y=0 />
                     }
                 })
             }
        }
    }
}

impl Component for MapComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let map_html = MapComponent::build_map_html(&link);
        let w = props.viewbox_width.clone();
        let h = props.viewbox_height.clone();
        MapComponent {
            props,
            link,
            map_html,
            viewbox: ViewBox {
                top_left: Point { x: 0, y: 0 },
                w: w,
                h: h,
                zoom_in_limit: MAP_ZOOM_MAX,
                zoom_out_limit: MAP_ZOOM_MIN,
            },
        }
    }

    fn view(&self) -> Html {
        let ondrag = self.link.callback(|e: MouseEvent| Msg::Drag(e));
        let onscroll = self.link.callback(|e: WheelEvent| Msg::Scroll(e));
        html! {
            <svg baseprofile="tiny" viewBox={self.viewbox.to_string()} version="1.2" xmlns="http://www.w3.org/2000/svg"
                 onmousemove={ondrag} onwheel={onscroll} id="map">
                     { self.map_html.clone() }
            </svg>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Drag(e) => {
                if e.buttons() == 1 {
                    self.viewbox.drag(Point {
                        x: -e.movement_x() / 2,
                        y: -e.movement_y() / 2,
                    });
                    return true;
                }
                false
            }
            Msg::Scroll(e) => {
                if e.delta_y() > 0.0 {
                    self.viewbox.zoom_to_center(1.0 - MAP_ZOOM_STEP);
                } else if e.delta_y() < 0.0 {
                    self.viewbox.zoom_to_center(1.0 + MAP_ZOOM_STEP);
                }
                true
            }
            Msg::CountryClick(id) => {
                if self.props.oncountryclick != None {
                    self.props.oncountryclick.as_ref().unwrap().emit(id);
                }
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.viewbox.w = props.viewbox_width.clone();
            self.viewbox.h = props.viewbox_height.clone();
            self.props = props;
            return true;
        }
        false
    }
}
