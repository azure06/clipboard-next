# clipboard-next

clipboard-next is a fork of [clipboard-rs](https://github.com/ChurchTao/clipboard-rs), a cross-platform library written in Rust for getting and setting the system-level clipboard content. It supports Linux, Windows, and MacOS.

## Examples

### All Usage Examples

[Examples](examples)

### Simple Read and Write

```rust
use clipboard_next::{Clipboard, ClipboardContext, ContentFormat};

fn main() {
	let ctx = ClipboardContext::new().unwrap();
	let types = ctx.available_formats().unwrap();
	println!("{:?}", types);

	let has_rtf = ctx.has(ContentFormat::Rtf);
	println!("has_rtf={}", has_rtf);

	let rtf = ctx.get_rich_text().unwrap_or("".to_string());

	println!("rtf={}", rtf);

	let has_html = ctx.has(ContentFormat::Html);
	println!("has_html={}", has_html);

	let html = ctx.get_html().unwrap_or("".to_string());

	println!("html={}", html);

	let content = ctx.get_text().unwrap_or("".to_string());

	println!("txt={}", content);
}

```

### Reading Images

```rust
use clipboard_next::{common::RustImage, Clipboard, ClipboardContext};

const TMP_PATH: &str = "/tmp/";

fn main() {
	let ctx = ClipboardContext::new().unwrap();
	let types = ctx.available_formats().unwrap();
	println!("{:?}", types);

	let img = ctx.get_image();

	match img {
		Ok(img) => {
			img.save_to_path(format!("{}test.png", TMP_PATH).as_str())
				.unwrap();

			let resize_img = img.thumbnail(300, 300).unwrap();

			resize_img
				.save_to_path(format!("{}test_thumbnail.png", TMP_PATH).as_str())
				.unwrap();
		}
		Err(err) => {
			println!("err={}", err);
		}
	}
}

```

### Reading Any Format

```rust
use clipboard_next::{Clipboard, ClipboardContext};

fn main() {
    let ctx = ClipboardContext::new().unwrap();
    let types = ctx.available_formats().unwrap();
    println!("{:?}", types);

    let buffer = ctx.get_buffer("public.html").unwrap();

    let string = String::from_utf8(buffer).unwrap();

    println!("{}", string);
}

```

### Listening to Clipboard Changes

```rust
use clipboard_next::{
	Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
};
use std::{thread, time::Duration};

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
		println!(
			"on_clipboard_change, txt = {}",
			self.ctx.get_text().unwrap()
		);
	}
}

fn main() {
	let manager = Manager::new();

	let mut watcher = ClipboardWatcherContext::new().unwrap();

	let watcher_shutdown = watcher.add_handler(manager).get_shutdown_channel();

	thread::spawn(move || {
		thread::sleep(Duration::from_secs(5));
		println!("stop watch!");
		watcher_shutdown.stop();
	});

	println!("start watch!");
	watcher.start_watch();
}


```

## X11 - Clipboard Read Timeout

By default, in X11 clipboard-next implements a read timeout of 500 ms. You can override or disable this timeout by creating **ClipboardContext** using `new_with_options`:

```rust
#[cfg(unix)]
fn setup_clipboard() -> ClipboardContext {
	ClipboardContext::new_with_options(ClipboardContextX11Options { read_timeout: None }).unwrap()
}

#[cfg(not(unix))]
fn setup_clipboard(ctx: &mut ClipboardContext) -> ClipboardContext{
	ClipboardContext::new().unwrap()
}
```

## Thanks

- Fork of [clipboard-rs](https://github.com/ChurchTao/clipboard-rs)
- API design is inspired by [electron](https://www.electronjs.org/en/docs/latest/api/clipboard)
- Linux part of the project code is referenced from [x11-clipboard](https://github.com/quininer/x11-clipboard/tree/master)
