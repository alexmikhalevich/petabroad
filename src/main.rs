mod components;

use components::map::{MapComponent, Props};
use mapex::map::Map;

fn main() {
    let map = Map::create(800, 600, "https://a.tile.openstreetmap.org/{z}/{x}/{y}.png");
    yew::start_app_with_props::<MapComponent>(Props{map});
}
