#[path = "set-1/hextobase64.rs"]
mod hextobase64;


fn main() {
	println!("Hello, world!");
	hextobase64::hex_to_base64("7162375123f");
}
