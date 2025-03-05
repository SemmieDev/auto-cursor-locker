use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{ClipCursor, GetClientRect, GetCursorInfo, GetForegroundWindow, CURSORINFO};

fn main() {
    loop {
        println!("Waiting for cursor to be hidden");

        if let Err(error) = wait_for_cursor_state(true) {
            eprintln!("Failed to wait for cursor to be hidden: {}", error);
            continue;
        }

        let foreground_window = unsafe { GetForegroundWindow() };

        if foreground_window.is_invalid() {
            eprintln!("No foreground window detected");
            continue;
        }

        let mut client_rect = RECT::default();

        if let Err(error) = unsafe { GetClientRect(foreground_window, &mut client_rect) } {
            eprintln!("Failed to get client area: {}", error);
            continue;
        }

        let mut upper_left = POINT {
            x: 0,
            y: 0
        };

        if !unsafe { ClientToScreen(foreground_window, &mut upper_left) }.as_bool() {
            eprintln!("Failed to get upper left corner");
            continue;
        }

        let clip_area = RECT {
            left: upper_left.x,
            top: upper_left.y,
            right: upper_left.x + client_rect.right,
            bottom: upper_left.y + client_rect.bottom
        };

        if let Err(error) = unsafe { ClipCursor(Some(&clip_area)) } {
            eprintln!("Failed to clip cursor: {}", error);
            continue;
        }
        
        println!("Clipped cursor");
        
        if let Err(error) = wait_for_cursor_state(false) {
            eprintln!("Failed to wait for cursor to be shown: {}", error);
            continue;
        }
        
        if let Err(error) = unsafe { ClipCursor(None) } {
            eprintln!("Failed to unclip cursor: {}", error);
            continue;
        }
        
        println!("Unclipped cursor");
    }
}

fn wait_for_cursor_state(hidden: bool)-> windows::core::Result<()> {
    loop {
        let mut cursor_info = CURSORINFO {
            cbSize: size_of::<CURSORINFO>() as u32,
            ..CURSORINFO::default()
        };

        unsafe { GetCursorInfo(&mut cursor_info) }?;

        if (hidden && cursor_info.flags.0 == 0) || (!hidden && cursor_info.flags.0 != 0) {
            return Ok(());
        }

        sleep(Duration::from_millis(10));
    }
}
