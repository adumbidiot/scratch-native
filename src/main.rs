extern crate hyper;
extern crate hyper_tls;
extern crate http;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio;
extern crate futures;
extern crate tokio_process;
#[macro_use]
extern crate toml;

extern crate curl;

use hyper_tls::HttpsConnector;
use hyper::Client;
use hyper::rt::{self, Future};
use std::sync::Arc;

mod scratch;

fn main() {	
	let https = HttpsConnector::new(4).unwrap();
    let client = Arc::from(Client::builder()
		.build::<_, hyper::Body>(https));
	
	let mut api = scratch::api::Api::new();
	print!("Downlading Project... ");
	let mut project = api.get_project("239742347");
	println!("Done");
	
	return;
	
	
	//let project = scratch::get_project_json("239742347").expect("Could not get project");
	//println!("{:#?}", &project);
	
	print!("Generating Project... ");
	/*let work = scratch::generate_project(&client, &project, &std::path::PathBuf::from("projects"))
		.unwrap()
		.and_then(|_|{
			println!("Done");
			return scratch::build_project(&std::path::PathBuf::from("projects\\239742347"));
		})
		.and_then(|fut|{
			print!("Building Project... ");
			return fut;
		})
		.map(|_|{
			println!("Done");
		})
		.map_err(|err|{
			println!("Error: {:#?}", err);
		});
	
	rt::run(work);
	*/
}
