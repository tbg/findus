#![allow(non_snake_case)]
extern crate protobuf;
extern crate hyper;

use std::env;
use std::io::Read;
use std::io;
use std::string::String;
use std::vec::Vec;

use protobuf::parse_from_bytes;
use protobuf::Message;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::header::Headers;
use hyper::mime::Mime;

mod api;
mod config;
mod data;
mod errors;

trait Request {
    fn mut_header(&mut self) -> &mut api::RequestHeader;
}

//impl Request for api::PutRequest {
    // TODO:
    // macro_rules! impl_trait { ($($ty:ty),*) => { $(impl Trait for $ty { fn foo(&self) -> … { … self.get_header() … } })* } }
    // impl_trait!(api::PutRequest, ...)
//    fn mut_header(&mut self) -> &mut api::RequestHeader {
//        self.mut_header()
//    }
//}

impl api::PutRequest {
    fn reply(&self) -> api::PutResponse {
        api::PutResponse::new()
    }
}

fn main() {
    let key = "COCKROACH_PORT";
    let addr = match env::var(key) {
        Ok(val) => val,
        Err(e)  => "tcp://localhost:8080".to_owned(),
    }.replace("tcp://", "http://");
    println!("connecting to {}", addr);

    let mut putArgs = api::PutRequest::new();
    let mut putResp = putArgs.reply();
    putArgs.mut_header().set_raft_id(1);
    putArgs.mut_header().set_user("root".to_owned());
    putArgs.mut_header().set_key(b"tkey".to_vec());

    let mut sender = HTTPSender::new(addr);

    let e = sender.send(
        &putArgs,
        &mut putResp,
    );

    println!("ts={}", putResp.get_header().get_timestamp().get_wall_time());
    println!("error={}", putResp.get_header().get_error().get_message());
}

trait KVSender {
    fn send(&mut self, &Message, &mut Response);
}

trait Response : Message {
    fn mut_header(&mut self) -> &mut api::ResponseHeader;
}

impl Response for api::PutResponse {
    fn mut_header(&mut self) -> &mut api::ResponseHeader {
        self.mut_header()
    }
    
}

pub enum SendError {
    IoError(io::Error),
    WireError(String),
}

struct HTTPSender {
    client: Client,
    addr: String,
}

impl HTTPSender {
    pub fn new(addr: String) -> HTTPSender {
        HTTPSender{
            client: hyper::Client::new(),
            addr: addr,
        }
    }
}


impl KVSender for HTTPSender {
    fn send(&mut self, args: &Message, reply: &mut Response) {
        let enc = args.write_to_bytes().unwrap();
        //reply.merge_from_bytes(&enc);

        let mut headers = Headers::new();
        headers.set(ContentType("application/x-protobuf".parse().unwrap()));
        let res = self.client.post(&self.addr)
            .body(&*enc) // or &enc[..]
            .headers(headers)
            .send();
        match res {
            Ok(mut resp) => match resp.status {
                StatusCode::Ok => {
                    // TODO check unmarshal error.
                    reply.merge_from(&mut protobuf::CodedInputStream::new(&mut resp));
                },
                _ => {
                    let mut err = errors::Error::new();
                    err.set_message("error unmarshaling".to_owned());
                    reply.mut_header().set_error(err)
                }
            },
            _ => {
                let mut err = errors::Error::new();
                err.set_message("request error".to_owned());
                reply.mut_header().set_error(err)
            },
        }
    }
}
