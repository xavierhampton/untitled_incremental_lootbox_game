use ratatui::prelude::*;

use crate::game::item::Rarity;

pub fn rarity_span(rarity: Rarity) -> Span<'static> {
    let label = format!("[{}]", rarity.label());
    Span::styled(
        label,
        Style::default()
            .fg(rarity.color())
            .add_modifier(Modifier::BOLD),
    )
}
