use super::country_view::CountryViewComponent;
use super::map::MapComponent;
use crate::utils::map_data::{get_countries_borders, get_countries_names};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::settings::{
    CAPTION_CLASS, MAIN_CAPTION_ID, MAP_ZOOM_MIN, MENU_BUTTON_ID, TOP_SPACER_ID,
};

pub enum Msg {
    CountryClick(String),
}

pub struct MainWindowComponent {
    link: ComponentLink<Self>,
    country_view_id: String,
    country_view_name: String,
    country_view_path: String,
}

impl Component for MainWindowComponent {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MainWindowComponent {
            link,
            country_view_name: "".to_string(),
            country_view_path: "".to_string(),
            country_view_id: "".to_string(),
        }
    }

    fn view(&self) -> Html {
        let oncountryclick = self.link.callback(|id: String| Msg::CountryClick(id));
        html! {
            <>
                <div id={TOP_SPACER_ID.clone()}>
                    <h1 class={CAPTION_CLASS.clone()} id={MAIN_CAPTION_ID.clone()}>{"petabroad.io"}</h1>
                    <button id={MENU_BUTTON_ID.clone()}>
                        <i class="fa fa-bars" />
                    </button>
                </div>
                <MapComponent oncountryclick={oncountryclick} viewbox_width={MAP_ZOOM_MIN}
                              viewbox_height={MAP_ZOOM_MIN} />
                <CountryViewComponent name={self.country_view_name.clone()}
                                      path={self.country_view_path.clone()}
                                      id={self.country_view_id.clone()} />
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CountryClick(id) => {
                self.country_view_name = get_countries_names()
                    .get(&id as &str)
                    .unwrap_or_else(|| panic!("Mismatch in countries list"))
                    .to_string();
                self.country_view_path = get_countries_borders()
                    .get(&id as &str)
                    .unwrap_or_else(|| panic!("Mismatch in countries list"))
                    .to_string();
                self.country_view_id = id;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}
