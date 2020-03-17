#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod lib {
    #[derive(Copy, Clone)]
    #[repr(u16)]
    pub enum Colours {
        BLACK,
        BLUE,
        GREEN,
        CYAN,
        RED,
        MAGENTA,
        BROWN,
        LIGHTGRAY,
        GRAY,
        LIGHTBLUE,
        LIGHTGREEN,
        LIGHTCYAN,
        LIGHTRED,
        LIGHTMAGENTA,
        YELLOW,
        WHITE
    }

    mod detail;

// TODO cache handle

    pub fn push_colour(col: Colours) {
        let handle = detail::get_output_handle();
        detail::set_colour(handle, col);
    }

    pub fn pop_colour() {
        let handle = detail::get_output_handle();
        detail::set_colour(handle, Colours::WHITE);
    }
}
