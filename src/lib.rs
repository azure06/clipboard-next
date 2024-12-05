pub mod common;
mod platform;
pub use common::{ClipboardContent, ClipboardHandler, ContentFormat, Result, RustImageData};
pub use image::imageops::FilterType;
#[cfg(target_os = "linux")]
pub use platform::ClipboardContextX11Options;
pub use platform::{ClipboardContext, ClipboardWatcherContext, WatcherShutdown};

pub trait Clipboard: Send {
	/// Get all formats of the current content in the clipboard
	fn available_formats(&self) -> Result<Vec<String>>;

	fn has(&self, format: ContentFormat) -> bool;

	/// clear clipboard
	fn clear(&self) -> Result<()>;

	/// Get the data in the specified format in the clipboard as a byte array
	fn get_buffer(&self, format: &str) -> Result<Vec<u8>>;

	/// Get plain text content in the clipboard as string
	fn get_text(&self) -> Result<String>;

	/// Get the rich text content in the clipboard as string
	fn get_rich_text(&self) -> Result<String>;

	/// Get the html format content in the clipboard as string
	fn get_html(&self) -> Result<String>;

	fn get_image(&self) -> Result<RustImageData>;

	fn get_files(&self) -> Result<Vec<String>>;

	fn get(&self, formats: &[ContentFormat]) -> Result<Vec<ClipboardContent>>;

	fn set_buffer(&self, format: &str, buffer: Vec<u8>) -> Result<()>;

	fn set_text(&self, text: String) -> Result<()>;

	fn set_rich_text(&self, text: String) -> Result<()>;

	fn set_html(&self, html: String) -> Result<()>;

	fn set_image(&self, image: RustImageData) -> Result<()>;

	fn set_files(&self, files: Vec<String>) -> Result<()>;

	/// set image will clear clipboard
	fn set(&self, contents: Vec<ClipboardContent>) -> Result<()>;
}

pub trait ClipboardWatcher<T: ClipboardHandler>: Send {
	/// Add a clipboard change handler, you can add multiple handlers, the handler needs to implement the trait [`ClipboardHandler`]
	fn add_handler(&mut self, handler: T) -> &mut Self;

	/// Start monitoring clipboard changes, this is a blocking method, until the monitoring ends, or the stop method is called, so it is recommended to call it in a separate thread
	fn start_watch(&mut self);

	/// Get the channel to stop monitoring, you can stop monitoring through this channel
	fn get_shutdown_channel(&self) -> WatcherShutdown;
}

impl WatcherShutdown {
	/// stop watching
	pub fn stop(self) {
		drop(self);
	}
}
