use ratatui::{
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::game::{
    Game,
    prelude::{Construction, Player, Position, Tile},
};

pub struct BoardWidget<'a>(pub &'a Game, pub Position);

impl<'a> Widget for BoardWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // TODO make tile squarer
        let board_length = std::cmp::min(area.width, area.height);
        let board_area = area.inner(Margin {
            horizontal: (area.width - board_length) / 2,
            vertical: (area.height - board_length) / 2,
        });

        let grid = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Ratio(1, 5); 5])
            .split(board_area)
            .iter()
            .map(|&column| {
                Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints(vec![Constraint::Ratio(1, 5); 5])
                    .split(column)
            })
            .collect::<Vec<_>>();
        for (position, tile) in self.0.board().get_tiles() {
            TileWidget {
                tile,
                cursor: position == self.1,
                selected: self.0.selected().contains(position),
                selectable: self.0.selectable().contains(position),
            }
            .render(grid[position.row()][position.col()], buf);
        }
    }
}

struct TileWidget<'a> {
    tile: &'a Tile,
    cursor: bool,
    selected: bool,
    selectable: bool,
}

impl<'a> Widget for TileWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let player = match self.tile.player {
            Some(Player::Player1) => "P1",
            Some(Player::Player2) => "P2",
            None => "  ",
        };

        let construction = match self.tile.construction {
            Construction::GroundLevel => "GF",
            Construction::FirstLevel => "1F",
            Construction::SecondLevel => "2F",
            Construction::ThirdLevel => "3F",
            Construction::Dome => "DD",
        };

        let color = if self.selectable {
            Color::Green
        } else {
            Color::Red
        };
        let style = if self.cursor {
            Style::from(Modifier::BOLD | Modifier::ITALIC)
        } else {
            Style::default()
        };
        let border_type = if self.selected {
            BorderType::Double
        } else {
            BorderType::Plain
        };

        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(border_type)
            .border_style(Style::default().fg(color));

        Paragraph::new(format!("{player} {construction}"))
            .style(style)
            .block(block)
            .render(area, buf);
    }
}
