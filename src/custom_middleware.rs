use web_framework;

pub struct CustomRequest<Rq> {
	original: Rq
}

impl<Rq: web_framework::Request> web_framework::Request for CustomRequest<Rq> {

}

impl<Rq> CustomRequest<Rq> {
	pub fn get_dummy_value(&self) -> int {
		5
	}
}

pub fn my_middleware<Rq: web_framework::Request, Rp: web_framework::Response>(request: Rq, response: Rp)
	-> (CustomRequest<Rq>, Rp)
{
	(CustomRequest { original: request }, response)
}
