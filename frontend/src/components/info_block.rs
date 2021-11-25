use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::log;
use crate::settings::{
    get_block_icon, COUNTRY_INFO_BLOCK_ICON_CLASS, COUNTRY_INFO_BLOCK_ICON_DIV_CLASS,
    COUNTRY_INFO_BLOCK_TEXT_DIV_CLASS, COUNTRY_INFO_DIV_CLASS,
};
use crate::utils::country_data::CountryData;

#[derive(PartialEq, Clone)]
pub enum BlockType {
    Vaccine,
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
    pub block_type: BlockType,
    pub data: Option<CountryData>,
}

pub struct InfoBlockComponent {
    props: Props,
    link: ComponentLink<Self>,
    data: Option<Html>,
}

impl InfoBlockComponent {
    fn parse_vaccine_data(&self, country_data: CountryData) -> Html {
        if country_data.vaccines == None {
            return html! { {"No vaccines data for this country" } };
        }
        html! {
            <ul>
            {
                for country_data.vaccines.unwrap().iter().map(|vaccines_data| {
                    html! { <li><b>{vaccines_data.name.clone()}</b>{": "}{vaccines_data.desc.clone()}</li> }
                })
            }
            </ul>
        }
    }

    fn parse_data(&self) -> Html {
        if self.props.data == None {
            return html! { {"No data for this country"} };
        }
        let country_data = self.props.data.clone().unwrap();
        match self.props.block_type {
            BlockType::Vaccine => self.parse_vaccine_data(country_data),
        }
    }
}

impl Component for InfoBlockComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InfoBlockComponent {
            props,
            link,
            data: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class={COUNTRY_INFO_DIV_CLASS.clone()}>
                <div class={COUNTRY_INFO_BLOCK_ICON_DIV_CLASS.clone()}>
                    <img src={get_block_icon(self.props.block_type.clone())}
                         class={COUNTRY_INFO_BLOCK_ICON_CLASS.clone()} />
                </div>
                <div class={COUNTRY_INFO_BLOCK_TEXT_DIV_CLASS.clone()}>
                    { self.data.clone().unwrap_or_else(|| html! { "No data for this country" }) }
                </div>
            </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            self.data = Some(self.parse_data());
            true
        } else {
            false
        }
    }
}
