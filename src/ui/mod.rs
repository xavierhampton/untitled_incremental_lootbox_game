mod chest_art;
mod game_view;
mod layout;
mod tab_panel;
pub mod tabs;
pub mod widgets;

use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    layout::draw_layout(frame, app);
}
