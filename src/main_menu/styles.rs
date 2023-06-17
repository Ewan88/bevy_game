use crate::prelude::*;

// DARK_GREEN_1: Color = Color::rgb_u8(4, 12, 6);
// DARK_GREEN_2: Color = Color::rgb_u8(17, 35, 24);
// DARK_GREEN_3: Color = Color::rgb_u8(30, 58, 41);
// DARK_GREEN_4: Color = Color::rgb_u8(48, 93, 66);

// LIGHT_GREEN_1: Color = Color::rgb_u8(77, 128, 97);
// LIGHT_GREEN_2: Color = Color::rgb_u8(137, 162, 87);
// LIGHT_GREEN_3: Color = Color::rgb_u8(190, 220, 127);
// LIGHT_GREEN_4: Color = Color::rgb_u8(238, 255, 204);

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

pub const MAIN_MENU_STYLE: Style = Style {
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    flex_direction: FlexDirection::Column,
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
    margin: UiRect::all(Val::Px(20.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
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
