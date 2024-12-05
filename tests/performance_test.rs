use std::{env, path::PathBuf, thread, time::Duration};

use clipboard_next::{
	common::{RustImage, RustImageData},
	Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
	ContentFormat,
};

#[cfg(target_os = "macos")]
const TMP_PATH: &str = "/tmp/";
#[cfg(target_os = "windows")]
const TMP_PATH: &str = "C:\\Windows\\Temp\\";
#[cfg(all(
	unix,
	not(any(
		target_os = "macos",
		target_os = "ios",
		target_os = "android",
		target_os = "emscripten"
	))
))]
const TMP_PATH: &str = "/tmp/";

#[test]
fn test_performance() {
	check_clipboard_formats_every_5_seconds();
}

struct Manager {
	ctx: ClipboardContext,
}

impl Manager {
	pub fn new() -> Self {
		let ctx = ClipboardContext::new().unwrap();
		Manager { ctx }
	}
}

impl ClipboardHandler for Manager {
	fn on_clipboard_change(&mut self) {
		let home_dir: PathBuf = env::var_os("HOME")
			.or_else(|| env::var_os("USERPROFILE"))
			.map(PathBuf::from)
			.expect("Could not determine the user's home directory");

		let buf: PathBuf = home_dir.join("desktop/file.png");
		let path = buf.to_str().unwrap();

		let clipboard_context = ClipboardContext::new().unwrap();
		match clipboard_context.available_formats() {
			Ok(formats) => {
				println!("Available formats: {:?}", formats);
			}
			Err(e) => {
				println!("Error fetching formats: {}", e);
			}
		}
		if clipboard_context.has(ContentFormat::Image) {
			let image = clipboard_context.get_image();
			let _ = image
				.inspect(|v| println!("Ok {:?}", v.save_to_path(path)))
				.inspect_err(|e| println!("Err {}", e));
		}
	}
}

fn check_clipboard_formats_every_5_seconds() {
	let manager = Manager::new();

	let mut watcher = ClipboardWatcherContext::new().unwrap();

	let watcher_shutdown = watcher.add_handler(manager).get_shutdown_channel();

	watcher.start_watch();

	// loop {
	// 	thread::sleep(Duration::from_secs(5)); // Wait for 5 seconds

	// 	// Get available formats
	// 	match clipboard_context.available_formats() {
	// 		Ok(formats) => {
	// 			println!("Available formats: {:?}", formats);
	// 		}
	// 		Err(e) => {
	// 			println!("Error fetching formats: {}", e);
	// 		}
	// 	}
	// }
}
