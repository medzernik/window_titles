use winapi::{
    um::{
        winuser::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, IsWindowVisible, GetWindowThreadProcessId},
        winnt::LPWSTR
    },
    shared::{minwindef::{BOOL, LPARAM}, windef::HWND},
};

use crate::{ConnectionTrait, Result};

pub struct Connection;
impl ConnectionTrait for Connection {
    fn new() -> Result<Self> { Ok(Self) }
    fn window_titles(&self) -> Result<Vec<(u32,String)>> {
        let state: Box<Vec<(u32,String)>> = Box::new(Vec::new());
        let ptr = Box::into_raw(state);
        let state;
        unsafe {
            EnumWindows(Some(enumerate_windows), ptr as LPARAM);
            state = Box::from_raw(ptr);
        }
        Ok(*state)
    }
}

unsafe extern "system" fn enumerate_windows(window: HWND, state: LPARAM) -> BOOL {
    if IsWindowVisible(window) == 0 { return true.into() }
    let mut lpdw_process_id: u32 = 0;

    let state = state as *mut Vec<(u32,String)>;
    let mut length = GetWindowTextLengthW(window);
    if length == 0 { return true.into() }
    length = length + 1;
    let mut title: Vec<u16> = vec![0; length as usize];
    let textw = GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, length);
    if textw != 0 {
        if let Ok(title) = String::from_utf16(title[0..(textw as usize)].as_ref()) {
            GetWindowThreadProcessId(window, &mut lpdw_process_id);

            (*state).push((lpdw_process_id, title));
        }
    }
    true.into()
}
