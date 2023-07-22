use crate::prelude::*;

const TEXT_COLOR: Color =
    Color::rgb(190.0 / 255.0, 220.0 / 255.0, 127.0 / 255.0);
const FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";

pub const NORMAL_BUTTON: Color =
    Color::rgb(17.0 / 255.0, 35.0 / 255.0, 24.0 / 255.0);
pub const HOVERED_BUTTON: Color =
    Color::rgb(48.0 / 255.0, 93.0 / 255.0, 66.0 / 255.0);
pub const PRESSED_BUTTON: Color =
    Color::rgb(77.0 / 255.0, 128.0 / 255.0, 97.0 / 255.0);

pub const BACKGROUND: Color =
    Color::rgb(4.0 / 255.0, 12.0 / 255.0, 6.0 / 255.0);

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style.flex_direction = FlexDirection::Column;
    style.row_gap = Val::Px(8.);
    style.column_gap = Val::Px(8.);
    style
};

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(80.);
    style.height = Val::Percent(65.);
    style.margin = UiRect::all(Val::Px(20.0));
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 80.0,
        color: TEXT_COLOR,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 40.0,
        color: TEXT_COLOR,
    }
}
