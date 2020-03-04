use winapi::um::winnt::HANDLE;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::wincon::SetConsoleTextAttribute;
use crate::Colours;

pub fn get_output_handle() -> HANDLE {
    unsafe {
        GetStdHandle(STD_OUTPUT_HANDLE)
    }
}

pub fn set_colour(handle: HANDLE, col: Colours) {
    unsafe {
        SetConsoleTextAttribute(handle, col);
    }
}