use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, List, ListState, StatefulWidget, Widget},
};

use crate::{
    control::{Message, handle_event},
    player::{PLAYER_TYPE, PlayerOrHuman, get_player_from_selection},
};

pub fn player_selection_menu(terminal: &mut DefaultTerminal) -> (PlayerOrHuman, PlayerOrHuman) {
    let mut list_state = ListState::default().with_selected(Some(0));
    let player1_selection = loop {
        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.render_stateful_widget(
                    crate::ui::menu_widget::MenuWidget { player1: None },
                    area,
                    &mut list_state,
                );
            })
            .expect("failed to draw frame");
        let Some(message) = handle_event() else {
            continue;
        };

        match message {
            Message::Up => {
                list_state.select_previous();
            }
            Message::Down => {
                list_state.select_next();
            }
            Message::Select => {
                break list_state.selected().unwrap();
            }
            _ => {}
        }
    };

    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.render_stateful_widget(
                    MenuWidget {
                        player1: Some(player1_selection),
                    },
                    area,
                    &mut list_state,
                );
            })
            .expect("failed to draw frame");
        let Some(message) = handle_event() else {
            continue;
        };

        match message {
            Message::Up => {
                list_state.select_previous();
            }
            Message::Down => {
                list_state.select_next();
            }
            Message::Select => {
                return (
                    get_player_from_selection(player1_selection),
                    get_player_from_selection(list_state.selected().unwrap()),
                );
            }
            _ => {}
        }
    }
}

pub struct MenuWidget {
    pub(crate) player1: Option<usize>,
}

impl StatefulWidget for MenuWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let player1_list = List::new(PLAYER_TYPE)
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
