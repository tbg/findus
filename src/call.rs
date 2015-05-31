use protobuf::Message;

pub trait Request : Message {
    fn mut_header(&mut self) -> &mut ::api::RequestHeader;
    fn create_reply(&self) -> Box<Response>;
}

pub trait Response : Message {
    fn mut_header(&mut self) -> &mut ::api::ResponseHeader;
}

impl Request for ::api::PutRequest {
    fn mut_header(&mut self) -> &mut ::api::RequestHeader {
        self.mut_header()
    }
    fn create_reply(&self) -> Box<Response> {
        Box::new(::api::PutResponse::new()) as Box<Response>
    }
}

impl Response for ::api::PutResponse {
    fn mut_header(&mut self) -> &mut ::api::ResponseHeader {
        self.mut_header()
    }
}

pub struct Call {
    pub args:  Box<Request>,
    pub reply: Box<Response>,
}

impl Call {
    pub fn put() -> Call {
        let args = Box::new(::api::PutRequest::new()) as Box<Request>;
        let reply = args.create_reply();
        Call {
            args: args,
            reply: reply,
        }
    }
}

#[test]
fn create() {
    let mut c = Call::put();
}
