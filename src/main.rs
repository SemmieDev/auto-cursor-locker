use std::thread::sleep;
use std::time::Duration;
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{ClipCursor, GetClientRect, GetCursorInfo, GetCursorPos, WindowFromPoint, CURSORINFO};

fn main() {
    'mainLoop: loop {
        println!("Waiting for cursor to be hidden");

        if let Err(error) = wait_for_cursor_state(true) {
            eprintln!("Failed to wait for cursor to be hidden: {}", error);
            continue;
        }

        let mut cursor_pos = POINT::default();

        if let Err(error) = unsafe { GetCursorPos(&mut cursor_pos) } {
            eprintln!("Failed to get cursor position: {error}");
            continue;
        }

        let window = unsafe { WindowFromPoint(cursor_pos) };

        if window.is_invalid() {
            println!("Mouse not over a window");
            continue;
        }

        let mut client_rect = RECT::default();

        if let Err(error) = unsafe { GetClientRect(window, &mut client_rect) } {
            eprintln!("Failed to get client area: {error}");
            continue;
        }

        let mut upper_left = POINT::default();

        if !unsafe { ClientToScreen(window, &mut upper_left) }.as_bool() {
            eprintln!("Failed to get upper left corner");
            continue;
        }

        let clip_area = RECT {
            left: upper_left.x,
            top: upper_left.y,
            right: upper_left.x + client_rect.right,
            bottom: upper_left.y + client_rect.bottom
        };

        println!("Clipping cursor");
        
        while let Ok(hidden) = get_cursor_hidden() {
            if !hidden {
                break;
            }

            if let Err(error) = unsafe { ClipCursor(Some(&clip_area)) } {
                eprintln!("Failed to clip cursor: {error}");
                continue 'mainLoop;
            }

            sleep(Duration::from_micros(10));
        }
        
        if let Err(error) = unsafe { ClipCursor(None) } {
            eprintln!("Failed to unclip cursor: {error}");
            continue;
        }
        
        println!("Unclipped cursor");
    }
}

fn wait_for_cursor_state(hidden: bool)-> windows::core::Result<()> {
    loop {
        let cursor_hidden = get_cursor_hidden()?;

        if hidden == cursor_hidden {
            return Ok(());
        }

        sleep(Duration::from_millis(10));
    }
}

fn get_cursor_hidden() -> windows::core::Result<bool> {
    let mut cursor_info = CURSORINFO {
        cbSize: size_of::<CURSORINFO>() as u32,
        ..CURSORINFO::default()
    };

    unsafe { GetCursorInfo(&mut cursor_info) }?;

    Ok(cursor_info.flags.0 == 0)
}
