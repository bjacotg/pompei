use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, List, ListState, StatefulWidget, Widget},
};

pub struct MenuWidget {
    pub(crate) player1: Option<usize>,
}

impl StatefulWidget for MenuWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let player_type = ["Human Player", "Random", "Pick First"];
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let player1_list = List::new(player_type)
            .block(Block::bordered().title("Player 1"))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        let player2_list = player1_list
            .clone()
            .block(Block::bordered().title("Player 2"));

        if let Some(player1_selection) = self.player1 {
            let mut player1_selection = ListState::default().with_selected(Some(player1_selection));
            StatefulWidget::render(player1_list, layout[0], buf, &mut player1_selection);

            StatefulWidget::render(player2_list, layout[1], buf, state);
        } else {
            StatefulWidget::render(player1_list, layout[0], buf, state);

            Widget::render(player2_list, layout[1], buf);
        }
    }
}
