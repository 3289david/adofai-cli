use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum GameInput {
    Hit,
    Pause,
    Quit,
    Restart,
    SpeedUp,
    SpeedDown,
    ToggleAutoPlay,
    MouseClick(u16, u16),
    MouseScroll(i16),
    None,
}

pub struct InputHandler {
    pub mouse_enabled: bool,
}

impl InputHandler {
    pub fn new(mouse_enabled: bool) -> Self {
        Self { mouse_enabled }
    }

    pub fn poll(&self, timeout: Duration) -> Option<GameInput> {
        if event::poll(timeout).ok()? {
            match event::read().ok()? {
                Event::Key(key) => Some(self.handle_key(key)),
                Event::Mouse(mouse) => Some(self.handle_mouse(mouse)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn handle_key(&self, key: KeyEvent) -> GameInput {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return GameInput::Quit;
        }

        match key.code {
            KeyCode::Char(' ') | KeyCode::Char('j') | KeyCode::Char('k')
            | KeyCode::Char('d') | KeyCode::Char('f') => GameInput::Hit,
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => GameInput::Hit,
            KeyCode::Char('p') | KeyCode::Esc => GameInput::Pause,
            KeyCode::Char('q') => GameInput::Quit,
            KeyCode::Char('r') => GameInput::Restart,
            KeyCode::Char('+') | KeyCode::Char('=') => GameInput::SpeedUp,
            KeyCode::Char('-') => GameInput::SpeedDown,
            KeyCode::Char('a') => GameInput::ToggleAutoPlay,
            _ => GameInput::None,
        }
    }

    fn handle_mouse(&self, mouse: MouseEvent) -> GameInput {
        match mouse.kind {
            MouseEventKind::Down(_) => GameInput::MouseClick(mouse.column, mouse.row),
            MouseEventKind::ScrollUp => GameInput::MouseScroll(1),
            MouseEventKind::ScrollDown => GameInput::MouseScroll(-1),
            _ => GameInput::None,
        }
    }
}
