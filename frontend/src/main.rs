mod components;
mod settings;
mod utils;

use components::main_window::MainWindowComponent;

fn main() {
    yew::start_app::<MainWindowComponent>();
}
