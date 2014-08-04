extern crate web_framework;

mod custom_middleware;

fn main() {
	let mut server = web_framework::Server::new();

	let mut server = server.with_middleware_fn(custom_middleware::my_middleware);

	server.get("/", home_page);

	server.listen();
}

fn home_page<Q: web_framework::Request, P: web_framework::Response>(request: custom_middleware::CustomRequest<Q>, mut response: P) {
	use std::io::MemReader;

	println!("middleware returned: {}", request.get_dummy_value());

	response.set_body(MemReader::new("hello world".as_bytes().to_vec()));
}
