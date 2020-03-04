#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

enum Colours {
    BLACK = 0,
    BLUE = 1,
    GREEN = 2,
    CYAN = 3,
    RED = 4,
    MAGENTA = 5,
    BROWN = 6,
    LIGHTGRAY = 7,
    GRAY = 8,
    LIGHTBLUE = 9,
    LIGHTGREEN = 10,
    LIGHTCYAN = 11,
    LIGHTRED = 12,
    LIGHTMAGENTA = 13,
    YELLOW = 14,
    WHITE = 15
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