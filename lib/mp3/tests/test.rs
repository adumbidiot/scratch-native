extern crate mp3;

#[test]
fn parse_mp3(){
	mp3::parse(&std::fs::read("./tests/1.mp3").unwrap());
	println!("XXXXXXXXXX");
	mp3_2_parse();
}

fn mp3_2_parse(){
	mp3::parse(&std::fs::read("./tests/2.mp3").unwrap());
}