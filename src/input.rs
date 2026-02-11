#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameKeyCode {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Tab,
    BackTab,
    Enter,
    Esc,
    Other,
}

#[derive(Debug, Clone, Copy)]
pub struct GameKeyEvent {
    pub code: GameKeyCode,
    pub ctrl: bool,
    pub shift: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<crossterm::event::KeyEvent> for GameKeyEvent {
    fn from(key: crossterm::event::KeyEvent) -> Self {
        use crossterm::event::{KeyCode, KeyModifiers};
        let code = match key.code {
            KeyCode::Char(c) => GameKeyCode::Char(c),
            KeyCode::Up => GameKeyCode::Up,
            KeyCode::Down => GameKeyCode::Down,
            KeyCode::Left => GameKeyCode::Left,
            KeyCode::Right => GameKeyCode::Right,
            KeyCode::Tab => GameKeyCode::Tab,
            KeyCode::BackTab => GameKeyCode::BackTab,
            KeyCode::Enter => GameKeyCode::Enter,
            KeyCode::Esc => GameKeyCode::Esc,
            _ => GameKeyCode::Other,
        };
        Self {
            code,
            ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
            shift: key.modifiers.contains(KeyModifiers::SHIFT),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl From<ratzilla::event::KeyEvent> for GameKeyEvent {
    fn from(key: ratzilla::event::KeyEvent) -> Self {
        use ratzilla::event::KeyCode;
        let code = match key.code {
            KeyCode::Tab if key.shift => GameKeyCode::BackTab,
            KeyCode::Char(c) => GameKeyCode::Char(c),
            KeyCode::Up => GameKeyCode::Up,
            KeyCode::Down => GameKeyCode::Down,
            KeyCode::Left => GameKeyCode::Left,
            KeyCode::Right => GameKeyCode::Right,
            KeyCode::Tab => GameKeyCode::Tab,
            KeyCode::Enter => GameKeyCode::Enter,
            KeyCode::Esc => GameKeyCode::Esc,
            _ => GameKeyCode::Other,
        };
        Self {
            code,
            ctrl: key.ctrl,
            shift: key.shift,
        }
    }
}
