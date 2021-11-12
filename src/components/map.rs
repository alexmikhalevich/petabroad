use yew::{html, virtual_dom::VNode, Component, ComponentLink, Html, Properties, ShouldRender};

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

impl Component for MapComponent {
    type Properties = ();
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        build_countries_list().iter().map(|c| panic!("{}", format!("{:?}", c.render())));
        MapComponent { link }
    }

    fn view(&self) -> Html {
        html! {
            <svg baseprofile="tiny" fill="#ececec" height="857" stroke="black" stroke-linecap="round"
                 stroke-linejoin="round" stroke-width=".2" version="1.2" viewbox="0 0 1500 857"
                 width="1500" xmlns="http://www.w3.org/2000/svg">
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
