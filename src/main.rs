#![allow(non_snake_case)]
extern crate protobuf;
extern crate hyper;

use std::io::Read;
use std::string::String;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::header::Headers;
use hyper::mime::Mime;
use protobuf::parse_from_bytes;
use protobuf::Message;

mod api;
mod config;
mod data;
mod errors;

struct Call {
    args: Box<api::RequestUnion>,
    reply: Box<api::ResponseUnion>,
    //post: fn() -> Option<errors::Error>,
}

fn main() {
    let mut gr = api::PutRequest::new();
    println!("{}", gr.has_header());
    gr.mut_header().set_raft_id(5);
    gr.mut_header().set_user_priority(10);
    let mut user = String::new();
    user.push_str("root");
    gr.mut_header().set_user(user);
    gr.mut_header().set_key(b"tkey".to_vec());
    let encoded_message: Vec<u8> = gr.write_to_bytes().unwrap();
    //println!("{}", String::from_utf8(encoded_message.clone()).unwrap());
    let gr2 = parse_from_bytes::<api::PutRequest>(&encoded_message).unwrap();
    //println!("{}", gr2.get_header().get_user_priority());

    let mut client = Client::new();
    let resp = client.send(&gr);

    let mut c: Call = Call{
        args: Box::new(api::RequestUnion::new()),
        reply: Box::new(api::ResponseUnion::new())
    };
    c.args.set_put(gr);
    c.reply.set_put(resp.clone().unwrap());
    let stderr = errors::Error::new();
    let ref err = match resp {
        Ok(ref resp) => resp.get_header().get_error(),
        Err(x) => &stderr,
    };
    println!("message: {}", err.get_message())
}

trait KVSender {
    fn send(&mut self, &Message) -> Result<api::PutResponse,bool>;
}

impl KVSender for Client {
    fn send(&mut self, msg: &Message) -> Result<api::PutResponse, bool> {
        let encoded_message: Vec<u8> = msg.write_to_bytes().unwrap();
        let mut body = String::new();
        let mut headers = Headers::new();
        let proto_mime: Mime = "application/x-protobuf".parse().unwrap();
        headers.set(ContentType(proto_mime));
        let s = String::from_utf8(encoded_message).unwrap();
        let mut resp = self.post("http://localhost:8080/kv/db/Put")
            .body(&s)
            .headers(headers)
            .send().unwrap();
        match resp.status {
            StatusCode::Ok =>  {
                println!("{}", "call ok!");
                resp.read_to_string(&mut body);
                Ok(parse_from_bytes::<api::PutResponse>(&body.into_bytes()).unwrap())
            },
            _ => {
                println!("{}", resp.status);
                Err(true)
            }
        }
    }
}
