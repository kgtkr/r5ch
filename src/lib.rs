extern crate crypto;
extern crate rustc_serialize;

mod utils;
pub mod dat;

pub struct Kakikomi {
	server: String,
	board: String,
	thread: String,
	name: String,
	mail: String,
	message: String,
}

pub enum Submit {
	Write,
	Consent,
}

pub struct KakikomiBbscgi {
	submit: String,
	cookie: Vec<String>,
}

pub struct NewThread {
	server: String,
	board: String,
	subject: String,
	name: String,
	mail: String,
	message: String,
}
