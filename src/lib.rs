extern crate indexmap;
extern crate num_traits;

pub mod floyd_warshall;
pub mod graph;
pub mod request;
pub mod response;

use crate::request::Request;
use crate::response::Response;

pub struct ExchangeRatePath {}

impl ExchangeRatePath {
    /// Create a new instance of ExchangeRatePath structure.
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let request = Self::form_request();
        let response = Self::process_request(request);
        Self::write_response(response);
    }

    fn form_request() -> Request {
        // Price update line:
        // <timestamp> <exchange> <source_currency> <destination_currency> <forward_factor> <backward_factor>

        // Exchange rate request line:
        // EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>

        Request::read_from_stdin()
    }

    fn process_request(request: Request) -> Response {
        Response {}
    }

    fn write_response(response: Response) {}
}
