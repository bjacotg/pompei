use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub enum Message {
    Select,
    Up,
    Down,
    Left,
    Right,
    Quit,
    Reset,
}

pub fn handle_event() -> Option<Message> {
    let Ok(Event::Key(KeyEvent { code, .. })) = event::read() else {
        return None;
    };

    match code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('h') | KeyCode::Left => Some(Message::Left),
        KeyCode::Char('j') | KeyCode::Down => Some(Message::Down),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::Up),
        KeyCode::Char('l') | KeyCode::Right => Some(Message::Right),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Message::Select),
        KeyCode::Esc => Some(Message::Reset),
        _ => None,
    }
}
