use protobuf::Message;

pub trait Request : Message {
    fn mut_header(&mut self) -> &mut ::api::RequestHeader;
    fn create_reply(&self) -> Box<Response>;
}

pub trait Response : Message {
    fn mut_header(&mut self) -> &mut ::api::ResponseHeader;
}

macro_rules! impl_response {
    ($($ty:ty),*) => {
        $(impl Response for $ty {
            fn mut_header(&mut self) -> &mut ::api::ResponseHeader {
                self.mut_header()
            }
        })*
    }
}

// Implement request for the given type, with the given constructor for the response.
// Would look more elegant if the response type were passed in instead of the constructor,
// but macro hygiene doesn't let you call the static method ::new() then.
macro_rules! impl_request {
    ($([$ta:ty, $cons:expr]),*) => {
        $(impl Request for $ta {
            fn mut_header(&mut self) -> &mut ::api::RequestHeader {
                self.mut_header()
            } 
            fn create_reply(&self) -> Box<Response> {
                Box::new($cons()) as Box<Response>
            }
        })*
    }
}

impl_request!(
    [::api::PutRequest, ::api::PutResponse::new],
    [::api::GetRequest, ::api::GetResponse::new]
);

impl_response!(::api::PutResponse, ::api::GetResponse);

pub struct Call {
    pub args:  Box<Request>,
    pub reply: Box<Response>,
}

impl Call {
    fn from_req(args: Box<Request>) -> Call {
        let reply = args.create_reply();
        Call {
            args: args,
            reply: reply,
        }
    }
    // TODO implement all of these.
    pub fn put() -> Call {
        Call::from_req(Box::new(::api::PutRequest::new()) as Box<Request>)
    }
    pub fn get() -> Call {
        Call::from_req(Box::new(::api::GetRequest::new()) as Box<Request>)
    }
}

#[test]
fn create() {
    Call::put();
}
