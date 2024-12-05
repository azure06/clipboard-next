use clipboard_next::{Clipboard, ClipboardContext};

fn main() {
	let ctx = ClipboardContext::new().unwrap();
	let types = ctx.available_formats().unwrap();
	println!("{:?}", types);

	let buffer = ctx.get_buffer("public.html").unwrap();

	let string = String::from_utf8(buffer).unwrap();

	println!("{}", string);
}
