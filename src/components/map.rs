use yew::{Component, ComponentLink, Html, ShouldRender, Properties, virtual_dom::VNode};
use web_sys::Node;
use mapex::map::{Map, MapexComponent};

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub map: Map,
}

pub struct MapComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for MapComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MapComponent { props, link }
    }

    fn view(&self) -> Html {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&self.props.map.html()[..]);

        let node = Node::from(div);
        let vnode = VNode::VRef(node);
        vnode
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
