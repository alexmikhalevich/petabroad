use super::country::CountryComponent;
use super::country_info::CountryInfoComponent;
use crate::utils::viewbox::{Point, ViewBox};
use wasm_bindgen::JsCast;
use yew::{
    html,
    utils::document,
    web_sys::{HtmlDivElement, SvgGraphicsElement, SvgRect},
    Component, ComponentLink, Html, Properties, ShouldRender,
};

const COUNTRY_VIEW_SCALE: f32 = 0.7;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub path: String,
}

pub struct CountryViewComponent {
    props: Props,
    link: ComponentLink<Self>,
    country_translate_x: i32,
    country_translate_y: i32,
    transformed: bool,
    view_box: ViewBox,
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

    fn set_viewbox(&mut self, svg_bbox: &SvgRect, country_view: &HtmlDivElement) {
        let top_left_point = Point {
            x: country_view.offset_width() / 2,
            y: country_view.offset_height() / 2,
        } - Point {
            x: (svg_bbox.width() / 2.0) as i32,
            y: (svg_bbox.height() / 2.0) as i32,
        };
        self.view_box = ViewBox {
            top_left: top_left_point,
            w: svg_bbox.width() as u32,
            h: svg_bbox.height() as u32,
            zoom_in_limit: 0,
            zoom_out_limit: 0,
        };
        self.view_box.zoom_to_center(COUNTRY_VIEW_SCALE);
    }

    fn set_translate_x(&mut self, svg_bbox: &SvgRect, country_view: &HtmlDivElement) {
        let desired_x = (country_view.offset_width() as f32 - svg_bbox.width()) / 2.0;
        let current_x = svg_bbox.x();
        self.country_translate_x = (desired_x - current_x) as i32;
    }

    fn set_translate_y(&mut self, svg_bbox: &SvgRect, country_view: &HtmlDivElement) {
        let desired_y = (country_view.offset_height() as f32 - svg_bbox.height()) / 2.0;
        let current_y = svg_bbox.y();
        self.country_translate_y = (desired_y - current_y) as i32;
    }

    fn transform(&mut self) {
        let svg_bbox = document()
            .get_element_by_id("country_view_country_path")
            .expect("Unable to get element with id `country_view_country_path`")
            .dyn_into::<SvgGraphicsElement>()
            .expect("Unable to cast country path element to SvgGraphicsElement")
            .get_b_box()
            .ok()
            .unwrap();
        let country_view = document()
            .get_element_by_id("country_view_country")
            .expect("Unable to get element with id `country_view_country`")
            .dyn_into::<HtmlDivElement>()
            .expect("Unable to cast country view div to HtmlDivElement");
        self.set_translate_x(&svg_bbox, &country_view);
        self.set_translate_y(&svg_bbox, &country_view);
        self.set_viewbox(&svg_bbox, &country_view);
        self.transformed = true;
    }
}

impl Component for CountryViewComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CountryViewComponent {
            props,
            link,
            country_translate_x: 0,
            country_translate_y: 0,
            transformed: false,
            view_box: ViewBox {
                top_left: Point { x: 0, y: 0 },
                w: 0,
                h: 0,
                zoom_in_limit: 0,
                zoom_out_limit: 0,
            },
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="country_view">
                <div id="country_view_country">
                    <svg baseprofile="tiny" viewBox={self.view_box.to_string()} version="1.2"
                         xmlns="http://www.w3.org/2000/svg" id="country_view_country_svg">
                         <CountryComponent id="country_view_country_path" name={self.props.name.clone()}
                                           path={self.props.path.clone()}
                                           translate_x={self.country_translate_x.clone()}
                                           translate_y={self.country_translate_y.clone()} />
                    </svg>
                </div>
                <CountryInfoComponent id={self.props.id.clone()} name={self.props.name.clone()} />
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.toggle_visibility(false);
        } else if !self.transformed {
            self.transform();
            self.link.send_message(());
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            self.toggle_visibility(true);
            self.transformed = false;
            true
        } else {
            false
        }
    }
}
