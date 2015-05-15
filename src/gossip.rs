// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use super::config::Attributes;
use super::config::Replica;
use super::config::RangeDescriptor;
use super::config::GCPolicy;
use super::config::AcctConfig;
use super::config::PermConfig;
use super::config::ZoneConfig;
use super::config::RangeTree;
use super::config::RangeTreeNode;
use super::config::Addr;
use super::config::StoreCapacity;
use super::config::NodeDescriptor;
use super::config::StoreDescriptor;

#[derive(Clone,Default)]
pub struct GossipRequest {
    // message fields
    node_id: ::std::option::Option<i32>,
    addr: ::protobuf::SingularPtrField<Addr>,
    l_addr: ::protobuf::SingularPtrField<Addr>,
    max_seq: ::std::option::Option<i64>,
    delta: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GossipRequest {
    pub fn new() -> GossipRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GossipRequest {
        static mut instance: ::protobuf::lazy::Lazy<GossipRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GossipRequest,
        };
        unsafe {
            instance.get(|| {
                GossipRequest {
                    node_id: ::std::option::Option::None,
                    addr: ::protobuf::SingularPtrField::none(),
                    l_addr: ::protobuf::SingularPtrField::none(),
                    max_seq: ::std::option::Option::None,
                    delta: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = ::std::option::Option::None;
    }

    pub fn has_node_id(&self) -> bool {
        self.node_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: i32) {
        self.node_id = ::std::option::Option::Some(v);
    }

    pub fn get_node_id<'a>(&self) -> i32 {
        self.node_id.unwrap_or(0)
    }

    // optional .cockroach.proto.Addr addr = 2;

    pub fn clear_addr(&mut self) {
        self.addr.clear();
    }

    pub fn has_addr(&self) -> bool {
        self.addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addr(&mut self, v: Addr) {
        self.addr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addr<'a>(&'a mut self) -> &'a mut Addr {
        if self.addr.is_none() {
            self.addr.set_default();
        };
        self.addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_addr(&mut self) -> Addr {
        self.addr.take().unwrap_or_else(|| Addr::new())
    }

    pub fn get_addr<'a>(&'a self) -> &'a Addr {
        self.addr.as_ref().unwrap_or_else(|| Addr::default_instance())
    }

    // optional .cockroach.proto.Addr l_addr = 3;

    pub fn clear_l_addr(&mut self) {
        self.l_addr.clear();
    }

    pub fn has_l_addr(&self) -> bool {
        self.l_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l_addr(&mut self, v: Addr) {
        self.l_addr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_l_addr<'a>(&'a mut self) -> &'a mut Addr {
        if self.l_addr.is_none() {
            self.l_addr.set_default();
        };
        self.l_addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_l_addr(&mut self) -> Addr {
        self.l_addr.take().unwrap_or_else(|| Addr::new())
    }

    pub fn get_l_addr<'a>(&'a self) -> &'a Addr {
        self.l_addr.as_ref().unwrap_or_else(|| Addr::default_instance())
    }

    // optional int64 max_seq = 4;

    pub fn clear_max_seq(&mut self) {
        self.max_seq = ::std::option::Option::None;
    }

    pub fn has_max_seq(&self) -> bool {
        self.max_seq.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_seq(&mut self, v: i64) {
        self.max_seq = ::std::option::Option::Some(v);
    }

    pub fn get_max_seq<'a>(&self) -> i64 {
        self.max_seq.unwrap_or(0)
    }

    // optional bytes delta = 5;

    pub fn clear_delta(&mut self) {
        self.delta.clear();
    }

    pub fn has_delta(&self) -> bool {
        self.delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta(&mut self, v: ::std::vec::Vec<u8>) {
        self.delta = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delta<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.delta.is_none() {
            self.delta.set_default();
        };
        self.delta.as_mut().unwrap()
    }

    // Take field
    pub fn take_delta(&mut self) -> ::std::vec::Vec<u8> {
        self.delta.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_delta<'a>(&'a self) -> &'a [u8] {
        match self.delta.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for GossipRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.node_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.addr.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.l_addr.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.max_seq = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.delta.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.node_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.addr.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.l_addr.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.max_seq.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.delta.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.addr.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.l_addr.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.max_seq {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.delta.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GossipRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GossipRequest {
    fn new() -> GossipRequest {
        GossipRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GossipRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "node_id",
                    GossipRequest::has_node_id,
                    GossipRequest::get_node_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "addr",
                    GossipRequest::has_addr,
                    GossipRequest::get_addr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "l_addr",
                    GossipRequest::has_l_addr,
                    GossipRequest::get_l_addr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "max_seq",
                    GossipRequest::has_max_seq,
                    GossipRequest::get_max_seq,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "delta",
                    GossipRequest::has_delta,
                    GossipRequest::get_delta,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GossipRequest>(
                    "GossipRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GossipRequest {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_addr();
        self.clear_l_addr();
        self.clear_max_seq();
        self.clear_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GossipRequest {
    fn eq(&self, other: &GossipRequest) -> bool {
        self.node_id == other.node_id &&
        self.addr == other.addr &&
        self.l_addr == other.l_addr &&
        self.max_seq == other.max_seq &&
        self.delta == other.delta &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GossipRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GossipResponse {
    // message fields
    delta: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    alternate: ::protobuf::SingularPtrField<Addr>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GossipResponse {
    pub fn new() -> GossipResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GossipResponse {
        static mut instance: ::protobuf::lazy::Lazy<GossipResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GossipResponse,
        };
        unsafe {
            instance.get(|| {
                GossipResponse {
                    delta: ::protobuf::SingularField::none(),
                    alternate: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes delta = 1;

    pub fn clear_delta(&mut self) {
        self.delta.clear();
    }

    pub fn has_delta(&self) -> bool {
        self.delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta(&mut self, v: ::std::vec::Vec<u8>) {
        self.delta = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delta<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.delta.is_none() {
            self.delta.set_default();
        };
        self.delta.as_mut().unwrap()
    }

    // Take field
    pub fn take_delta(&mut self) -> ::std::vec::Vec<u8> {
        self.delta.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_delta<'a>(&'a self) -> &'a [u8] {
        match self.delta.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .cockroach.proto.Addr alternate = 2;

    pub fn clear_alternate(&mut self) {
        self.alternate.clear();
    }

    pub fn has_alternate(&self) -> bool {
        self.alternate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alternate(&mut self, v: Addr) {
        self.alternate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alternate<'a>(&'a mut self) -> &'a mut Addr {
        if self.alternate.is_none() {
            self.alternate.set_default();
        };
        self.alternate.as_mut().unwrap()
    }

    // Take field
    pub fn take_alternate(&mut self) -> Addr {
        self.alternate.take().unwrap_or_else(|| Addr::new())
    }

    pub fn get_alternate<'a>(&'a self) -> &'a Addr {
        self.alternate.as_ref().unwrap_or_else(|| Addr::default_instance())
    }
}

impl ::protobuf::Message for GossipResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.delta.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.alternate.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.delta.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.alternate.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.delta.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.alternate.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GossipResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GossipResponse {
    fn new() -> GossipResponse {
        GossipResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GossipResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "delta",
                    GossipResponse::has_delta,
                    GossipResponse::get_delta,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "alternate",
                    GossipResponse::has_alternate,
                    GossipResponse::get_alternate,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GossipResponse>(
                    "GossipResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GossipResponse {
    fn clear(&mut self) {
        self.clear_delta();
        self.clear_alternate();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GossipResponse {
    fn eq(&self, other: &GossipResponse) -> bool {
        self.delta == other.delta &&
        self.alternate == other.alternate &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GossipResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1c, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8c, 0x01,
    0x0a, 0x0d, 0x47, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x0f, 0x0a, 0x07, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x23, 0x0a, 0x04, 0x61, 0x64, 0x64, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x41, 0x64, 0x64, 0x72, 0x12, 0x25, 0x0a, 0x06, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x12, 0x0f, 0x0a, 0x07,
    0x6d, 0x61, 0x78, 0x5f, 0x73, 0x65, 0x71, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x12, 0x0d, 0x0a,
    0x05, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x49, 0x0a, 0x0e,
    0x47, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0d,
    0x0a, 0x05, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x28, 0x0a,
    0x09, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x15, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x42, 0x07, 0x5a, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x4a, 0xc0, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1d, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x02, 0x07, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x03, 0x08, 0x17,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x05, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x05, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x05, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x05,
    0x14, 0x1b, 0x0a, 0x4d, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x14, 0x01, 0x1a, 0x41,
    0x20, 0x47, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x20, 0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x20, 0x52, 0x50, 0x43, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x15, 0x0a, 0x24, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x02, 0x1d, 0x1a, 0x17, 0x20, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x27, 0x73, 0x20, 0x49,
    0x44, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x11, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x1b, 0x1c, 0x0a, 0x30, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x2a, 0x1a, 0x23, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0c, 0x21, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0c, 0x28, 0x29, 0x0a, 0x8b, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x0f, 0x02, 0x2c, 0x1a, 0x7e, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x6e,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x6f, 0x64, 0x65,
    0x20, 0x28, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6b, 0x6c, 0x75, 0x64,
    0x67, 0x65, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x67, 0x6f,
    0x73, 0x73, 0x69, 0x70, 0x20, 0x74, 0x6f, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x20, 0x77, 0x68, 0x65,
    0x6e, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64,
    0x29, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x0b, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x21, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x2a, 0x2b, 0x0a, 0x40, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x11, 0x02, 0x1d, 0x1a, 0x33, 0x20, 0x4d, 0x61, 0x78, 0x69, 0x6d,
    0x75, 0x6d, 0x20, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x20, 0x66, 0x72, 0x6f,
    0x6d, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x65, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x11, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x11, 0x1b, 0x1c, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x13,
    0x02, 0x1b, 0x1a, 0x31, 0x20, 0x52, 0x65, 0x63, 0x69, 0x70, 0x72, 0x6f, 0x63, 0x61, 0x6c, 0x20,
    0x64, 0x65, 0x6c, 0x74, 0x61, 0x20, 0x6f, 0x66, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x67, 0x6f, 0x73,
    0x73, 0x69, 0x70, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x13, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x13, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x13, 0x19, 0x1a, 0x0a, 0x7c, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x18, 0x00, 0x1d, 0x01, 0x1a, 0x70, 0x20, 0x47, 0x6f, 0x73, 0x73,
    0x69, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x47, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x2e, 0x47, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x20, 0x52, 0x50,
    0x43, 0x2e, 0x0a, 0x20, 0x20, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x62, 0x65, 0x20, 0x6e, 0x69, 0x6c, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x41, 0x6c, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x74, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x18, 0x08, 0x16, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x1a, 0x02, 0x1b, 0x1a, 0x28, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64,
    0x20, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x27, 0x73, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1a, 0x19, 0x1a, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x1c, 0x02, 0x2f, 0x1a, 0x36, 0x20, 0x4e, 0x6f, 0x6e, 0x2d, 0x6e, 0x69, 0x6c, 0x20, 0x6d, 0x65,
    0x61, 0x6e, 0x73, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x1c, 0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x1c, 0x21, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x1c, 0x2d, 0x2e,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
