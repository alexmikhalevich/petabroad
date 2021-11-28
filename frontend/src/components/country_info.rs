use serde_json::from_str;
use yew::{
    format::Nothing,
    html,
    services::fetch::{FetchService, FetchTask, Request, Response, StatusCode},
    Component, ComponentLink, Html, Properties, ShouldRender,
};

use super::info_block::{BlockType, InfoBlockComponent};
use crate::settings::{BACKEND_ADDRESS, CAPTION_CLASS, COUNTRY_INFO_CAPTION, COUNTRY_INFO_ID};
use crate::utils::country_data::CountryData;

pub enum Msg {
    ReceiveData(Result<String, anyhow::Error>),
    ReceiveError(StatusCode),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub id: String,
    pub name: String,
}

pub struct CountryInfoComponent {
    props: Props,
    link: ComponentLink<Self>,
    data: Option<CountryData>,
    fetch_task: Option<FetchTask>,
}

impl CountryInfoComponent {
    fn request_data(&mut self) {
        let request = Request::get(format!(
            "http://{}/country/{}",
            BACKEND_ADDRESS,
            self.props.id.clone()
        ))
        .body(Nothing)
        .expect("Unable to build request");
        let callback = self
            .link
            .callback(|response: Response<Result<String, anyhow::Error>>| {
                if response.status() == StatusCode::OK {
                    Msg::ReceiveData(response.into_body())
                } else {
                    Msg::ReceiveError(response.status())
                }
            });
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        self.fetch_task = Some(task);
    }
}

impl Component for CountryInfoComponent {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CountryInfoComponent {
            props,
            link,
            fetch_task: None,
            data: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id={COUNTRY_INFO_ID.clone()}>
                <h1 class={format!("{} {}", CAPTION_CLASS, COUNTRY_INFO_CAPTION)}>{self.props.name.clone()}</h1>
                <InfoBlockComponent id={self.props.id.clone()} data={self.data.clone()} block_type={BlockType::Vaccine} />
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReceiveData(response) => {
                self.data = match response {
                    Ok(val) => Some(
                        from_str(&val)
                            .unwrap_or_else(|e| panic!("Unable to deserialize CountryData: {}", e)),
                    ),
                    Err(_) => None,
                };
                true
            }
            Msg::ReceiveError(_status_code) => {
                self.data = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            self.request_data();
            true
        } else {
            false
        }
    }
}
