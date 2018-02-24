use std::collections::HashSet;
use core::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Unknown,
    Backspace,
    Tab,
    Return,
    Escape,
    Space,
    Exclaim,
    Quotedbl,
    Hash,
    Dollar,
    Percent,
    Ampersand,
    Quote,
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Colon,
    Semicolon,
    Less,
    Equals,
    Greater,
    Question,
    At,
    LeftBracket,
    Backslash,
    RightBracket,
    Caret,
    Underscore,
    Backquote,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Delete,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLockClear,
    KpDivide,
    KpMultiply,
    KpMinus,
    KpPlus,
    KpEnter,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    Kp0,
    KpPeriod,
    Application,
    Power,
    KpEquals,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    KpComma,
    KpEqualsAS400,
    AltErase,
    Sysreq,
    Cancel,
    Clear,
    Prior,
    Return2,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,
    Kp00,
    Kp000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubUnit,
    KpLeftParen,
    KpRightParen,
    KpLeftBrace,
    KpRightBrace,
    KpTab,
    KpBackspace,
    KpA,
    KpB,
    KpC,
    KpD,
    KpE,
    KpF,
    KpXor,
    KpPower,
    KpPercent,
    KpLess,
    KpGreater,
    KpAmpersand,
    KpDblAmpersand,
    KpVerticalBar,
    KpDblVerticalBar,
    KpColon,
    KpHash,
    KpSpace,
    KpAt,
    KpExclam,
    KpMemStore,
    KpMemRecall,
    KpMemClear,
    KpMemAdd,
    KpMemSubtract,
    KpMemMultiply,
    KpMemDivide,
    KpPlusMinus,
    KpClear,
    KpClearEntry,
    KpBinary,
    KpOctal,
    KpDecimal,
    KpHexadecimal,
    Ctrl,
    Shift,
    Alt,
    Gui,
    Mode,
    AudioNext,
    AudioPrev,
    AudioStop,
    AudioPlay,
    AudioMute,
    MediaSelect,
    Www,
    Mail,
    Calculator,
    Computer,
    AcSearch,
    AcHome,
    AcBack,
    AcForward,
    AcStop,
    AcRefresh,
    AcBookmarks,
    BrightnessDown,
    BrightnessUp,
    DisplaySwitch,
    KbdIllumToggle,
    KbdIllumDown,
    KbdIllumUp,
    Eject,
    Sleep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Unknown,
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub enum InputEvent {
    MouseMove(i32, i32),
    MouseDown {
        button: MouseButton,
        position: (i32, i32),
    },
    MouseUp {
        button: MouseButton,
        position: (i32, i32),
    },
    KeyDown(Key),
    KeyUp(Key),
}

#[derive(Clone)]
struct InputState {
    pub keys_down: HashSet<Key>,
    pub mouse_buttons_down: HashSet<MouseButton>,
    pub mouse_position: Vec2,
}

impl InputState {
    fn new() -> InputState {
        InputState {
            keys_down: HashSet::new(),
            mouse_buttons_down: HashSet::new(),
            mouse_position: Vec2::zero(),
        }
    }
}

pub struct Input {
    last_state: InputState,
    current_state: InputState,
}

impl Input {
    pub fn new() -> Input {
        Input {
            last_state: InputState::new(),
            current_state: InputState::new(),
        }
    }

    pub fn update(&mut self, events: &[InputEvent]) {
        self.last_state = self.current_state.clone();

        for e in events {
            match *e {
                InputEvent::KeyDown(key) => {
                    self.current_state.keys_down.insert(key);
                }
                InputEvent::KeyUp(key) => {
                    self.current_state.keys_down.remove(&key);
                }
                InputEvent::MouseDown { button, .. } => {
                    self.current_state.mouse_buttons_down.insert(button);
                }
                InputEvent::MouseUp { button, .. } => {
                    self.current_state.mouse_buttons_down.remove(&button);
                }
                InputEvent::MouseMove(x, y) => {
                    self.current_state.mouse_position = Vec2::new(x as f32, y as f32);
                }
            }
        }
    }

    pub fn key_is_down(&self, key: &Key) -> bool {
        self.current_state.keys_down.contains(key)
    }

    pub fn key_is_pressed(&self, key: &Key) -> bool {
        !self.last_state.keys_down.contains(key) && self.current_state.keys_down.contains(key)
    }

    pub fn key_is_released(&self, key: &Key) -> bool {
        self.last_state.keys_down.contains(key) && !self.current_state.keys_down.contains(key)
    }

    pub fn mouse_button_is_down(&self, button: &MouseButton) -> bool {
        self.current_state.mouse_buttons_down.contains(button)
    }

    pub fn mouse_button_is_pressed(&self, button: &MouseButton) -> bool {
        !self.last_state.mouse_buttons_down.contains(button)
            && self.current_state.mouse_buttons_down.contains(button)
    }

    pub fn mouse_button_is_released(&self, button: &MouseButton) -> bool {
        self.last_state.mouse_buttons_down.contains(button)
            && !self.current_state.mouse_buttons_down.contains(button)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.current_state.mouse_position
    }
}
