use crate::error::WHKError;
use std::hash::Hash;

/// Represents a virtual key (VK) code.
///
/// # See Also
/// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
///
#[derive(Debug, Clone, Copy)]
pub enum VKey {
    Back,
    Tab,
    Clear,
    Return,
    Shift,
    Control,
    Menu,
    Pause,
    Capital,
    Escape,
    Space,
    Prior,
    Next,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Select,
    Print,
    Execute,
    Snapshot,
    Insert,
    Delete,
    Help,
    LWin,
    RWin,
    Apps,
    Sleep,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
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
    Numlock,
    Scroll,
    LShift,
    RShift,
    LControl,
    RControl,
    LMenu,
    RMenu,
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserHome,
    VolumeMute,
    VolumeDown,
    VolumeUp,
    MediaNextTrack,
    MediaPrevTrack,
    MediaStop,
    MediaPlayPause,
    LaunchMail,
    LaunchMediaSelect,
    LaunchApp1,
    LaunchApp2,
    Oem1,
    OemPlus,
    OemComma,
    OemMinus,
    OemPeriod,
    Oem2,
    Oem3,
    Oem4,
    Oem5,
    Oem6,
    Oem7,
    Oem8,
    Oem102,
    Attn,
    Crsel,
    Exsel,
    Play,
    Zoom,
    Pa1,
    OemClear,
    Vk0,
    Vk1,
    Vk2,
    Vk3,
    Vk4,
    Vk5,
    Vk6,
    Vk7,
    Vk8,
    Vk9,
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
    CustomKeyCode(u16),
}

impl VKey {
    /// Converts a `VKey` to its corresponding Windows Virtual-Key (VK) code.
    ///
    /// # See Also
    /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
    ///
    pub const fn to_vk_code(&self) -> u16 {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        match self {
            VKey::Back => VK_BACK.0,
            VKey::Tab => VK_TAB.0,
            VKey::Clear => VK_CLEAR.0,
            VKey::Return => VK_RETURN.0,
            VKey::Shift => VK_SHIFT.0,
            VKey::Control => VK_CONTROL.0,
            VKey::Menu => VK_MENU.0,
            VKey::Pause => VK_PAUSE.0,
            VKey::Capital => VK_CAPITAL.0,
            VKey::Escape => VK_ESCAPE.0,
            VKey::Space => VK_SPACE.0,
            VKey::Prior => VK_PRIOR.0,
            VKey::Next => VK_NEXT.0,
            VKey::End => VK_END.0,
            VKey::Home => VK_HOME.0,
            VKey::Left => VK_LEFT.0,
            VKey::Up => VK_UP.0,
            VKey::Right => VK_RIGHT.0,
            VKey::Down => VK_DOWN.0,
            VKey::Select => VK_SELECT.0,
            VKey::Print => VK_PRINT.0,
            VKey::Execute => VK_EXECUTE.0,
            VKey::Snapshot => VK_SNAPSHOT.0,
            VKey::Insert => VK_INSERT.0,
            VKey::Delete => VK_DELETE.0,
            VKey::Help => VK_HELP.0,
            VKey::LWin => VK_LWIN.0,
            VKey::RWin => VK_RWIN.0,
            VKey::Apps => VK_APPS.0,
            VKey::Sleep => VK_SLEEP.0,
            VKey::Numpad0 => VK_NUMPAD0.0,
            VKey::Numpad1 => VK_NUMPAD1.0,
            VKey::Numpad2 => VK_NUMPAD2.0,
            VKey::Numpad3 => VK_NUMPAD3.0,
            VKey::Numpad4 => VK_NUMPAD4.0,
            VKey::Numpad5 => VK_NUMPAD5.0,
            VKey::Numpad6 => VK_NUMPAD6.0,
            VKey::Numpad7 => VK_NUMPAD7.0,
            VKey::Numpad8 => VK_NUMPAD8.0,
            VKey::Numpad9 => VK_NUMPAD9.0,
            VKey::Multiply => VK_MULTIPLY.0,
            VKey::Add => VK_ADD.0,
            VKey::Separator => VK_SEPARATOR.0,
            VKey::Subtract => VK_SUBTRACT.0,
            VKey::Decimal => VK_DECIMAL.0,
            VKey::Divide => VK_DIVIDE.0,
            VKey::F1 => VK_F1.0,
            VKey::F2 => VK_F2.0,
            VKey::F3 => VK_F3.0,
            VKey::F4 => VK_F4.0,
            VKey::F5 => VK_F5.0,
            VKey::F6 => VK_F6.0,
            VKey::F7 => VK_F7.0,
            VKey::F8 => VK_F8.0,
            VKey::F9 => VK_F9.0,
            VKey::F10 => VK_F10.0,
            VKey::F11 => VK_F11.0,
            VKey::F12 => VK_F12.0,
            VKey::F13 => VK_F13.0,
            VKey::F14 => VK_F14.0,
            VKey::F15 => VK_F15.0,
            VKey::F16 => VK_F16.0,
            VKey::F17 => VK_F17.0,
            VKey::F18 => VK_F18.0,
            VKey::F19 => VK_F19.0,
            VKey::F20 => VK_F20.0,
            VKey::F21 => VK_F21.0,
            VKey::F22 => VK_F22.0,
            VKey::F23 => VK_F23.0,
            VKey::F24 => VK_F24.0,
            VKey::Numlock => VK_NUMLOCK.0,
            VKey::Scroll => VK_SCROLL.0,
            VKey::LShift => VK_LSHIFT.0,
            VKey::RShift => VK_RSHIFT.0,
            VKey::LControl => VK_LCONTROL.0,
            VKey::RControl => VK_RCONTROL.0,
            VKey::LMenu => VK_LMENU.0,
            VKey::RMenu => VK_RMENU.0,
            VKey::BrowserBack => VK_BROWSER_BACK.0,
            VKey::BrowserForward => VK_BROWSER_FORWARD.0,
            VKey::BrowserRefresh => VK_BROWSER_REFRESH.0,
            VKey::BrowserStop => VK_BROWSER_STOP.0,
            VKey::BrowserSearch => VK_BROWSER_SEARCH.0,
            VKey::BrowserFavorites => VK_BROWSER_FAVORITES.0,
            VKey::BrowserHome => VK_BROWSER_HOME.0,
            VKey::VolumeMute => VK_VOLUME_MUTE.0,
            VKey::VolumeDown => VK_VOLUME_DOWN.0,
            VKey::VolumeUp => VK_VOLUME_UP.0,
            VKey::MediaNextTrack => VK_MEDIA_NEXT_TRACK.0,
            VKey::MediaPrevTrack => VK_MEDIA_PREV_TRACK.0,
            VKey::MediaStop => VK_MEDIA_STOP.0,
            VKey::MediaPlayPause => VK_MEDIA_PLAY_PAUSE.0,
            VKey::LaunchMail => VK_LAUNCH_MAIL.0,
            VKey::LaunchMediaSelect => VK_LAUNCH_MEDIA_SELECT.0,
            VKey::LaunchApp1 => VK_LAUNCH_APP1.0,
            VKey::LaunchApp2 => VK_LAUNCH_APP2.0,
            VKey::Oem1 => VK_OEM_1.0,
            VKey::OemPlus => VK_OEM_PLUS.0,
            VKey::OemComma => VK_OEM_COMMA.0,
            VKey::OemMinus => VK_OEM_MINUS.0,
            VKey::OemPeriod => VK_OEM_PERIOD.0,
            VKey::Oem2 => VK_OEM_2.0,
            VKey::Oem3 => VK_OEM_3.0,
            VKey::Oem4 => VK_OEM_4.0,
            VKey::Oem5 => VK_OEM_5.0,
            VKey::Oem6 => VK_OEM_6.0,
            VKey::Oem7 => VK_OEM_7.0,
            VKey::Oem8 => VK_OEM_8.0,
            VKey::Oem102 => VK_OEM_102.0,
            VKey::Attn => VK_ATTN.0,
            VKey::Crsel => VK_CRSEL.0,
            VKey::Exsel => VK_EXSEL.0,
            VKey::Play => VK_PLAY.0,
            VKey::Zoom => VK_ZOOM.0,
            VKey::Pa1 => VK_PA1.0,
            VKey::OemClear => VK_OEM_CLEAR.0,
            VKey::Vk0 => VK_0.0,
            VKey::Vk1 => VK_1.0,
            VKey::Vk2 => VK_2.0,
            VKey::Vk3 => VK_3.0,
            VKey::Vk4 => VK_4.0,
            VKey::Vk5 => VK_5.0,
            VKey::Vk6 => VK_6.0,
            VKey::Vk7 => VK_7.0,
            VKey::Vk8 => VK_8.0,
            VKey::Vk9 => VK_9.0,
            VKey::A => VK_A.0,
            VKey::B => VK_B.0,
            VKey::C => VK_C.0,
            VKey::D => VK_D.0,
            VKey::E => VK_E.0,
            VKey::F => VK_F.0,
            VKey::G => VK_G.0,
            VKey::H => VK_H.0,
            VKey::I => VK_I.0,
            VKey::J => VK_J.0,
            VKey::K => VK_K.0,
            VKey::L => VK_L.0,
            VKey::M => VK_M.0,
            VKey::N => VK_N.0,
            VKey::O => VK_O.0,
            VKey::P => VK_P.0,
            VKey::Q => VK_Q.0,
            VKey::R => VK_R.0,
            VKey::S => VK_S.0,
            VKey::T => VK_T.0,
            VKey::U => VK_U.0,
            VKey::V => VK_V.0,
            VKey::W => VK_W.0,
            VKey::X => VK_X.0,
            VKey::Y => VK_Y.0,
            VKey::Z => VK_Z.0,
            VKey::CustomKeyCode(vk) => *vk,
        }
    }

    /// Returns a `VKey` based a Windows Virtual-Key (VK) code.
    ///
    /// # See Also
    /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
    ///
    pub const fn from_vk_code(vk_code: u16) -> VKey {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        match vk_code {
            _ if vk_code == VK_BACK.0 => VKey::Back,
            _ if vk_code == VK_TAB.0 => VKey::Tab,
            _ if vk_code == VK_CLEAR.0 => VKey::Clear,
            _ if vk_code == VK_RETURN.0 => VKey::Return,
            _ if vk_code == VK_SHIFT.0 => VKey::Shift,
            _ if vk_code == VK_CONTROL.0 => VKey::Control,
            _ if vk_code == VK_MENU.0 => VKey::Menu,
            _ if vk_code == VK_PAUSE.0 => VKey::Pause,
            _ if vk_code == VK_CAPITAL.0 => VKey::Capital,
            _ if vk_code == VK_ESCAPE.0 => VKey::Escape,
            _ if vk_code == VK_SPACE.0 => VKey::Space,
            _ if vk_code == VK_PRIOR.0 => VKey::Prior,
            _ if vk_code == VK_NEXT.0 => VKey::Next,
            _ if vk_code == VK_END.0 => VKey::End,
            _ if vk_code == VK_HOME.0 => VKey::Home,
            _ if vk_code == VK_LEFT.0 => VKey::Left,
            _ if vk_code == VK_UP.0 => VKey::Up,
            _ if vk_code == VK_RIGHT.0 => VKey::Right,
            _ if vk_code == VK_DOWN.0 => VKey::Down,
            _ if vk_code == VK_SELECT.0 => VKey::Select,
            _ if vk_code == VK_PRINT.0 => VKey::Print,
            _ if vk_code == VK_EXECUTE.0 => VKey::Execute,
            _ if vk_code == VK_SNAPSHOT.0 => VKey::Snapshot,
            _ if vk_code == VK_INSERT.0 => VKey::Insert,
            _ if vk_code == VK_DELETE.0 => VKey::Delete,
            _ if vk_code == VK_HELP.0 => VKey::Help,
            _ if vk_code == VK_LWIN.0 => VKey::LWin,
            _ if vk_code == VK_RWIN.0 => VKey::RWin,
            _ if vk_code == VK_APPS.0 => VKey::Apps,
            _ if vk_code == VK_SLEEP.0 => VKey::Sleep,
            _ if vk_code == VK_NUMPAD0.0 => VKey::Numpad0,
            _ if vk_code == VK_NUMPAD1.0 => VKey::Numpad1,
            _ if vk_code == VK_NUMPAD2.0 => VKey::Numpad2,
            _ if vk_code == VK_NUMPAD3.0 => VKey::Numpad3,
            _ if vk_code == VK_NUMPAD4.0 => VKey::Numpad4,
            _ if vk_code == VK_NUMPAD5.0 => VKey::Numpad5,
            _ if vk_code == VK_NUMPAD6.0 => VKey::Numpad6,
            _ if vk_code == VK_NUMPAD7.0 => VKey::Numpad7,
            _ if vk_code == VK_NUMPAD8.0 => VKey::Numpad8,
            _ if vk_code == VK_NUMPAD9.0 => VKey::Numpad9,
            _ if vk_code == VK_MULTIPLY.0 => VKey::Multiply,
            _ if vk_code == VK_ADD.0 => VKey::Add,
            _ if vk_code == VK_SEPARATOR.0 => VKey::Separator,
            _ if vk_code == VK_SUBTRACT.0 => VKey::Subtract,
            _ if vk_code == VK_DECIMAL.0 => VKey::Decimal,
            _ if vk_code == VK_DIVIDE.0 => VKey::Divide,
            _ if vk_code == VK_F1.0 => VKey::F1,
            _ if vk_code == VK_F2.0 => VKey::F2,
            _ if vk_code == VK_F3.0 => VKey::F3,
            _ if vk_code == VK_F4.0 => VKey::F4,
            _ if vk_code == VK_F5.0 => VKey::F5,
            _ if vk_code == VK_F6.0 => VKey::F6,
            _ if vk_code == VK_F7.0 => VKey::F7,
            _ if vk_code == VK_F8.0 => VKey::F8,
            _ if vk_code == VK_F9.0 => VKey::F9,
            _ if vk_code == VK_F10.0 => VKey::F10,
            _ if vk_code == VK_F11.0 => VKey::F11,
            _ if vk_code == VK_F12.0 => VKey::F12,
            _ if vk_code == VK_F13.0 => VKey::F13,
            _ if vk_code == VK_F14.0 => VKey::F14,
            _ if vk_code == VK_F15.0 => VKey::F15,
            _ if vk_code == VK_F16.0 => VKey::F16,
            _ if vk_code == VK_F17.0 => VKey::F17,
            _ if vk_code == VK_F18.0 => VKey::F18,
            _ if vk_code == VK_F19.0 => VKey::F19,
            _ if vk_code == VK_F20.0 => VKey::F20,
            _ if vk_code == VK_F21.0 => VKey::F21,
            _ if vk_code == VK_F22.0 => VKey::F22,
            _ if vk_code == VK_F23.0 => VKey::F23,
            _ if vk_code == VK_F24.0 => VKey::F24,
            _ if vk_code == VK_NUMLOCK.0 => VKey::Numlock,
            _ if vk_code == VK_SCROLL.0 => VKey::Scroll,
            _ if vk_code == VK_LSHIFT.0 => VKey::LShift,
            _ if vk_code == VK_RSHIFT.0 => VKey::RShift,
            _ if vk_code == VK_LCONTROL.0 => VKey::LControl,
            _ if vk_code == VK_RCONTROL.0 => VKey::RControl,
            _ if vk_code == VK_LMENU.0 => VKey::LMenu,
            _ if vk_code == VK_RMENU.0 => VKey::RMenu,
            _ if vk_code == VK_BROWSER_BACK.0 => VKey::BrowserBack,
            _ if vk_code == VK_BROWSER_FORWARD.0 => VKey::BrowserForward,
            _ if vk_code == VK_BROWSER_REFRESH.0 => VKey::BrowserRefresh,
            _ if vk_code == VK_BROWSER_STOP.0 => VKey::BrowserStop,
            _ if vk_code == VK_BROWSER_SEARCH.0 => VKey::BrowserSearch,
            _ if vk_code == VK_BROWSER_FAVORITES.0 => VKey::BrowserFavorites,
            _ if vk_code == VK_BROWSER_HOME.0 => VKey::BrowserHome,
            _ if vk_code == VK_VOLUME_MUTE.0 => VKey::VolumeMute,
            _ if vk_code == VK_VOLUME_DOWN.0 => VKey::VolumeDown,
            _ if vk_code == VK_VOLUME_UP.0 => VKey::VolumeUp,
            _ if vk_code == VK_MEDIA_NEXT_TRACK.0 => VKey::MediaNextTrack,
            _ if vk_code == VK_MEDIA_PREV_TRACK.0 => VKey::MediaPrevTrack,
            _ if vk_code == VK_MEDIA_STOP.0 => VKey::MediaStop,
            _ if vk_code == VK_MEDIA_PLAY_PAUSE.0 => VKey::MediaPlayPause,
            _ if vk_code == VK_LAUNCH_MAIL.0 => VKey::LaunchMail,
            _ if vk_code == VK_LAUNCH_MEDIA_SELECT.0 => VKey::LaunchMediaSelect,
            _ if vk_code == VK_LAUNCH_APP1.0 => VKey::LaunchApp1,
            _ if vk_code == VK_LAUNCH_APP2.0 => VKey::LaunchApp2,
            _ if vk_code == VK_OEM_1.0 => VKey::Oem1,
            _ if vk_code == VK_OEM_PLUS.0 => VKey::OemPlus,
            _ if vk_code == VK_OEM_COMMA.0 => VKey::OemComma,
            _ if vk_code == VK_OEM_MINUS.0 => VKey::OemMinus,
            _ if vk_code == VK_OEM_PERIOD.0 => VKey::OemPeriod,
            _ if vk_code == VK_OEM_2.0 => VKey::Oem2,
            _ if vk_code == VK_OEM_3.0 => VKey::Oem3,
            _ if vk_code == VK_OEM_4.0 => VKey::Oem4,
            _ if vk_code == VK_OEM_5.0 => VKey::Oem5,
            _ if vk_code == VK_OEM_6.0 => VKey::Oem6,
            _ if vk_code == VK_OEM_7.0 => VKey::Oem7,
            _ if vk_code == VK_OEM_8.0 => VKey::Oem8,
            _ if vk_code == VK_OEM_102.0 => VKey::Oem102,
            _ if vk_code == VK_ATTN.0 => VKey::Attn,
            _ if vk_code == VK_CRSEL.0 => VKey::Crsel,
            _ if vk_code == VK_EXSEL.0 => VKey::Exsel,
            _ if vk_code == VK_PLAY.0 => VKey::Play,
            _ if vk_code == VK_ZOOM.0 => VKey::Zoom,
            _ if vk_code == VK_PA1.0 => VKey::Pa1,
            _ if vk_code == VK_OEM_CLEAR.0 => VKey::OemClear,
            _ if vk_code == VK_0.0 => VKey::Vk0,
            _ if vk_code == VK_1.0 => VKey::Vk1,
            _ if vk_code == VK_2.0 => VKey::Vk2,
            _ if vk_code == VK_3.0 => VKey::Vk3,
            _ if vk_code == VK_4.0 => VKey::Vk4,
            _ if vk_code == VK_5.0 => VKey::Vk5,
            _ if vk_code == VK_6.0 => VKey::Vk6,
            _ if vk_code == VK_7.0 => VKey::Vk7,
            _ if vk_code == VK_8.0 => VKey::Vk8,
            _ if vk_code == VK_9.0 => VKey::Vk9,
            _ if vk_code == VK_A.0 => VKey::A,
            _ if vk_code == VK_B.0 => VKey::B,
            _ if vk_code == VK_C.0 => VKey::C,
            _ if vk_code == VK_D.0 => VKey::D,
            _ if vk_code == VK_E.0 => VKey::E,
            _ if vk_code == VK_F.0 => VKey::F,
            _ if vk_code == VK_G.0 => VKey::G,
            _ if vk_code == VK_H.0 => VKey::H,
            _ if vk_code == VK_I.0 => VKey::I,
            _ if vk_code == VK_J.0 => VKey::J,
            _ if vk_code == VK_K.0 => VKey::K,
            _ if vk_code == VK_L.0 => VKey::L,
            _ if vk_code == VK_M.0 => VKey::M,
            _ if vk_code == VK_N.0 => VKey::N,
            _ if vk_code == VK_O.0 => VKey::O,
            _ if vk_code == VK_P.0 => VKey::P,
            _ if vk_code == VK_Q.0 => VKey::Q,
            _ if vk_code == VK_R.0 => VKey::R,
            _ if vk_code == VK_S.0 => VKey::S,
            _ if vk_code == VK_T.0 => VKey::T,
            _ if vk_code == VK_U.0 => VKey::U,
            _ if vk_code == VK_V.0 => VKey::V,
            _ if vk_code == VK_W.0 => VKey::W,
            _ if vk_code == VK_X.0 => VKey::X,
            _ if vk_code == VK_Y.0 => VKey::Y,
            _ if vk_code == VK_Z.0 => VKey::Z,
            _ => VKey::CustomKeyCode(vk_code),
        }
    }

    /// Creates a `VKey` from a string representation of the key.
    ///
    /// NOTE: Certain common aliases for keys are accepted in addition to the Microsoft Virtual-Key Codes names
    ///
    /// WIN maps to `VKey::LWin`
    /// CTRL maps to `VKey::Control`
    /// ALT maps to `VKey::Menu`
    ///
    /// # See Also
    /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
    ///
    pub fn from_keyname(name: &str) -> Result<VKey, WHKError> {
        let name = name.to_ascii_uppercase();

        // 1 byte hex code => Use the raw keycode value
        if name.len() >= 3 && name.len() <= 6 && name.starts_with("0x") || name.starts_with("0X") {
            return if let Ok(val) = u16::from_str_radix(&name[2..], 16) {
                Ok(Self::from_vk_code(val))
            } else {
                Err(WHKError::InvalidKey(name))
            };
        }

        Ok(match name.trim_start_matches("VK_") {
            "BACK" => VKey::Back,
            "TAB" => VKey::Tab,
            "CLEAR" => VKey::Clear,
            "RETURN" => VKey::Return,
            "SHIFT" => VKey::Shift,
            "CTRL" => VKey::Control,
            "CONTROL" => VKey::Control,
            "ALT" => VKey::Menu,
            "MENU" => VKey::Menu,
            "PAUSE" => VKey::Pause,
            "CAPITAL" => VKey::Capital,
            "ESCAPE" => VKey::Escape,
            "SPACE" => VKey::Space,
            "PRIOR" => VKey::Prior,
            "NEXT" => VKey::Next,
            "END" => VKey::End,
            "HOME" => VKey::Home,
            "LEFT" => VKey::Left,
            "UP" => VKey::Up,
            "RIGHT" => VKey::Right,
            "DOWN" => VKey::Down,
            "SELECT" => VKey::Select,
            "PRINT" => VKey::Print,
            "EXECUTE" => VKey::Execute,
            "SNAPSHOT" => VKey::Snapshot,
            "INSERT" => VKey::Insert,
            "DELETE" => VKey::Delete,
            "HELP" => VKey::Help,
            "WIN" => VKey::LWin,
            "LWIN" => VKey::LWin,
            "RWIN" => VKey::RWin,
            "APPS" => VKey::Apps,
            "SLEEP" => VKey::Sleep,
            "NUMPAD0" => VKey::Numpad0,
            "NUMPAD1" => VKey::Numpad1,
            "NUMPAD2" => VKey::Numpad2,
            "NUMPAD3" => VKey::Numpad3,
            "NUMPAD4" => VKey::Numpad4,
            "NUMPAD5" => VKey::Numpad5,
            "NUMPAD6" => VKey::Numpad6,
            "NUMPAD7" => VKey::Numpad7,
            "NUMPAD8" => VKey::Numpad8,
            "NUMPAD9" => VKey::Numpad9,
            "MULTIPLY" => VKey::Multiply,
            "ADD" => VKey::Add,
            "SEPARATOR" => VKey::Separator,
            "SUBTRACT" => VKey::Subtract,
            "DECIMAL" => VKey::Decimal,
            "DIVIDE" => VKey::Divide,
            "F1" => VKey::F1,
            "F2" => VKey::F2,
            "F3" => VKey::F3,
            "F4" => VKey::F4,
            "F5" => VKey::F5,
            "F6" => VKey::F6,
            "F7" => VKey::F7,
            "F8" => VKey::F8,
            "F9" => VKey::F9,
            "F10" => VKey::F10,
            "F11" => VKey::F11,
            "F12" => VKey::F12,
            "F13" => VKey::F13,
            "F14" => VKey::F14,
            "F15" => VKey::F15,
            "F16" => VKey::F16,
            "F17" => VKey::F17,
            "F18" => VKey::F18,
            "F19" => VKey::F19,
            "F20" => VKey::F20,
            "F21" => VKey::F21,
            "F22" => VKey::F22,
            "F23" => VKey::F23,
            "F24" => VKey::F24,
            "NUMLOCK" => VKey::Numlock,
            "SCROLL" => VKey::Scroll,
            "LSHIFT" => VKey::LShift,
            "RSHIFT" => VKey::RShift,
            "LCTRL" => VKey::LControl,
            "LCONTROL" => VKey::LControl,
            "RCTRL" => VKey::RControl,
            "RCONTROL" => VKey::RControl,
            "LALT" => VKey::LMenu,
            "LMENU" => VKey::LMenu,
            "RALT" => VKey::RMenu,
            "RMENU" => VKey::RMenu,
            "BROWSER_BACK" => VKey::BrowserBack,
            "BROWSER_FORWARD" => VKey::BrowserForward,
            "BROWSER_REFRESH" => VKey::BrowserRefresh,
            "BROWSER_STOP" => VKey::BrowserStop,
            "BROWSER_SEARCH" => VKey::BrowserSearch,
            "BROWSER_FAVORITES" => VKey::BrowserFavorites,
            "BROWSER_HOME" => VKey::BrowserHome,
            "VOLUME_MUTE" => VKey::VolumeMute,
            "VOLUME_DOWN" => VKey::VolumeDown,
            "VOLUME_UP" => VKey::VolumeUp,
            "MEDIA_NEXT_TRACK" => VKey::MediaNextTrack,
            "MEDIA_PREV_TRACK" => VKey::MediaPrevTrack,
            "MEDIA_STOP" => VKey::MediaStop,
            "MEDIA_PLAY_PAUSE" => VKey::MediaPlayPause,
            "LAUNCH_MAIL" => VKey::LaunchMail,
            "LAUNCH_MEDIA_SELECT" => VKey::LaunchMediaSelect,
            "LAUNCH_APP1" => VKey::LaunchApp1,
            "LAUNCH_APP2" => VKey::LaunchApp2,
            "OEM_1" => VKey::Oem1,
            "OEM_PLUS" => VKey::OemPlus,
            "OEM_COMMA" => VKey::OemComma,
            "OEM_MINUS" => VKey::OemMinus,
            "OEM_PERIOD" => VKey::OemPeriod,
            "OEM_2" => VKey::Oem2,
            "OEM_3" => VKey::Oem3,
            "OEM_4" => VKey::Oem4,
            "OEM_5" => VKey::Oem5,
            "OEM_6" => VKey::Oem6,
            "OEM_7" => VKey::Oem7,
            "OEM_8" => VKey::Oem8,
            "OEM_102" => VKey::Oem102,
            "ATTN" => VKey::Attn,
            "CRSEL" => VKey::Crsel,
            "EXSEL" => VKey::Exsel,
            "PLAY" => VKey::Play,
            "ZOOM" => VKey::Zoom,
            "PA1" => VKey::Pa1,
            "OEM_CLEAR" => VKey::OemClear,
            "0" => VKey::Vk0,
            "1" => VKey::Vk1,
            "2" => VKey::Vk2,
            "3" => VKey::Vk3,
            "4" => VKey::Vk4,
            "5" => VKey::Vk5,
            "6" => VKey::Vk6,
            "7" => VKey::Vk7,
            "8" => VKey::Vk8,
            "9" => VKey::Vk9,
            "A" => VKey::A,
            "B" => VKey::B,
            "C" => VKey::C,
            "D" => VKey::D,
            "E" => VKey::E,
            "F" => VKey::F,
            "G" => VKey::G,
            "H" => VKey::H,
            "I" => VKey::I,
            "J" => VKey::J,
            "K" => VKey::K,
            "L" => VKey::L,
            "M" => VKey::M,
            "N" => VKey::N,
            "O" => VKey::O,
            "P" => VKey::P,
            "Q" => VKey::Q,
            "R" => VKey::R,
            "S" => VKey::S,
            "T" => VKey::T,
            "U" => VKey::U,
            "V" => VKey::V,
            "W" => VKey::W,
            "X" => VKey::X,
            "Y" => VKey::Y,
            "Z" => VKey::Z,
            _ => return Err(WHKError::InvalidKey(name)),
        })
    }
}

impl std::fmt::Display for VKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            VKey::Back => "VK_BACK",
            VKey::Tab => "VK_TAB",
            VKey::Clear => "VK_CLEAR",
            VKey::Return => "VK_RETURN",
            VKey::Shift => "VK_SHIFT",
            VKey::Control => "VK_CONTROL",
            VKey::Menu => "VK_MENU",
            VKey::Pause => "VK_PAUSE",
            VKey::Capital => "VK_CAPITAL",
            VKey::Escape => "VK_ESCAPE",
            VKey::Space => "VK_SPACE",
            VKey::Prior => "VK_PRIOR",
            VKey::Next => "VK_NEXT",
            VKey::End => "VK_END",
            VKey::Home => "VK_HOME",
            VKey::Left => "VK_LEFT",
            VKey::Up => "VK_UP",
            VKey::Right => "VK_RIGHT",
            VKey::Down => "VK_DOWN",
            VKey::Select => "VK_SELECT",
            VKey::Print => "VK_PRINT",
            VKey::Execute => "VK_EXECUTE",
            VKey::Snapshot => "VK_SNAPSHOT",
            VKey::Insert => "VK_INSERT",
            VKey::Delete => "VK_DELETE",
            VKey::Help => "VK_HELP",
            VKey::LWin => "VK_LWIN",
            VKey::RWin => "VK_RWIN",
            VKey::Apps => "VK_APPS",
            VKey::Sleep => "VK_SLEEP",
            VKey::Numpad0 => "VK_NUMPAD0",
            VKey::Numpad1 => "VK_NUMPAD1",
            VKey::Numpad2 => "VK_NUMPAD2",
            VKey::Numpad3 => "VK_NUMPAD3",
            VKey::Numpad4 => "VK_NUMPAD4",
            VKey::Numpad5 => "VK_NUMPAD5",
            VKey::Numpad6 => "VK_NUMPAD6",
            VKey::Numpad7 => "VK_NUMPAD7",
            VKey::Numpad8 => "VK_NUMPAD8",
            VKey::Numpad9 => "VK_NUMPAD9",
            VKey::Multiply => "VK_MULTIPLY",
            VKey::Add => "VK_ADD",
            VKey::Separator => "VK_SEPARATOR",
            VKey::Subtract => "VK_SUBTRACT",
            VKey::Decimal => "VK_DECIMAL",
            VKey::Divide => "VK_DIVIDE",
            VKey::F1 => "VK_F1",
            VKey::F2 => "VK_F2",
            VKey::F3 => "VK_F3",
            VKey::F4 => "VK_F4",
            VKey::F5 => "VK_F5",
            VKey::F6 => "VK_F6",
            VKey::F7 => "VK_F7",
            VKey::F8 => "VK_F8",
            VKey::F9 => "VK_F9",
            VKey::F10 => "VK_F10",
            VKey::F11 => "VK_F11",
            VKey::F12 => "VK_F12",
            VKey::F13 => "VK_F13",
            VKey::F14 => "VK_F14",
            VKey::F15 => "VK_F15",
            VKey::F16 => "VK_F16",
            VKey::F17 => "VK_F17",
            VKey::F18 => "VK_F18",
            VKey::F19 => "VK_F19",
            VKey::F20 => "VK_F20",
            VKey::F21 => "VK_F21",
            VKey::F22 => "VK_F22",
            VKey::F23 => "VK_F23",
            VKey::F24 => "VK_F24",
            VKey::Numlock => "VK_NUMLOCK",
            VKey::Scroll => "VK_SCROLL",
            VKey::LShift => "VK_LSHIFT",
            VKey::RShift => "VK_RSHIFT",
            VKey::LControl => "VK_LCONTROL",
            VKey::RControl => "VK_RCONTROL",
            VKey::LMenu => "VK_LMENU",
            VKey::RMenu => "VK_RMENU",
            VKey::BrowserBack => "VK_BROWSER_BACK",
            VKey::BrowserForward => "VK_BROWSER_FORWARD",
            VKey::BrowserRefresh => "VK_BROWSER_REFRESH",
            VKey::BrowserStop => "VK_BROWSER_STOP",
            VKey::BrowserSearch => "VK_BROWSER_SEARCH",
            VKey::BrowserFavorites => "VK_BROWSER_FAVORITES",
            VKey::BrowserHome => "VK_BROWSER_HOME",
            VKey::VolumeMute => "VK_VOLUME_MUTE",
            VKey::VolumeDown => "VK_VOLUME_DOWN",
            VKey::VolumeUp => "VK_VOLUME_UP",
            VKey::MediaNextTrack => "VK_MEDIA_NEXT_TRACK",
            VKey::MediaPrevTrack => "VK_MEDIA_PREV_TRACK",
            VKey::MediaStop => "VK_MEDIA_STOP",
            VKey::MediaPlayPause => "VK_MEDIA_PLAY_PAUSE",
            VKey::LaunchMail => "VK_LAUNCH_MAIL",
            VKey::LaunchMediaSelect => "VK_LAUNCH_MEDIA_SELECT",
            VKey::LaunchApp1 => "VK_LAUNCH_APP1",
            VKey::LaunchApp2 => "VK_LAUNCH_APP2",
            VKey::Oem1 => "VK_OEM_1",
            VKey::OemPlus => "VK_OEM_PLUS",
            VKey::OemComma => "VK_OEM_COMMA",
            VKey::OemMinus => "VK_OEM_MINUS",
            VKey::OemPeriod => "VK_OEM_PERIOD",
            VKey::Oem2 => "VK_OEM_2",
            VKey::Oem3 => "VK_OEM_3",
            VKey::Oem4 => "VK_OEM_4",
            VKey::Oem5 => "VK_OEM_5",
            VKey::Oem6 => "VK_OEM_6",
            VKey::Oem7 => "VK_OEM_7",
            VKey::Oem8 => "VK_OEM_8",
            VKey::Oem102 => "VK_OEM_102",
            VKey::Attn => "VK_ATTN",
            VKey::Crsel => "VK_CRSEL",
            VKey::Exsel => "VK_EXSEL",
            VKey::Play => "VK_PLAY",
            VKey::Zoom => "VK_ZOOM",
            VKey::Pa1 => "VK_PA1",
            VKey::OemClear => "VK_OEM_CLEAR",
            VKey::Vk0 => "VK_0",
            VKey::Vk1 => "VK_1",
            VKey::Vk2 => "VK_2",
            VKey::Vk3 => "VK_3",
            VKey::Vk4 => "VK_4",
            VKey::Vk5 => "VK_5",
            VKey::Vk6 => "VK_6",
            VKey::Vk7 => "VK_7",
            VKey::Vk8 => "VK_8",
            VKey::Vk9 => "VK_9",
            VKey::A => "VK_A",
            VKey::B => "VK_B",
            VKey::C => "VK_C",
            VKey::D => "VK_D",
            VKey::E => "VK_E",
            VKey::F => "VK_F",
            VKey::G => "VK_G",
            VKey::H => "VK_H",
            VKey::I => "VK_I",
            VKey::J => "VK_J",
            VKey::K => "VK_K",
            VKey::L => "VK_L",
            VKey::M => "VK_M",
            VKey::N => "VK_N",
            VKey::O => "VK_O",
            VKey::P => "VK_P",
            VKey::Q => "VK_Q",
            VKey::R => "VK_R",
            VKey::S => "VK_S",
            VKey::T => "VK_T",
            VKey::U => "VK_U",
            VKey::V => "VK_V",
            VKey::W => "VK_W",
            VKey::X => "VK_X",
            VKey::Y => "VK_Y",
            VKey::Z => "VK_Z",
            VKey::CustomKeyCode(vk) => return write!(f, "Custom({})", vk),
        };
        write!(f, "{}", name)
    }
}

impl std::str::FromStr for VKey {
    type Err = WHKError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VKey::from_keyname(s)
    }
}

impl PartialEq<VKey> for VKey {
    fn eq(&self, other: &VKey) -> bool {
        self.to_vk_code() == other.to_vk_code()
    }
}

impl Eq for VKey {}

impl Hash for VKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_vk_code().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_vk_code() {
        assert_eq!(VKey::Back.to_vk_code(), 0x08); // VK_BACK
        assert_eq!(VKey::Return.to_vk_code(), 0x0D); // VK_RETURN
        assert_eq!(VKey::Space.to_vk_code(), 0x20); // VK_SPACE
        assert_eq!(VKey::F12.to_vk_code(), 0x7B); // VK_F12
        assert_eq!(VKey::CustomKeyCode(1234).to_vk_code(), 1234); // Custom key
    }

    #[test]
    fn test_from_keyname() {
        assert_eq!(VKey::from_keyname("BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_keyname("VK_BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_keyname("RETURN").unwrap(), VKey::Return);
        assert_eq!(
            VKey::from_keyname("0x29").unwrap(),
            VKey::CustomKeyCode(0x29)
        );
        assert!(VKey::from_keyname("INVALID_KEY").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", VKey::Back), "VK_BACK");
        assert_eq!(format!("{}", VKey::Return), "VK_RETURN");
        assert_eq!(format!("{}", VKey::CustomKeyCode(1234)), "Custom(1234)");
    }

    #[test]
    fn test_from_str() {
        use std::str::FromStr;
        assert_eq!(VKey::from_str("BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_str("VK_BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_str("INVALID_KEY").is_err(), true);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(VKey::Back, VKey::Back); // Identical keys
        assert_eq!(VKey::CustomKeyCode(1234), VKey::CustomKeyCode(1234)); // Same custom key
        assert_ne!(VKey::CustomKeyCode(1234), VKey::CustomKeyCode(5678)); // Different custom keys
    }

    #[test]
    fn test_custom_keycode_range() {
        assert_eq!(VKey::CustomKeyCode(0).to_vk_code(), 0);
        assert_eq!(VKey::CustomKeyCode(65535).to_vk_code(), 65535); // Maximum value for u16
    }
}
