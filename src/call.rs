use protobuf::Message;

pub trait Request : Message {
    fn method(&self) -> &str;
    fn mut_header(&mut self) -> &mut ::api::RequestHeader;
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
        }
        )*
    }
}

// Implement request for the given type, with the given constructor for the response.
// Would look more elegant if the response type were passed in instead of the constructor,
// but macro hygiene doesn't let you call the static method ::new() then.
macro_rules! impl_request {
    ($([$nm:expr, $ta:ty]),*) => {
        $(impl Request for $ta {
            fn method(&self) -> &str {
                return $nm;
            }
            fn mut_header(&mut self) -> &mut ::api::RequestHeader {
                self.mut_header()
            } 
        })*
    }
}

impl_request!(
    ["Put", ::api::PutRequest],
    ["Get", ::api::GetRequest]
);

impl_response!(::api::PutResponse, ::api::GetResponse);

pub struct Call<Rq, Rs> {
    pub args:  Rq,
    pub reply: Rs,
}

impl<Rq, Rs> Call<Rq, Rs> {
    fn new(args: Rq, reply: Rs) -> Call<Rq, Rs> {
        Call::<Rq,Rs> {
            args: args,
            reply: reply,
        }
    }
}
impl<Rq, Rs> Call<Rq, Rs> where Rq : Request, Rs : Response {
    pub fn generic(self) -> Call<Box<Request>, Box<Response>> {
        Call::<_,_> {
            args: Box::new(self.args) as Box<Request>,
            reply: Box::new(self.reply) as Box<Response>,
        }
    }
}

pub fn put() -> Call<::api::PutRequest, ::api::PutResponse> {
    Call::new(::api::PutRequest::new(), ::api::PutResponse::new())
}

pub fn get() -> Call<::api::GetRequest, ::api::GetResponse> {
    Call::new(::api::GetRequest::new(), ::api::GetResponse::new())
}

#[test]
fn it_works() {
    let mut c = put();
}
