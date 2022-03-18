// colors
pub mod colors {
    pub const _CYAN: &str = "\x1b[36m";
    pub const _GREEN: &str = "\x1b[32m";
    pub const _BLUE: &str = "\x1b[34m";
    pub const _WHITE: &str = "\x1B[37m";
    pub const _RED: &str = "\x1b[31m";
    pub const _YELLOW: &str = "\x1B[32m";
    pub const _BOLD: &str = "\x1B[1m";
    pub const _RESET: &str = "\x1B[0m";
}

pub use self::colors::_CYAN as cyan;
pub use self::colors::_GREEN as green;
pub use self::colors::_BLUE as blue;
pub use self::colors::_WHITE as white;
pub use self::colors::_RED as red;
pub use self::colors::_YELLOW as yellow;
pub use self::colors::_BOLD as bold;
pub use self::colors::_RESET as reset;
