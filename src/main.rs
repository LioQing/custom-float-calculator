pub use yew::prelude::*;
pub use float_format::*;
pub use regex::Regex;

mod components;
pub use components::*;

mod utils;
pub use utils::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}