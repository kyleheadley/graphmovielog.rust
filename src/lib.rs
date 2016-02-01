use std::io::prelude::*;
use std::fs::File;

pub fn startframe(file: &mut File, title: &str, comment: Option<&str>) {
	write!(file, "[state]").unwrap();
	let mut lines = title.lines();
	if let Some(first) = lines.next() {
		write!(file, "{}", first).unwrap();
		for l in lines {
			write!(file, " {}", l).unwrap();
		}
	}
	writeln!(file, "").unwrap();
	if let Some(comment) = comment {
		writeln!(file, "{}", comment).unwrap();
	}
}


#[test]
fn it_works() {
	let mut f = File::create("testlogging.txt").unwrap();
	startframe(&mut f, &"first".to_string(), None);
	startframe(&mut f, "one\nmore", Some("then we're done"));
	startframe(&mut f, "this is it!", Some(" the
		last
		one!!!
		"));
	startframe(&mut f, "done", None);
}
