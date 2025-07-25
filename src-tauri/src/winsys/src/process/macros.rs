#[macro_export]
macro_rules! close_handle {
    ($handle: expr) => {
        if !$handle.is_invalid() {
            let _ = unsafe { CloseHandle($handle) };
        }
    };
}

#[macro_export]
macro_rules! close_handle_option {
    ($handle:expr) => {{
        if let Some(handle) = $handle {
            if !handle.is_invalid() {
                let _ = unsafe { CloseHandle(handle) };
            }
        }
    }};
}
