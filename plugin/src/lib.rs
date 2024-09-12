#[allow(warnings)]
mod bindings;

use std::u64;

use bindings::wasi::{
    http::{
        outgoing_handler::{self, OutgoingRequest},
        types::{self, Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam},
    },
    io::streams::StreamError,
};

struct Component;

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        if request.path_with_query().unwrap() != "/sse" {
            let resp = OutgoingResponse::new(Fields::new());
            resp.set_status_code(200).unwrap();
            let body = resp.body().unwrap();
            ResponseOutparam::set(response_out, Ok(resp));

            let out = body.write().unwrap();
            out.blocking_write_and_flush("ok".as_bytes()).unwrap();
            drop(out);

            OutgoingBody::finish(body, None).unwrap();
            return;
        }

        // read sse
        let req = OutgoingRequest::new(Fields::new());
        req.set_path_with_query(Some("/sse")).unwrap();
        req.set_method(&types::Method::Get).unwrap();
        req.set_scheme(Some(&types::Scheme::Http)).unwrap();
        req.set_authority(Some("127.0.0.1:10000")).unwrap();

        let future_response = outgoing_handler::handle(req, None).unwrap();
        future_response.subscribe().block();

        let resp_headers = Fields::new();
        resp_headers
            .append(
                &"Content-Type".to_string(),
                &"text/event-stream".as_bytes().to_vec(),
            )
            .unwrap();
        resp_headers
            .append(
                &"Cache-Control".to_string(),
                &"no-cache".as_bytes().to_vec(),
            )
            .unwrap();
        let resp = OutgoingResponse::new(resp_headers);
        resp.set_status_code(200).unwrap();
        let body = resp.body().unwrap();
        ResponseOutparam::set(response_out, Ok(resp));

        let out = body.write().unwrap();

        let incoming_response = future_response.get().unwrap().unwrap().unwrap();
        let incoming_body = incoming_response.consume().unwrap();

        let stream = incoming_body.stream().unwrap();
        loop {
            let chunk = match stream.blocking_read(u64::MAX) {
                Ok(value) => value,
                Err(StreamError::Closed) => break,
                Err(e) => panic!("read stream error: {}", e),
            };

            if chunk.len() > 0 {
                let val = std::str::from_utf8(&chunk).unwrap();
                println!("response val: {}", val);
            }
            out.blocking_write_and_flush(&chunk).unwrap();
        }
        drop(out);

        OutgoingBody::finish(body, None).unwrap();
    }
}

impl bindings::Guest for Component {
    fn on_init() {
        println!("on identity init");
    }

    fn on_init_async() {
        println!("on identity init async");
    }
}

bindings::export!(Component with_types_in bindings);
