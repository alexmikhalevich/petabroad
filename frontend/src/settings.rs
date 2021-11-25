use crate::components::info_block::BlockType;

pub const TOP_SPACER_ID: &'static str = "top_spacer";
pub const COUNTRY_COMPONENT_CLASS: &'static str = "country";
pub const COUNTRY_INFO_ID: &'static str = "country_view_desc";
pub const COUNTRY_VIEW_ID: &'static str = "country_view";
pub const COUNTRY_VIEW_PATH_ID: &'static str = "country_view_country_path";
pub const COUNTRY_VIEW_SUBDIV_ID: &'static str = "country_view_country";
pub const COUNTRY_VIEW_SVG_ID: &'static str = "country_view_country_svg";
pub const COUNTRY_VIEW_CLOSE_BUTTON_ID: &'static str = "country_view_close_button";
pub const CAPTION_CLASS: &'static str = "caption";
pub const COUNTRY_INFO_CAPTION: &'static str = "country_info_caption";
pub const COUNTRY_INFO_DIV_CLASS: &'static str = "country_info_block";
pub const COUNTRY_INFO_BLOCK_ICON_DIV_CLASS: &'static str = "country_info_block_icon";
pub const COUNTRY_INFO_BLOCK_ICON_CLASS: &'static str = "country_info_block_icon";
pub const COUNTRY_INFO_BLOCK_TEXT_DIV_CLASS: &'static str = "country_info_block_text";
pub const MAIN_CAPTION_ID: &'static str = "main_caption";
pub const MENU_BUTTON_ID: &'static str = "burger_button";
pub const MAP_ID: &'static str = "map";

pub const COUNTRY_VIEW_SCALE: f32 = 0.7;
pub const MAP_ZOOM_STEP: f32 = 0.05;
pub const MAP_ZOOM_MIN: u32 = 2000;
pub const MAP_ZOOM_MAX: u32 = 300;

pub const BACKEND_ADDRESS: &'static str = "localhost:8081";

pub fn get_block_icon(block_type: BlockType) -> String {
    match block_type {
        BlockType::Vaccine => "http://cdn.onlinewebfonts.com/svg/img_319799.png".to_string(),
    }
}
