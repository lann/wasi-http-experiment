use std::io::Write;

mod convert;
mod error;
mod wasi_http;
mod wasi_streams;

// TODO: figure out where to call `drop_*`

struct Echo;

impl bindings::handler::Handler for Echo {
    fn handle(
        request: bindings::handler::IncomingRequest,
        response_out: bindings::handler::ResponseOutparam,
    ) {
        let mut req = http::Request::try_from(wasi_http::IncomingRequest(request))
            .expect("request to be valid");
        let body = std::io::read_to_string(req.body_mut()).expect("read_to_string");

        let resp_headers = wasi_http::Fields::new(&[]);
        let resp = wasi_http::OutgoingResponse::new(200, resp_headers);

        let outparam = wasi_http::ResponseOutparam(response_out);
        outparam.set(&resp).unwrap();

        let mut out_stream = resp.into_stream().unwrap();
        out_stream.write_all(body.as_bytes()).unwrap();
        out_stream.finish(None);
    }
}

bindings::export!(Echo);
