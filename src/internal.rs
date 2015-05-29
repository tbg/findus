// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use super::api::ClientCmdID;
use super::api::RequestHeader;
use super::api::ResponseHeader;
use super::api::ContainsRequest;
use super::api::ContainsResponse;
use super::api::GetRequest;
use super::api::GetResponse;
use super::api::PutRequest;
use super::api::PutResponse;
use super::api::ConditionalPutRequest;
use super::api::ConditionalPutResponse;
use super::api::IncrementRequest;
use super::api::IncrementResponse;
use super::api::DeleteRequest;
use super::api::DeleteResponse;
use super::api::DeleteRangeRequest;
use super::api::DeleteRangeResponse;
use super::api::ScanRequest;
use super::api::ScanResponse;
use super::api::EndTransactionRequest;
use super::api::EndTransactionResponse;
use super::api::RequestUnion;
use super::api::ResponseUnion;
use super::api::BatchRequest;
use super::api::BatchResponse;
use super::api::AdminSplitRequest;
use super::api::AdminSplitResponse;
use super::api::AdminMergeRequest;
use super::api::AdminMergeResponse;
use super::api::ReadConsistencyType;
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
use super::data::Timestamp;
use super::data::Value;
use super::data::MVCCValue;
use super::data::KeyValue;
use super::data::RawKeyValue;
use super::data::StoreIdent;
use super::data::SplitTrigger;
use super::data::MergeTrigger;
use super::data::ChangeReplicasTrigger;
use super::data::InternalCommitTrigger;
use super::data::NodeList;
use super::data::Transaction;
use super::data::Lease;
use super::data::MVCCMetadata;
use super::data::GCMetadata;
use super::data::MVCCStats;
use super::data::ReplicaChangeType;
use super::data::IsolationType;
use super::data::TransactionStatus;

#[derive(Clone,Default)]
pub struct InternalRangeLookupRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    max_ranges: ::std::option::Option<i32>,
    ignore_intents: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalRangeLookupRequest {
    pub fn new() -> InternalRangeLookupRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalRangeLookupRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalRangeLookupRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalRangeLookupRequest,
        };
        unsafe {
            instance.get(|| {
                InternalRangeLookupRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    max_ranges: ::std::option::Option::None,
                    ignore_intents: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional int32 max_ranges = 2;

    pub fn clear_max_ranges(&mut self) {
        self.max_ranges = ::std::option::Option::None;
    }

    pub fn has_max_ranges(&self) -> bool {
        self.max_ranges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_ranges(&mut self, v: i32) {
        self.max_ranges = ::std::option::Option::Some(v);
    }

    pub fn get_max_ranges<'a>(&self) -> i32 {
        self.max_ranges.unwrap_or(0)
    }

    // optional bool ignore_intents = 3;

    pub fn clear_ignore_intents(&mut self) {
        self.ignore_intents = ::std::option::Option::None;
    }

    pub fn has_ignore_intents(&self) -> bool {
        self.ignore_intents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignore_intents(&mut self, v: bool) {
        self.ignore_intents = ::std::option::Option::Some(v);
    }

    pub fn get_ignore_intents<'a>(&self) -> bool {
        self.ignore_intents.unwrap_or(false)
    }
}

impl ::protobuf::Message for InternalRangeLookupRequest {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_ranges = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.ignore_intents = ::std::option::Option::Some(tmp);
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.max_ranges.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.ignore_intents.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.max_ranges {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.ignore_intents {
            try!(os.write_bool(3, v));
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
        ::std::any::TypeId::of::<InternalRangeLookupRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalRangeLookupRequest {
    fn new() -> InternalRangeLookupRequest {
        InternalRangeLookupRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalRangeLookupRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalRangeLookupRequest::has_header,
                    InternalRangeLookupRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_ranges",
                    InternalRangeLookupRequest::has_max_ranges,
                    InternalRangeLookupRequest::get_max_ranges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "ignore_intents",
                    InternalRangeLookupRequest::has_ignore_intents,
                    InternalRangeLookupRequest::get_ignore_intents,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalRangeLookupRequest>(
                    "InternalRangeLookupRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalRangeLookupRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_max_ranges();
        self.clear_ignore_intents();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalRangeLookupRequest {
    fn eq(&self, other: &InternalRangeLookupRequest) -> bool {
        self.header == other.header &&
        self.max_ranges == other.max_ranges &&
        self.ignore_intents == other.ignore_intents &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalRangeLookupRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalRangeLookupResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    ranges: ::protobuf::RepeatedField<RangeDescriptor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalRangeLookupResponse {
    pub fn new() -> InternalRangeLookupResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalRangeLookupResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalRangeLookupResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalRangeLookupResponse,
        };
        unsafe {
            instance.get(|| {
                InternalRangeLookupResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    ranges: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    // repeated .cockroach.proto.RangeDescriptor ranges = 2;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<RangeDescriptor>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<RangeDescriptor> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<RangeDescriptor> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges<'a>(&'a self) -> &'a [RangeDescriptor] {
        &self.ranges
    }
}

impl ::protobuf::Message for InternalRangeLookupResponse {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ranges.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.ranges.iter() {
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
        ::std::any::TypeId::of::<InternalRangeLookupResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalRangeLookupResponse {
    fn new() -> InternalRangeLookupResponse {
        InternalRangeLookupResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalRangeLookupResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalRangeLookupResponse::has_header,
                    InternalRangeLookupResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ranges",
                    InternalRangeLookupResponse::get_ranges,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalRangeLookupResponse>(
                    "InternalRangeLookupResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalRangeLookupResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalRangeLookupResponse {
    fn eq(&self, other: &InternalRangeLookupResponse) -> bool {
        self.header == other.header &&
        self.ranges == other.ranges &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalRangeLookupResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalHeartbeatTxnRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalHeartbeatTxnRequest {
    pub fn new() -> InternalHeartbeatTxnRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalHeartbeatTxnRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalHeartbeatTxnRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalHeartbeatTxnRequest,
        };
        unsafe {
            instance.get(|| {
                InternalHeartbeatTxnRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalHeartbeatTxnRequest {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalHeartbeatTxnRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalHeartbeatTxnRequest {
    fn new() -> InternalHeartbeatTxnRequest {
        InternalHeartbeatTxnRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalHeartbeatTxnRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalHeartbeatTxnRequest::has_header,
                    InternalHeartbeatTxnRequest::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalHeartbeatTxnRequest>(
                    "InternalHeartbeatTxnRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalHeartbeatTxnRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalHeartbeatTxnRequest {
    fn eq(&self, other: &InternalHeartbeatTxnRequest) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalHeartbeatTxnRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalHeartbeatTxnResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalHeartbeatTxnResponse {
    pub fn new() -> InternalHeartbeatTxnResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalHeartbeatTxnResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalHeartbeatTxnResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalHeartbeatTxnResponse,
        };
        unsafe {
            instance.get(|| {
                InternalHeartbeatTxnResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalHeartbeatTxnResponse {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalHeartbeatTxnResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalHeartbeatTxnResponse {
    fn new() -> InternalHeartbeatTxnResponse {
        InternalHeartbeatTxnResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalHeartbeatTxnResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalHeartbeatTxnResponse::has_header,
                    InternalHeartbeatTxnResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalHeartbeatTxnResponse>(
                    "InternalHeartbeatTxnResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalHeartbeatTxnResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalHeartbeatTxnResponse {
    fn eq(&self, other: &InternalHeartbeatTxnResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalHeartbeatTxnResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalGCRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    gc_meta: ::protobuf::SingularPtrField<GCMetadata>,
    keys: ::protobuf::RepeatedField<InternalGCRequest_GCKey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalGCRequest {
    pub fn new() -> InternalGCRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalGCRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalGCRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalGCRequest,
        };
        unsafe {
            instance.get(|| {
                InternalGCRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    gc_meta: ::protobuf::SingularPtrField::none(),
                    keys: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional .cockroach.proto.GCMetadata gc_meta = 2;

    pub fn clear_gc_meta(&mut self) {
        self.gc_meta.clear();
    }

    pub fn has_gc_meta(&self) -> bool {
        self.gc_meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_meta(&mut self, v: GCMetadata) {
        self.gc_meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gc_meta<'a>(&'a mut self) -> &'a mut GCMetadata {
        if self.gc_meta.is_none() {
            self.gc_meta.set_default();
        };
        self.gc_meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_gc_meta(&mut self) -> GCMetadata {
        self.gc_meta.take().unwrap_or_else(|| GCMetadata::new())
    }

    pub fn get_gc_meta<'a>(&'a self) -> &'a GCMetadata {
        self.gc_meta.as_ref().unwrap_or_else(|| GCMetadata::default_instance())
    }

    // repeated .cockroach.proto.InternalGCRequest.GCKey keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<InternalGCRequest_GCKey>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<InternalGCRequest_GCKey> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<InternalGCRequest_GCKey> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [InternalGCRequest_GCKey] {
        &self.keys
    }
}

impl ::protobuf::Message for InternalGCRequest {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.gc_meta.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.gc_meta.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.keys.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.gc_meta.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.keys.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalGCRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalGCRequest {
    fn new() -> InternalGCRequest {
        InternalGCRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalGCRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalGCRequest::has_header,
                    InternalGCRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gc_meta",
                    InternalGCRequest::has_gc_meta,
                    InternalGCRequest::get_gc_meta,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "keys",
                    InternalGCRequest::get_keys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalGCRequest>(
                    "InternalGCRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalGCRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_gc_meta();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalGCRequest {
    fn eq(&self, other: &InternalGCRequest) -> bool {
        self.header == other.header &&
        self.gc_meta == other.gc_meta &&
        self.keys == other.keys &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalGCRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalGCRequest_GCKey {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::protobuf::SingularPtrField<Timestamp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalGCRequest_GCKey {
    pub fn new() -> InternalGCRequest_GCKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalGCRequest_GCKey {
        static mut instance: ::protobuf::lazy::Lazy<InternalGCRequest_GCKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalGCRequest_GCKey,
        };
        unsafe {
            instance.get(|| {
                InternalGCRequest_GCKey {
                    key: ::protobuf::SingularField::none(),
                    timestamp: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .cockroach.proto.Timestamp timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: Timestamp) {
        self.timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestamp<'a>(&'a mut self) -> &'a mut Timestamp {
        if self.timestamp.is_none() {
            self.timestamp.set_default();
        };
        self.timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_timestamp(&mut self) -> Timestamp {
        self.timestamp.take().unwrap_or_else(|| Timestamp::new())
    }

    pub fn get_timestamp<'a>(&'a self) -> &'a Timestamp {
        self.timestamp.as_ref().unwrap_or_else(|| Timestamp::default_instance())
    }
}

impl ::protobuf::Message for InternalGCRequest_GCKey {
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
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.timestamp.set_default();
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.timestamp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.timestamp.as_ref() {
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
        ::std::any::TypeId::of::<InternalGCRequest_GCKey>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalGCRequest_GCKey {
    fn new() -> InternalGCRequest_GCKey {
        InternalGCRequest_GCKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalGCRequest_GCKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    InternalGCRequest_GCKey::has_key,
                    InternalGCRequest_GCKey::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "timestamp",
                    InternalGCRequest_GCKey::has_timestamp,
                    InternalGCRequest_GCKey::get_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalGCRequest_GCKey>(
                    "InternalGCRequest_GCKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalGCRequest_GCKey {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalGCRequest_GCKey {
    fn eq(&self, other: &InternalGCRequest_GCKey) -> bool {
        self.key == other.key &&
        self.timestamp == other.timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalGCRequest_GCKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalGCResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalGCResponse {
    pub fn new() -> InternalGCResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalGCResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalGCResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalGCResponse,
        };
        unsafe {
            instance.get(|| {
                InternalGCResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalGCResponse {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalGCResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalGCResponse {
    fn new() -> InternalGCResponse {
        InternalGCResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalGCResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalGCResponse::has_header,
                    InternalGCResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalGCResponse>(
                    "InternalGCResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalGCResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalGCResponse {
    fn eq(&self, other: &InternalGCResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalGCResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalPushTxnRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    pushee_txn: ::protobuf::SingularPtrField<Transaction>,
    push_type: ::std::option::Option<PushTxnType>,
    range_lookup: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalPushTxnRequest {
    pub fn new() -> InternalPushTxnRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalPushTxnRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalPushTxnRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalPushTxnRequest,
        };
        unsafe {
            instance.get(|| {
                InternalPushTxnRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    pushee_txn: ::protobuf::SingularPtrField::none(),
                    push_type: ::std::option::Option::None,
                    range_lookup: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional .cockroach.proto.Transaction pushee_txn = 2;

    pub fn clear_pushee_txn(&mut self) {
        self.pushee_txn.clear();
    }

    pub fn has_pushee_txn(&self) -> bool {
        self.pushee_txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pushee_txn(&mut self, v: Transaction) {
        self.pushee_txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pushee_txn<'a>(&'a mut self) -> &'a mut Transaction {
        if self.pushee_txn.is_none() {
            self.pushee_txn.set_default();
        };
        self.pushee_txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_pushee_txn(&mut self) -> Transaction {
        self.pushee_txn.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_pushee_txn<'a>(&'a self) -> &'a Transaction {
        self.pushee_txn.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }

    // optional .cockroach.proto.PushTxnType push_type = 3;

    pub fn clear_push_type(&mut self) {
        self.push_type = ::std::option::Option::None;
    }

    pub fn has_push_type(&self) -> bool {
        self.push_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_push_type(&mut self, v: PushTxnType) {
        self.push_type = ::std::option::Option::Some(v);
    }

    pub fn get_push_type<'a>(&self) -> PushTxnType {
        self.push_type.unwrap_or(PushTxnType::PUSH_TIMESTAMP)
    }

    // optional bool range_lookup = 4;

    pub fn clear_range_lookup(&mut self) {
        self.range_lookup = ::std::option::Option::None;
    }

    pub fn has_range_lookup(&self) -> bool {
        self.range_lookup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_lookup(&mut self, v: bool) {
        self.range_lookup = ::std::option::Option::Some(v);
    }

    pub fn get_range_lookup<'a>(&self) -> bool {
        self.range_lookup.unwrap_or(false)
    }
}

impl ::protobuf::Message for InternalPushTxnRequest {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pushee_txn.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.push_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.range_lookup = ::std::option::Option::Some(tmp);
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pushee_txn.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.push_type.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        if self.range_lookup.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pushee_txn.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.push_type {
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.range_lookup {
            try!(os.write_bool(4, v));
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
        ::std::any::TypeId::of::<InternalPushTxnRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalPushTxnRequest {
    fn new() -> InternalPushTxnRequest {
        InternalPushTxnRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalPushTxnRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalPushTxnRequest::has_header,
                    InternalPushTxnRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pushee_txn",
                    InternalPushTxnRequest::has_pushee_txn,
                    InternalPushTxnRequest::get_pushee_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "push_type",
                    InternalPushTxnRequest::has_push_type,
                    InternalPushTxnRequest::get_push_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "range_lookup",
                    InternalPushTxnRequest::has_range_lookup,
                    InternalPushTxnRequest::get_range_lookup,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalPushTxnRequest>(
                    "InternalPushTxnRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalPushTxnRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_pushee_txn();
        self.clear_push_type();
        self.clear_range_lookup();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalPushTxnRequest {
    fn eq(&self, other: &InternalPushTxnRequest) -> bool {
        self.header == other.header &&
        self.pushee_txn == other.pushee_txn &&
        self.push_type == other.push_type &&
        self.range_lookup == other.range_lookup &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalPushTxnRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalPushTxnResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    pushee_txn: ::protobuf::SingularPtrField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalPushTxnResponse {
    pub fn new() -> InternalPushTxnResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalPushTxnResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalPushTxnResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalPushTxnResponse,
        };
        unsafe {
            instance.get(|| {
                InternalPushTxnResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    pushee_txn: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    // optional .cockroach.proto.Transaction pushee_txn = 2;

    pub fn clear_pushee_txn(&mut self) {
        self.pushee_txn.clear();
    }

    pub fn has_pushee_txn(&self) -> bool {
        self.pushee_txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pushee_txn(&mut self, v: Transaction) {
        self.pushee_txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pushee_txn<'a>(&'a mut self) -> &'a mut Transaction {
        if self.pushee_txn.is_none() {
            self.pushee_txn.set_default();
        };
        self.pushee_txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_pushee_txn(&mut self) -> Transaction {
        self.pushee_txn.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_pushee_txn<'a>(&'a self) -> &'a Transaction {
        self.pushee_txn.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }
}

impl ::protobuf::Message for InternalPushTxnResponse {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pushee_txn.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pushee_txn.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pushee_txn.as_ref() {
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
        ::std::any::TypeId::of::<InternalPushTxnResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalPushTxnResponse {
    fn new() -> InternalPushTxnResponse {
        InternalPushTxnResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalPushTxnResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalPushTxnResponse::has_header,
                    InternalPushTxnResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pushee_txn",
                    InternalPushTxnResponse::has_pushee_txn,
                    InternalPushTxnResponse::get_pushee_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalPushTxnResponse>(
                    "InternalPushTxnResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalPushTxnResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_pushee_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalPushTxnResponse {
    fn eq(&self, other: &InternalPushTxnResponse) -> bool {
        self.header == other.header &&
        self.pushee_txn == other.pushee_txn &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalPushTxnResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalResolveIntentRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalResolveIntentRequest {
    pub fn new() -> InternalResolveIntentRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalResolveIntentRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalResolveIntentRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalResolveIntentRequest,
        };
        unsafe {
            instance.get(|| {
                InternalResolveIntentRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalResolveIntentRequest {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalResolveIntentRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalResolveIntentRequest {
    fn new() -> InternalResolveIntentRequest {
        InternalResolveIntentRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalResolveIntentRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalResolveIntentRequest::has_header,
                    InternalResolveIntentRequest::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalResolveIntentRequest>(
                    "InternalResolveIntentRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalResolveIntentRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalResolveIntentRequest {
    fn eq(&self, other: &InternalResolveIntentRequest) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalResolveIntentRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalResolveIntentResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalResolveIntentResponse {
    pub fn new() -> InternalResolveIntentResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalResolveIntentResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalResolveIntentResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalResolveIntentResponse,
        };
        unsafe {
            instance.get(|| {
                InternalResolveIntentResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalResolveIntentResponse {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalResolveIntentResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalResolveIntentResponse {
    fn new() -> InternalResolveIntentResponse {
        InternalResolveIntentResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalResolveIntentResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalResolveIntentResponse::has_header,
                    InternalResolveIntentResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalResolveIntentResponse>(
                    "InternalResolveIntentResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalResolveIntentResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalResolveIntentResponse {
    fn eq(&self, other: &InternalResolveIntentResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalResolveIntentResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalResolveIntentRangeRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalResolveIntentRangeRequest {
    pub fn new() -> InternalResolveIntentRangeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalResolveIntentRangeRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalResolveIntentRangeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalResolveIntentRangeRequest,
        };
        unsafe {
            instance.get(|| {
                InternalResolveIntentRangeRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalResolveIntentRangeRequest {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalResolveIntentRangeRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalResolveIntentRangeRequest {
    fn new() -> InternalResolveIntentRangeRequest {
        InternalResolveIntentRangeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalResolveIntentRangeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalResolveIntentRangeRequest::has_header,
                    InternalResolveIntentRangeRequest::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalResolveIntentRangeRequest>(
                    "InternalResolveIntentRangeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalResolveIntentRangeRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalResolveIntentRangeRequest {
    fn eq(&self, other: &InternalResolveIntentRangeRequest) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalResolveIntentRangeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalResolveIntentRangeResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalResolveIntentRangeResponse {
    pub fn new() -> InternalResolveIntentRangeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalResolveIntentRangeResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalResolveIntentRangeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalResolveIntentRangeResponse,
        };
        unsafe {
            instance.get(|| {
                InternalResolveIntentRangeResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalResolveIntentRangeResponse {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalResolveIntentRangeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalResolveIntentRangeResponse {
    fn new() -> InternalResolveIntentRangeResponse {
        InternalResolveIntentRangeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalResolveIntentRangeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalResolveIntentRangeResponse::has_header,
                    InternalResolveIntentRangeResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalResolveIntentRangeResponse>(
                    "InternalResolveIntentRangeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalResolveIntentRangeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalResolveIntentRangeResponse {
    fn eq(&self, other: &InternalResolveIntentRangeResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalResolveIntentRangeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalMergeRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalMergeRequest {
    pub fn new() -> InternalMergeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalMergeRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalMergeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalMergeRequest,
        };
        unsafe {
            instance.get(|| {
                InternalMergeRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional .cockroach.proto.Value value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: Value) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut Value {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> Value {
        self.value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        self.value.as_ref().unwrap_or_else(|| Value::default_instance())
    }
}

impl ::protobuf::Message for InternalMergeRequest {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.value.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.value.as_ref() {
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
        ::std::any::TypeId::of::<InternalMergeRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalMergeRequest {
    fn new() -> InternalMergeRequest {
        InternalMergeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalMergeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalMergeRequest::has_header,
                    InternalMergeRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    InternalMergeRequest::has_value,
                    InternalMergeRequest::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalMergeRequest>(
                    "InternalMergeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalMergeRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalMergeRequest {
    fn eq(&self, other: &InternalMergeRequest) -> bool {
        self.header == other.header &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalMergeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalMergeResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalMergeResponse {
    pub fn new() -> InternalMergeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalMergeResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalMergeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalMergeResponse,
        };
        unsafe {
            instance.get(|| {
                InternalMergeResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalMergeResponse {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalMergeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalMergeResponse {
    fn new() -> InternalMergeResponse {
        InternalMergeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalMergeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalMergeResponse::has_header,
                    InternalMergeResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalMergeResponse>(
                    "InternalMergeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalMergeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalMergeResponse {
    fn eq(&self, other: &InternalMergeResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalMergeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalTruncateLogRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    index: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalTruncateLogRequest {
    pub fn new() -> InternalTruncateLogRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalTruncateLogRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalTruncateLogRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalTruncateLogRequest,
        };
        unsafe {
            instance.get(|| {
                InternalTruncateLogRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional uint64 index = 2;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index<'a>(&self) -> u64 {
        self.index.unwrap_or(0)
    }
}

impl ::protobuf::Message for InternalTruncateLogRequest {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.index = ::std::option::Option::Some(tmp);
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.index.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.index {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<InternalTruncateLogRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalTruncateLogRequest {
    fn new() -> InternalTruncateLogRequest {
        InternalTruncateLogRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalTruncateLogRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalTruncateLogRequest::has_header,
                    InternalTruncateLogRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "index",
                    InternalTruncateLogRequest::has_index,
                    InternalTruncateLogRequest::get_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalTruncateLogRequest>(
                    "InternalTruncateLogRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalTruncateLogRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalTruncateLogRequest {
    fn eq(&self, other: &InternalTruncateLogRequest) -> bool {
        self.header == other.header &&
        self.index == other.index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalTruncateLogRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalTruncateLogResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalTruncateLogResponse {
    pub fn new() -> InternalTruncateLogResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalTruncateLogResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalTruncateLogResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalTruncateLogResponse,
        };
        unsafe {
            instance.get(|| {
                InternalTruncateLogResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalTruncateLogResponse {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalTruncateLogResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalTruncateLogResponse {
    fn new() -> InternalTruncateLogResponse {
        InternalTruncateLogResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalTruncateLogResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalTruncateLogResponse::has_header,
                    InternalTruncateLogResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalTruncateLogResponse>(
                    "InternalTruncateLogResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalTruncateLogResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalTruncateLogResponse {
    fn eq(&self, other: &InternalTruncateLogResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalTruncateLogResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalLeaderLeaseRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    lease: ::protobuf::SingularPtrField<Lease>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalLeaderLeaseRequest {
    pub fn new() -> InternalLeaderLeaseRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalLeaderLeaseRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalLeaderLeaseRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalLeaderLeaseRequest,
        };
        unsafe {
            instance.get(|| {
                InternalLeaderLeaseRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    lease: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional .cockroach.proto.Lease lease = 2;

    pub fn clear_lease(&mut self) {
        self.lease.clear();
    }

    pub fn has_lease(&self) -> bool {
        self.lease.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lease(&mut self, v: Lease) {
        self.lease = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lease<'a>(&'a mut self) -> &'a mut Lease {
        if self.lease.is_none() {
            self.lease.set_default();
        };
        self.lease.as_mut().unwrap()
    }

    // Take field
    pub fn take_lease(&mut self) -> Lease {
        self.lease.take().unwrap_or_else(|| Lease::new())
    }

    pub fn get_lease<'a>(&'a self) -> &'a Lease {
        self.lease.as_ref().unwrap_or_else(|| Lease::default_instance())
    }
}

impl ::protobuf::Message for InternalLeaderLeaseRequest {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.lease.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.lease.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.lease.as_ref() {
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
        ::std::any::TypeId::of::<InternalLeaderLeaseRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalLeaderLeaseRequest {
    fn new() -> InternalLeaderLeaseRequest {
        InternalLeaderLeaseRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalLeaderLeaseRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalLeaderLeaseRequest::has_header,
                    InternalLeaderLeaseRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "lease",
                    InternalLeaderLeaseRequest::has_lease,
                    InternalLeaderLeaseRequest::get_lease,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalLeaderLeaseRequest>(
                    "InternalLeaderLeaseRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalLeaderLeaseRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_lease();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalLeaderLeaseRequest {
    fn eq(&self, other: &InternalLeaderLeaseRequest) -> bool {
        self.header == other.header &&
        self.lease == other.lease &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalLeaderLeaseRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalLeaderLeaseResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalLeaderLeaseResponse {
    pub fn new() -> InternalLeaderLeaseResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalLeaderLeaseResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalLeaderLeaseResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalLeaderLeaseResponse,
        };
        unsafe {
            instance.get(|| {
                InternalLeaderLeaseResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }
}

impl ::protobuf::Message for InternalLeaderLeaseResponse {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalLeaderLeaseResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalLeaderLeaseResponse {
    fn new() -> InternalLeaderLeaseResponse {
        InternalLeaderLeaseResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalLeaderLeaseResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalLeaderLeaseResponse::has_header,
                    InternalLeaderLeaseResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalLeaderLeaseResponse>(
                    "InternalLeaderLeaseResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalLeaderLeaseResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalLeaderLeaseResponse {
    fn eq(&self, other: &InternalLeaderLeaseResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalLeaderLeaseResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalRequestUnion {
    // message fields
    // message oneof groups
    value: ::std::option::Option<InternalRequestUnion_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum InternalRequestUnion_oneof_value {
    contains(ContainsRequest),
    get(GetRequest),
    put(PutRequest),
    conditional_put(ConditionalPutRequest),
    increment(IncrementRequest),
    delete(DeleteRequest),
    delete_range(DeleteRangeRequest),
    scan(ScanRequest),
    end_transaction(EndTransactionRequest),
    internal_push_txn(InternalPushTxnRequest),
    internal_resolve_intent(InternalResolveIntentRequest),
    internal_resolve_intent_range(InternalResolveIntentRangeRequest),
}

impl InternalRequestUnion {
    pub fn new() -> InternalRequestUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalRequestUnion {
        static mut instance: ::protobuf::lazy::Lazy<InternalRequestUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalRequestUnion,
        };
        unsafe {
            instance.get(|| {
                InternalRequestUnion {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ContainsRequest contains = 1;

    pub fn clear_contains(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_contains(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_contains(&mut self, v: ContainsRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contains<'a>(&'a mut self) -> &'a mut ContainsRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(ContainsRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_contains(&mut self) -> ContainsRequest {
        if self.has_contains() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(v)) => v,
                _ => panic!(),
            }
        } else {
            ContainsRequest::new()
        }
    }

    pub fn get_contains<'a>(&'a self) -> &'a ContainsRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(ref v)) => v,
            _ => ContainsRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.GetRequest get = 2;

    pub fn clear_get(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_get(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get<'a>(&'a mut self) -> &'a mut GetRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(GetRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get(&mut self) -> GetRequest {
        if self.has_get() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(v)) => v,
                _ => panic!(),
            }
        } else {
            GetRequest::new()
        }
    }

    pub fn get_get<'a>(&'a self) -> &'a GetRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(ref v)) => v,
            _ => GetRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.PutRequest put = 3;

    pub fn clear_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put<'a>(&'a mut self) -> &'a mut PutRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(PutRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_put(&mut self) -> PutRequest {
        if self.has_put() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutRequest::new()
        }
    }

    pub fn get_put<'a>(&'a self) -> &'a PutRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(ref v)) => v,
            _ => PutRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.ConditionalPutRequest conditional_put = 4;

    pub fn clear_conditional_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_conditional_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_conditional_put(&mut self, v: ConditionalPutRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conditional_put<'a>(&'a mut self) -> &'a mut ConditionalPutRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(ConditionalPutRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_conditional_put(&mut self) -> ConditionalPutRequest {
        if self.has_conditional_put() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(v)) => v,
                _ => panic!(),
            }
        } else {
            ConditionalPutRequest::new()
        }
    }

    pub fn get_conditional_put<'a>(&'a self) -> &'a ConditionalPutRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(ref v)) => v,
            _ => ConditionalPutRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.IncrementRequest increment = 5;

    pub fn clear_increment(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: IncrementRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_increment<'a>(&'a mut self) -> &'a mut IncrementRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(IncrementRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_increment(&mut self) -> IncrementRequest {
        if self.has_increment() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(v)) => v,
                _ => panic!(),
            }
        } else {
            IncrementRequest::new()
        }
    }

    pub fn get_increment<'a>(&'a self) -> &'a IncrementRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(ref v)) => v,
            _ => IncrementRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRequest delete = 6;

    pub fn clear_delete(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete<'a>(&'a mut self) -> &'a mut DeleteRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(DeleteRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteRequest {
        if self.has_delete() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRequest::new()
        }
    }

    pub fn get_delete<'a>(&'a self) -> &'a DeleteRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(ref v)) => v,
            _ => DeleteRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRangeRequest delete_range = 7;

    pub fn clear_delete_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_range(&mut self, v: DeleteRangeRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_range<'a>(&'a mut self) -> &'a mut DeleteRangeRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(DeleteRangeRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_range(&mut self) -> DeleteRangeRequest {
        if self.has_delete_range() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeRequest::new()
        }
    }

    pub fn get_delete_range<'a>(&'a self) -> &'a DeleteRangeRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(ref v)) => v,
            _ => DeleteRangeRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.ScanRequest scan = 8;

    pub fn clear_scan(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_scan(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_scan(&mut self, v: ScanRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scan<'a>(&'a mut self) -> &'a mut ScanRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(ScanRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_scan(&mut self) -> ScanRequest {
        if self.has_scan() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(v)) => v,
                _ => panic!(),
            }
        } else {
            ScanRequest::new()
        }
    }

    pub fn get_scan<'a>(&'a self) -> &'a ScanRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(ref v)) => v,
            _ => ScanRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.EndTransactionRequest end_transaction = 9;

    pub fn clear_end_transaction(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_transaction(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_transaction(&mut self, v: EndTransactionRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_transaction<'a>(&'a mut self) -> &'a mut EndTransactionRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(EndTransactionRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_transaction(&mut self) -> EndTransactionRequest {
        if self.has_end_transaction() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(v)) => v,
                _ => panic!(),
            }
        } else {
            EndTransactionRequest::new()
        }
    }

    pub fn get_end_transaction<'a>(&'a self) -> &'a EndTransactionRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(ref v)) => v,
            _ => EndTransactionRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalPushTxnRequest internal_push_txn = 30;

    pub fn clear_internal_push_txn(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_push_txn(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_push_txn(&mut self, v: InternalPushTxnRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_push_txn<'a>(&'a mut self) -> &'a mut InternalPushTxnRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(InternalPushTxnRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_push_txn(&mut self) -> InternalPushTxnRequest {
        if self.has_internal_push_txn() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalPushTxnRequest::new()
        }
    }

    pub fn get_internal_push_txn<'a>(&'a self) -> &'a InternalPushTxnRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(ref v)) => v,
            _ => InternalPushTxnRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentRequest internal_resolve_intent = 31;

    pub fn clear_internal_resolve_intent(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent(&mut self, v: InternalResolveIntentRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent<'a>(&'a mut self) -> &'a mut InternalResolveIntentRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(InternalResolveIntentRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent(&mut self) -> InternalResolveIntentRequest {
        if self.has_internal_resolve_intent() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentRequest::new()
        }
    }

    pub fn get_internal_resolve_intent<'a>(&'a self) -> &'a InternalResolveIntentRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(ref v)) => v,
            _ => InternalResolveIntentRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentRangeRequest internal_resolve_intent_range = 32;

    pub fn clear_internal_resolve_intent_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent_range(&mut self, v: InternalResolveIntentRangeRequest) {
        self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent_range<'a>(&'a mut self) -> &'a mut InternalResolveIntentRangeRequest {
        if let ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(InternalResolveIntentRangeRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent_range(&mut self) -> InternalResolveIntentRangeRequest {
        if self.has_internal_resolve_intent_range() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentRangeRequest::new()
        }
    }

    pub fn get_internal_resolve_intent_range<'a>(&'a self) -> &'a InternalResolveIntentRangeRequest {
        match self.value {
            ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(ref v)) => v,
            _ => InternalResolveIntentRangeRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for InternalRequestUnion {
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
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::contains(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::get(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::put(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::conditional_put(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::increment(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::delete_range(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::scan(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::end_transaction(try!(is.read_message())));
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_push_txn(try!(is.read_message())));
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent(try!(is.read_message())));
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRequestUnion_oneof_value::internal_resolve_intent_range(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &InternalRequestUnion_oneof_value::contains(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::get(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::conditional_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::increment(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::delete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::scan(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::end_transaction(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::internal_push_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::internal_resolve_intent(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRequestUnion_oneof_value::internal_resolve_intent_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &InternalRequestUnion_oneof_value::contains(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::get(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::put(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::conditional_put(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::increment(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::delete(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::delete_range(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::scan(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::end_transaction(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::internal_push_txn(ref v) => {
                    try!(os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::internal_resolve_intent(ref v) => {
                    try!(os.write_tag(31, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRequestUnion_oneof_value::internal_resolve_intent_range(ref v) => {
                    try!(os.write_tag(32, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<InternalRequestUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalRequestUnion {
    fn new() -> InternalRequestUnion {
        InternalRequestUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalRequestUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contains",
                    InternalRequestUnion::has_contains,
                    InternalRequestUnion::get_contains,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get",
                    InternalRequestUnion::has_get,
                    InternalRequestUnion::get_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put",
                    InternalRequestUnion::has_put,
                    InternalRequestUnion::get_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "conditional_put",
                    InternalRequestUnion::has_conditional_put,
                    InternalRequestUnion::get_conditional_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "increment",
                    InternalRequestUnion::has_increment,
                    InternalRequestUnion::get_increment,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete",
                    InternalRequestUnion::has_delete,
                    InternalRequestUnion::get_delete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_range",
                    InternalRequestUnion::has_delete_range,
                    InternalRequestUnion::get_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scan",
                    InternalRequestUnion::has_scan,
                    InternalRequestUnion::get_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end_transaction",
                    InternalRequestUnion::has_end_transaction,
                    InternalRequestUnion::get_end_transaction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_push_txn",
                    InternalRequestUnion::has_internal_push_txn,
                    InternalRequestUnion::get_internal_push_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent",
                    InternalRequestUnion::has_internal_resolve_intent,
                    InternalRequestUnion::get_internal_resolve_intent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent_range",
                    InternalRequestUnion::has_internal_resolve_intent_range,
                    InternalRequestUnion::get_internal_resolve_intent_range,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalRequestUnion>(
                    "InternalRequestUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalRequestUnion {
    fn clear(&mut self) {
        self.clear_contains();
        self.clear_get();
        self.clear_put();
        self.clear_conditional_put();
        self.clear_increment();
        self.clear_delete();
        self.clear_delete_range();
        self.clear_scan();
        self.clear_end_transaction();
        self.clear_internal_push_txn();
        self.clear_internal_resolve_intent();
        self.clear_internal_resolve_intent_range();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalRequestUnion {
    fn eq(&self, other: &InternalRequestUnion) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalRequestUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalResponseUnion {
    // message fields
    // message oneof groups
    value: ::std::option::Option<InternalResponseUnion_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum InternalResponseUnion_oneof_value {
    contains(ContainsResponse),
    get(GetResponse),
    put(PutResponse),
    conditional_put(ConditionalPutResponse),
    increment(IncrementResponse),
    delete(DeleteResponse),
    delete_range(DeleteRangeResponse),
    scan(ScanResponse),
    end_transaction(EndTransactionResponse),
    internal_push_txn(InternalPushTxnResponse),
    internal_resolve_intent(InternalResolveIntentResponse),
    internal_resolve_intent_range(InternalResolveIntentRangeResponse),
}

impl InternalResponseUnion {
    pub fn new() -> InternalResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<InternalResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalResponseUnion,
        };
        unsafe {
            instance.get(|| {
                InternalResponseUnion {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ContainsResponse contains = 1;

    pub fn clear_contains(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_contains(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_contains(&mut self, v: ContainsResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contains<'a>(&'a mut self) -> &'a mut ContainsResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(ContainsResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_contains(&mut self) -> ContainsResponse {
        if self.has_contains() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(v)) => v,
                _ => panic!(),
            }
        } else {
            ContainsResponse::new()
        }
    }

    pub fn get_contains<'a>(&'a self) -> &'a ContainsResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(ref v)) => v,
            _ => ContainsResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.GetResponse get = 2;

    pub fn clear_get(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_get(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get<'a>(&'a mut self) -> &'a mut GetResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(GetResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get(&mut self) -> GetResponse {
        if self.has_get() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(v)) => v,
                _ => panic!(),
            }
        } else {
            GetResponse::new()
        }
    }

    pub fn get_get<'a>(&'a self) -> &'a GetResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(ref v)) => v,
            _ => GetResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.PutResponse put = 3;

    pub fn clear_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put<'a>(&'a mut self) -> &'a mut PutResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(PutResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_put(&mut self) -> PutResponse {
        if self.has_put() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutResponse::new()
        }
    }

    pub fn get_put<'a>(&'a self) -> &'a PutResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(ref v)) => v,
            _ => PutResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.ConditionalPutResponse conditional_put = 4;

    pub fn clear_conditional_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_conditional_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_conditional_put(&mut self, v: ConditionalPutResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conditional_put<'a>(&'a mut self) -> &'a mut ConditionalPutResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(ConditionalPutResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_conditional_put(&mut self) -> ConditionalPutResponse {
        if self.has_conditional_put() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(v)) => v,
                _ => panic!(),
            }
        } else {
            ConditionalPutResponse::new()
        }
    }

    pub fn get_conditional_put<'a>(&'a self) -> &'a ConditionalPutResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(ref v)) => v,
            _ => ConditionalPutResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.IncrementResponse increment = 5;

    pub fn clear_increment(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: IncrementResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_increment<'a>(&'a mut self) -> &'a mut IncrementResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(IncrementResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_increment(&mut self) -> IncrementResponse {
        if self.has_increment() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(v)) => v,
                _ => panic!(),
            }
        } else {
            IncrementResponse::new()
        }
    }

    pub fn get_increment<'a>(&'a self) -> &'a IncrementResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(ref v)) => v,
            _ => IncrementResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteResponse delete = 6;

    pub fn clear_delete(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete<'a>(&'a mut self) -> &'a mut DeleteResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(DeleteResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteResponse {
        if self.has_delete() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteResponse::new()
        }
    }

    pub fn get_delete<'a>(&'a self) -> &'a DeleteResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(ref v)) => v,
            _ => DeleteResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRangeResponse delete_range = 7;

    pub fn clear_delete_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_range(&mut self, v: DeleteRangeResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_range<'a>(&'a mut self) -> &'a mut DeleteRangeResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(DeleteRangeResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_range(&mut self) -> DeleteRangeResponse {
        if self.has_delete_range() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeResponse::new()
        }
    }

    pub fn get_delete_range<'a>(&'a self) -> &'a DeleteRangeResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(ref v)) => v,
            _ => DeleteRangeResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.ScanResponse scan = 8;

    pub fn clear_scan(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_scan(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_scan(&mut self, v: ScanResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scan<'a>(&'a mut self) -> &'a mut ScanResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(ScanResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_scan(&mut self) -> ScanResponse {
        if self.has_scan() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(v)) => v,
                _ => panic!(),
            }
        } else {
            ScanResponse::new()
        }
    }

    pub fn get_scan<'a>(&'a self) -> &'a ScanResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(ref v)) => v,
            _ => ScanResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.EndTransactionResponse end_transaction = 9;

    pub fn clear_end_transaction(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_transaction(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_transaction(&mut self, v: EndTransactionResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_transaction<'a>(&'a mut self) -> &'a mut EndTransactionResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(EndTransactionResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_transaction(&mut self) -> EndTransactionResponse {
        if self.has_end_transaction() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(v)) => v,
                _ => panic!(),
            }
        } else {
            EndTransactionResponse::new()
        }
    }

    pub fn get_end_transaction<'a>(&'a self) -> &'a EndTransactionResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(ref v)) => v,
            _ => EndTransactionResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalPushTxnResponse internal_push_txn = 30;

    pub fn clear_internal_push_txn(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_push_txn(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_push_txn(&mut self, v: InternalPushTxnResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_push_txn<'a>(&'a mut self) -> &'a mut InternalPushTxnResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(InternalPushTxnResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_push_txn(&mut self) -> InternalPushTxnResponse {
        if self.has_internal_push_txn() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalPushTxnResponse::new()
        }
    }

    pub fn get_internal_push_txn<'a>(&'a self) -> &'a InternalPushTxnResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(ref v)) => v,
            _ => InternalPushTxnResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentResponse internal_resolve_intent = 31;

    pub fn clear_internal_resolve_intent(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent(&mut self, v: InternalResolveIntentResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent<'a>(&'a mut self) -> &'a mut InternalResolveIntentResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(InternalResolveIntentResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent(&mut self) -> InternalResolveIntentResponse {
        if self.has_internal_resolve_intent() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentResponse::new()
        }
    }

    pub fn get_internal_resolve_intent<'a>(&'a self) -> &'a InternalResolveIntentResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(ref v)) => v,
            _ => InternalResolveIntentResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentRangeResponse internal_resolve_intent_range = 32;

    pub fn clear_internal_resolve_intent_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent_range(&mut self, v: InternalResolveIntentRangeResponse) {
        self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent_range<'a>(&'a mut self) -> &'a mut InternalResolveIntentRangeResponse {
        if let ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(InternalResolveIntentRangeResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent_range(&mut self) -> InternalResolveIntentRangeResponse {
        if self.has_internal_resolve_intent_range() {
            match self.value.take() {
                ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentRangeResponse::new()
        }
    }

    pub fn get_internal_resolve_intent_range<'a>(&'a self) -> &'a InternalResolveIntentRangeResponse {
        match self.value {
            ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(ref v)) => v,
            _ => InternalResolveIntentRangeResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for InternalResponseUnion {
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
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::contains(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::get(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::put(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::conditional_put(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::increment(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::delete_range(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::scan(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::end_transaction(try!(is.read_message())));
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_push_txn(try!(is.read_message())));
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent(try!(is.read_message())));
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalResponseUnion_oneof_value::internal_resolve_intent_range(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &InternalResponseUnion_oneof_value::contains(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::get(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::conditional_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::increment(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::delete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::scan(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::end_transaction(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::internal_push_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::internal_resolve_intent(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalResponseUnion_oneof_value::internal_resolve_intent_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &InternalResponseUnion_oneof_value::contains(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::get(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::put(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::conditional_put(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::increment(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::delete(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::delete_range(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::scan(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::end_transaction(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::internal_push_txn(ref v) => {
                    try!(os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::internal_resolve_intent(ref v) => {
                    try!(os.write_tag(31, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalResponseUnion_oneof_value::internal_resolve_intent_range(ref v) => {
                    try!(os.write_tag(32, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<InternalResponseUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalResponseUnion {
    fn new() -> InternalResponseUnion {
        InternalResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contains",
                    InternalResponseUnion::has_contains,
                    InternalResponseUnion::get_contains,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get",
                    InternalResponseUnion::has_get,
                    InternalResponseUnion::get_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put",
                    InternalResponseUnion::has_put,
                    InternalResponseUnion::get_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "conditional_put",
                    InternalResponseUnion::has_conditional_put,
                    InternalResponseUnion::get_conditional_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "increment",
                    InternalResponseUnion::has_increment,
                    InternalResponseUnion::get_increment,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete",
                    InternalResponseUnion::has_delete,
                    InternalResponseUnion::get_delete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_range",
                    InternalResponseUnion::has_delete_range,
                    InternalResponseUnion::get_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scan",
                    InternalResponseUnion::has_scan,
                    InternalResponseUnion::get_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end_transaction",
                    InternalResponseUnion::has_end_transaction,
                    InternalResponseUnion::get_end_transaction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_push_txn",
                    InternalResponseUnion::has_internal_push_txn,
                    InternalResponseUnion::get_internal_push_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent",
                    InternalResponseUnion::has_internal_resolve_intent,
                    InternalResponseUnion::get_internal_resolve_intent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent_range",
                    InternalResponseUnion::has_internal_resolve_intent_range,
                    InternalResponseUnion::get_internal_resolve_intent_range,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalResponseUnion>(
                    "InternalResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalResponseUnion {
    fn clear(&mut self) {
        self.clear_contains();
        self.clear_get();
        self.clear_put();
        self.clear_conditional_put();
        self.clear_increment();
        self.clear_delete();
        self.clear_delete_range();
        self.clear_scan();
        self.clear_end_transaction();
        self.clear_internal_push_txn();
        self.clear_internal_resolve_intent();
        self.clear_internal_resolve_intent_range();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalResponseUnion {
    fn eq(&self, other: &InternalResponseUnion) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalBatchRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    requests: ::protobuf::RepeatedField<InternalRequestUnion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalBatchRequest {
    pub fn new() -> InternalBatchRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalBatchRequest {
        static mut instance: ::protobuf::lazy::Lazy<InternalBatchRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalBatchRequest,
        };
        unsafe {
            instance.get(|| {
                InternalBatchRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    requests: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // repeated .cockroach.proto.InternalRequestUnion requests = 2;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<InternalRequestUnion>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<InternalRequestUnion> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<InternalRequestUnion> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests<'a>(&'a self) -> &'a [InternalRequestUnion] {
        &self.requests
    }
}

impl ::protobuf::Message for InternalBatchRequest {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.requests.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.requests.iter() {
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
        ::std::any::TypeId::of::<InternalBatchRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalBatchRequest {
    fn new() -> InternalBatchRequest {
        InternalBatchRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalBatchRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalBatchRequest::has_header,
                    InternalBatchRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "requests",
                    InternalBatchRequest::get_requests,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalBatchRequest>(
                    "InternalBatchRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalBatchRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_requests();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalBatchRequest {
    fn eq(&self, other: &InternalBatchRequest) -> bool {
        self.header == other.header &&
        self.requests == other.requests &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalBatchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalBatchResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    responses: ::protobuf::RepeatedField<InternalResponseUnion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalBatchResponse {
    pub fn new() -> InternalBatchResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalBatchResponse {
        static mut instance: ::protobuf::lazy::Lazy<InternalBatchResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalBatchResponse,
        };
        unsafe {
            instance.get(|| {
                InternalBatchResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    responses: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    // repeated .cockroach.proto.InternalResponseUnion responses = 2;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<InternalResponseUnion>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<InternalResponseUnion> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<InternalResponseUnion> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses<'a>(&'a self) -> &'a [InternalResponseUnion] {
        &self.responses
    }
}

impl ::protobuf::Message for InternalBatchResponse {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.responses.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.responses.iter() {
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
        ::std::any::TypeId::of::<InternalBatchResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalBatchResponse {
    fn new() -> InternalBatchResponse {
        InternalBatchResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalBatchResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    InternalBatchResponse::has_header,
                    InternalBatchResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "responses",
                    InternalBatchResponse::get_responses,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalBatchResponse>(
                    "InternalBatchResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalBatchResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalBatchResponse {
    fn eq(&self, other: &InternalBatchResponse) -> bool {
        self.header == other.header &&
        self.responses == other.responses &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalBatchResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReadWriteCmdResponse {
    // message fields
    // message oneof groups
    value: ::std::option::Option<ReadWriteCmdResponse_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum ReadWriteCmdResponse_oneof_value {
    put(PutResponse),
    conditional_put(ConditionalPutResponse),
    increment(IncrementResponse),
    delete(DeleteResponse),
    delete_range(DeleteRangeResponse),
    end_transaction(EndTransactionResponse),
    internal_heartbeat_txn(InternalHeartbeatTxnResponse),
    internal_push_txn(InternalPushTxnResponse),
    internal_resolve_intent(InternalResolveIntentResponse),
    internal_resolve_intent_range(InternalResolveIntentRangeResponse),
    internal_merge(InternalMergeResponse),
    internal_truncate_log(InternalTruncateLogResponse),
    internal_gc(InternalGCResponse),
    internal_leader_lease(InternalLeaderLeaseResponse),
}

impl ReadWriteCmdResponse {
    pub fn new() -> ReadWriteCmdResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadWriteCmdResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReadWriteCmdResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadWriteCmdResponse,
        };
        unsafe {
            instance.get(|| {
                ReadWriteCmdResponse {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.PutResponse put = 1;

    pub fn clear_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put<'a>(&'a mut self) -> &'a mut PutResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(PutResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_put(&mut self) -> PutResponse {
        if self.has_put() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutResponse::new()
        }
    }

    pub fn get_put<'a>(&'a self) -> &'a PutResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(ref v)) => v,
            _ => PutResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.ConditionalPutResponse conditional_put = 2;

    pub fn clear_conditional_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_conditional_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_conditional_put(&mut self, v: ConditionalPutResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conditional_put<'a>(&'a mut self) -> &'a mut ConditionalPutResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(ConditionalPutResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_conditional_put(&mut self) -> ConditionalPutResponse {
        if self.has_conditional_put() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(v)) => v,
                _ => panic!(),
            }
        } else {
            ConditionalPutResponse::new()
        }
    }

    pub fn get_conditional_put<'a>(&'a self) -> &'a ConditionalPutResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(ref v)) => v,
            _ => ConditionalPutResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.IncrementResponse increment = 3;

    pub fn clear_increment(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: IncrementResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_increment<'a>(&'a mut self) -> &'a mut IncrementResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(IncrementResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_increment(&mut self) -> IncrementResponse {
        if self.has_increment() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(v)) => v,
                _ => panic!(),
            }
        } else {
            IncrementResponse::new()
        }
    }

    pub fn get_increment<'a>(&'a self) -> &'a IncrementResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(ref v)) => v,
            _ => IncrementResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteResponse delete = 4;

    pub fn clear_delete(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete<'a>(&'a mut self) -> &'a mut DeleteResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(DeleteResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteResponse {
        if self.has_delete() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteResponse::new()
        }
    }

    pub fn get_delete<'a>(&'a self) -> &'a DeleteResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(ref v)) => v,
            _ => DeleteResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRangeResponse delete_range = 5;

    pub fn clear_delete_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_range(&mut self, v: DeleteRangeResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_range<'a>(&'a mut self) -> &'a mut DeleteRangeResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(DeleteRangeResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_range(&mut self) -> DeleteRangeResponse {
        if self.has_delete_range() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeResponse::new()
        }
    }

    pub fn get_delete_range<'a>(&'a self) -> &'a DeleteRangeResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(ref v)) => v,
            _ => DeleteRangeResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.EndTransactionResponse end_transaction = 6;

    pub fn clear_end_transaction(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_transaction(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_transaction(&mut self, v: EndTransactionResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_transaction<'a>(&'a mut self) -> &'a mut EndTransactionResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(EndTransactionResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_transaction(&mut self) -> EndTransactionResponse {
        if self.has_end_transaction() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(v)) => v,
                _ => panic!(),
            }
        } else {
            EndTransactionResponse::new()
        }
    }

    pub fn get_end_transaction<'a>(&'a self) -> &'a EndTransactionResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(ref v)) => v,
            _ => EndTransactionResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalHeartbeatTxnResponse internal_heartbeat_txn = 10;

    pub fn clear_internal_heartbeat_txn(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_heartbeat_txn(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_heartbeat_txn(&mut self, v: InternalHeartbeatTxnResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_heartbeat_txn<'a>(&'a mut self) -> &'a mut InternalHeartbeatTxnResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(InternalHeartbeatTxnResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_heartbeat_txn(&mut self) -> InternalHeartbeatTxnResponse {
        if self.has_internal_heartbeat_txn() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalHeartbeatTxnResponse::new()
        }
    }

    pub fn get_internal_heartbeat_txn<'a>(&'a self) -> &'a InternalHeartbeatTxnResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(ref v)) => v,
            _ => InternalHeartbeatTxnResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalPushTxnResponse internal_push_txn = 11;

    pub fn clear_internal_push_txn(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_push_txn(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_push_txn(&mut self, v: InternalPushTxnResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_push_txn<'a>(&'a mut self) -> &'a mut InternalPushTxnResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(InternalPushTxnResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_push_txn(&mut self) -> InternalPushTxnResponse {
        if self.has_internal_push_txn() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalPushTxnResponse::new()
        }
    }

    pub fn get_internal_push_txn<'a>(&'a self) -> &'a InternalPushTxnResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(ref v)) => v,
            _ => InternalPushTxnResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentResponse internal_resolve_intent = 12;

    pub fn clear_internal_resolve_intent(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent(&mut self, v: InternalResolveIntentResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent<'a>(&'a mut self) -> &'a mut InternalResolveIntentResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(InternalResolveIntentResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent(&mut self) -> InternalResolveIntentResponse {
        if self.has_internal_resolve_intent() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentResponse::new()
        }
    }

    pub fn get_internal_resolve_intent<'a>(&'a self) -> &'a InternalResolveIntentResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(ref v)) => v,
            _ => InternalResolveIntentResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentRangeResponse internal_resolve_intent_range = 13;

    pub fn clear_internal_resolve_intent_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent_range(&mut self, v: InternalResolveIntentRangeResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent_range<'a>(&'a mut self) -> &'a mut InternalResolveIntentRangeResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(InternalResolveIntentRangeResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent_range(&mut self) -> InternalResolveIntentRangeResponse {
        if self.has_internal_resolve_intent_range() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentRangeResponse::new()
        }
    }

    pub fn get_internal_resolve_intent_range<'a>(&'a self) -> &'a InternalResolveIntentRangeResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(ref v)) => v,
            _ => InternalResolveIntentRangeResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalMergeResponse internal_merge = 14;

    pub fn clear_internal_merge(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_merge(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_merge(&mut self, v: InternalMergeResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_merge<'a>(&'a mut self) -> &'a mut InternalMergeResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(InternalMergeResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_merge(&mut self) -> InternalMergeResponse {
        if self.has_internal_merge() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalMergeResponse::new()
        }
    }

    pub fn get_internal_merge<'a>(&'a self) -> &'a InternalMergeResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(ref v)) => v,
            _ => InternalMergeResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalTruncateLogResponse internal_truncate_log = 15;

    pub fn clear_internal_truncate_log(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_truncate_log(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_truncate_log(&mut self, v: InternalTruncateLogResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_truncate_log<'a>(&'a mut self) -> &'a mut InternalTruncateLogResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(InternalTruncateLogResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_truncate_log(&mut self) -> InternalTruncateLogResponse {
        if self.has_internal_truncate_log() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalTruncateLogResponse::new()
        }
    }

    pub fn get_internal_truncate_log<'a>(&'a self) -> &'a InternalTruncateLogResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(ref v)) => v,
            _ => InternalTruncateLogResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalGCResponse internal_gc = 16;

    pub fn clear_internal_gc(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_gc(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_gc(&mut self, v: InternalGCResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_gc<'a>(&'a mut self) -> &'a mut InternalGCResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(InternalGCResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_gc(&mut self) -> InternalGCResponse {
        if self.has_internal_gc() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalGCResponse::new()
        }
    }

    pub fn get_internal_gc<'a>(&'a self) -> &'a InternalGCResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(ref v)) => v,
            _ => InternalGCResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalLeaderLeaseResponse internal_leader_lease = 17;

    pub fn clear_internal_leader_lease(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_leader_lease(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_leader_lease(&mut self, v: InternalLeaderLeaseResponse) {
        self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_leader_lease<'a>(&'a mut self) -> &'a mut InternalLeaderLeaseResponse {
        if let ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(InternalLeaderLeaseResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_leader_lease(&mut self) -> InternalLeaderLeaseResponse {
        if self.has_internal_leader_lease() {
            match self.value.take() {
                ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalLeaderLeaseResponse::new()
        }
    }

    pub fn get_internal_leader_lease<'a>(&'a self) -> &'a InternalLeaderLeaseResponse {
        match self.value {
            ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(ref v)) => v,
            _ => InternalLeaderLeaseResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for ReadWriteCmdResponse {
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
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::put(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::conditional_put(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::increment(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::delete_range(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::end_transaction(try!(is.read_message())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(try!(is.read_message())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_push_txn(try!(is.read_message())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent(try!(is.read_message())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(try!(is.read_message())));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_merge(try!(is.read_message())));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_truncate_log(try!(is.read_message())));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_gc(try!(is.read_message())));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ReadWriteCmdResponse_oneof_value::internal_leader_lease(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &ReadWriteCmdResponse_oneof_value::put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::conditional_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::increment(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::delete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::end_transaction(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_push_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_resolve_intent(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_merge(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_truncate_log(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_gc(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ReadWriteCmdResponse_oneof_value::internal_leader_lease(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &ReadWriteCmdResponse_oneof_value::put(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::conditional_put(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::increment(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::delete(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::delete_range(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::end_transaction(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_heartbeat_txn(ref v) => {
                    try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_push_txn(ref v) => {
                    try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_resolve_intent(ref v) => {
                    try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_resolve_intent_range(ref v) => {
                    try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_merge(ref v) => {
                    try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_truncate_log(ref v) => {
                    try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_gc(ref v) => {
                    try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ReadWriteCmdResponse_oneof_value::internal_leader_lease(ref v) => {
                    try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<ReadWriteCmdResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadWriteCmdResponse {
    fn new() -> ReadWriteCmdResponse {
        ReadWriteCmdResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadWriteCmdResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put",
                    ReadWriteCmdResponse::has_put,
                    ReadWriteCmdResponse::get_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "conditional_put",
                    ReadWriteCmdResponse::has_conditional_put,
                    ReadWriteCmdResponse::get_conditional_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "increment",
                    ReadWriteCmdResponse::has_increment,
                    ReadWriteCmdResponse::get_increment,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete",
                    ReadWriteCmdResponse::has_delete,
                    ReadWriteCmdResponse::get_delete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_range",
                    ReadWriteCmdResponse::has_delete_range,
                    ReadWriteCmdResponse::get_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end_transaction",
                    ReadWriteCmdResponse::has_end_transaction,
                    ReadWriteCmdResponse::get_end_transaction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_heartbeat_txn",
                    ReadWriteCmdResponse::has_internal_heartbeat_txn,
                    ReadWriteCmdResponse::get_internal_heartbeat_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_push_txn",
                    ReadWriteCmdResponse::has_internal_push_txn,
                    ReadWriteCmdResponse::get_internal_push_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent",
                    ReadWriteCmdResponse::has_internal_resolve_intent,
                    ReadWriteCmdResponse::get_internal_resolve_intent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent_range",
                    ReadWriteCmdResponse::has_internal_resolve_intent_range,
                    ReadWriteCmdResponse::get_internal_resolve_intent_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_merge",
                    ReadWriteCmdResponse::has_internal_merge,
                    ReadWriteCmdResponse::get_internal_merge,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_truncate_log",
                    ReadWriteCmdResponse::has_internal_truncate_log,
                    ReadWriteCmdResponse::get_internal_truncate_log,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_gc",
                    ReadWriteCmdResponse::has_internal_gc,
                    ReadWriteCmdResponse::get_internal_gc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_leader_lease",
                    ReadWriteCmdResponse::has_internal_leader_lease,
                    ReadWriteCmdResponse::get_internal_leader_lease,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadWriteCmdResponse>(
                    "ReadWriteCmdResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadWriteCmdResponse {
    fn clear(&mut self) {
        self.clear_put();
        self.clear_conditional_put();
        self.clear_increment();
        self.clear_delete();
        self.clear_delete_range();
        self.clear_end_transaction();
        self.clear_internal_heartbeat_txn();
        self.clear_internal_push_txn();
        self.clear_internal_resolve_intent();
        self.clear_internal_resolve_intent_range();
        self.clear_internal_merge();
        self.clear_internal_truncate_log();
        self.clear_internal_gc();
        self.clear_internal_leader_lease();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReadWriteCmdResponse {
    fn eq(&self, other: &ReadWriteCmdResponse) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReadWriteCmdResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalRaftCommandUnion {
    // message fields
    // message oneof groups
    value: ::std::option::Option<InternalRaftCommandUnion_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum InternalRaftCommandUnion_oneof_value {
    contains(ContainsRequest),
    get(GetRequest),
    put(PutRequest),
    conditional_put(ConditionalPutRequest),
    increment(IncrementRequest),
    delete(DeleteRequest),
    delete_range(DeleteRangeRequest),
    scan(ScanRequest),
    end_transaction(EndTransactionRequest),
    batch(BatchRequest),
    internal_range_lookup(InternalRangeLookupRequest),
    internal_heartbeat_txn(InternalHeartbeatTxnRequest),
    internal_push_txn(InternalPushTxnRequest),
    internal_resolve_intent(InternalResolveIntentRequest),
    internal_resolve_intent_range(InternalResolveIntentRangeRequest),
    internal_merge_response(InternalMergeRequest),
    internal_truncate_log(InternalTruncateLogRequest),
    internal_gc(InternalGCRequest),
    internal_lease(InternalLeaderLeaseRequest),
    internal_batch(InternalBatchRequest),
}

impl InternalRaftCommandUnion {
    pub fn new() -> InternalRaftCommandUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalRaftCommandUnion {
        static mut instance: ::protobuf::lazy::Lazy<InternalRaftCommandUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalRaftCommandUnion,
        };
        unsafe {
            instance.get(|| {
                InternalRaftCommandUnion {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.ContainsRequest contains = 1;

    pub fn clear_contains(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_contains(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_contains(&mut self, v: ContainsRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contains<'a>(&'a mut self) -> &'a mut ContainsRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(ContainsRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_contains(&mut self) -> ContainsRequest {
        if self.has_contains() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(v)) => v,
                _ => panic!(),
            }
        } else {
            ContainsRequest::new()
        }
    }

    pub fn get_contains<'a>(&'a self) -> &'a ContainsRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(ref v)) => v,
            _ => ContainsRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.GetRequest get = 2;

    pub fn clear_get(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_get(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get<'a>(&'a mut self) -> &'a mut GetRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(GetRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get(&mut self) -> GetRequest {
        if self.has_get() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(v)) => v,
                _ => panic!(),
            }
        } else {
            GetRequest::new()
        }
    }

    pub fn get_get<'a>(&'a self) -> &'a GetRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(ref v)) => v,
            _ => GetRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.PutRequest put = 3;

    pub fn clear_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put<'a>(&'a mut self) -> &'a mut PutRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(PutRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_put(&mut self) -> PutRequest {
        if self.has_put() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutRequest::new()
        }
    }

    pub fn get_put<'a>(&'a self) -> &'a PutRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(ref v)) => v,
            _ => PutRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.ConditionalPutRequest conditional_put = 4;

    pub fn clear_conditional_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_conditional_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_conditional_put(&mut self, v: ConditionalPutRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conditional_put<'a>(&'a mut self) -> &'a mut ConditionalPutRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(ConditionalPutRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_conditional_put(&mut self) -> ConditionalPutRequest {
        if self.has_conditional_put() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(v)) => v,
                _ => panic!(),
            }
        } else {
            ConditionalPutRequest::new()
        }
    }

    pub fn get_conditional_put<'a>(&'a self) -> &'a ConditionalPutRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(ref v)) => v,
            _ => ConditionalPutRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.IncrementRequest increment = 5;

    pub fn clear_increment(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: IncrementRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_increment<'a>(&'a mut self) -> &'a mut IncrementRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(IncrementRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_increment(&mut self) -> IncrementRequest {
        if self.has_increment() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(v)) => v,
                _ => panic!(),
            }
        } else {
            IncrementRequest::new()
        }
    }

    pub fn get_increment<'a>(&'a self) -> &'a IncrementRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(ref v)) => v,
            _ => IncrementRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRequest delete = 6;

    pub fn clear_delete(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete<'a>(&'a mut self) -> &'a mut DeleteRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(DeleteRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteRequest {
        if self.has_delete() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRequest::new()
        }
    }

    pub fn get_delete<'a>(&'a self) -> &'a DeleteRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(ref v)) => v,
            _ => DeleteRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRangeRequest delete_range = 7;

    pub fn clear_delete_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_range(&mut self, v: DeleteRangeRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_range<'a>(&'a mut self) -> &'a mut DeleteRangeRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(DeleteRangeRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_range(&mut self) -> DeleteRangeRequest {
        if self.has_delete_range() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeRequest::new()
        }
    }

    pub fn get_delete_range<'a>(&'a self) -> &'a DeleteRangeRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(ref v)) => v,
            _ => DeleteRangeRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.ScanRequest scan = 8;

    pub fn clear_scan(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_scan(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_scan(&mut self, v: ScanRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scan<'a>(&'a mut self) -> &'a mut ScanRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(ScanRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_scan(&mut self) -> ScanRequest {
        if self.has_scan() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(v)) => v,
                _ => panic!(),
            }
        } else {
            ScanRequest::new()
        }
    }

    pub fn get_scan<'a>(&'a self) -> &'a ScanRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(ref v)) => v,
            _ => ScanRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.EndTransactionRequest end_transaction = 9;

    pub fn clear_end_transaction(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_transaction(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_transaction(&mut self, v: EndTransactionRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_transaction<'a>(&'a mut self) -> &'a mut EndTransactionRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(EndTransactionRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_transaction(&mut self) -> EndTransactionRequest {
        if self.has_end_transaction() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(v)) => v,
                _ => panic!(),
            }
        } else {
            EndTransactionRequest::new()
        }
    }

    pub fn get_end_transaction<'a>(&'a self) -> &'a EndTransactionRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(ref v)) => v,
            _ => EndTransactionRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.BatchRequest batch = 30;

    pub fn clear_batch(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_batch(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_batch(&mut self, v: BatchRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_batch<'a>(&'a mut self) -> &'a mut BatchRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(BatchRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_batch(&mut self) -> BatchRequest {
        if self.has_batch() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(v)) => v,
                _ => panic!(),
            }
        } else {
            BatchRequest::new()
        }
    }

    pub fn get_batch<'a>(&'a self) -> &'a BatchRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(ref v)) => v,
            _ => BatchRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalRangeLookupRequest internal_range_lookup = 31;

    pub fn clear_internal_range_lookup(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_range_lookup(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_range_lookup(&mut self, v: InternalRangeLookupRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_range_lookup<'a>(&'a mut self) -> &'a mut InternalRangeLookupRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(InternalRangeLookupRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_range_lookup(&mut self) -> InternalRangeLookupRequest {
        if self.has_internal_range_lookup() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalRangeLookupRequest::new()
        }
    }

    pub fn get_internal_range_lookup<'a>(&'a self) -> &'a InternalRangeLookupRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(ref v)) => v,
            _ => InternalRangeLookupRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalHeartbeatTxnRequest internal_heartbeat_txn = 32;

    pub fn clear_internal_heartbeat_txn(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_heartbeat_txn(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_heartbeat_txn(&mut self, v: InternalHeartbeatTxnRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_heartbeat_txn<'a>(&'a mut self) -> &'a mut InternalHeartbeatTxnRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(InternalHeartbeatTxnRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_heartbeat_txn(&mut self) -> InternalHeartbeatTxnRequest {
        if self.has_internal_heartbeat_txn() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalHeartbeatTxnRequest::new()
        }
    }

    pub fn get_internal_heartbeat_txn<'a>(&'a self) -> &'a InternalHeartbeatTxnRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(ref v)) => v,
            _ => InternalHeartbeatTxnRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalPushTxnRequest internal_push_txn = 33;

    pub fn clear_internal_push_txn(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_push_txn(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_push_txn(&mut self, v: InternalPushTxnRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_push_txn<'a>(&'a mut self) -> &'a mut InternalPushTxnRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(InternalPushTxnRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_push_txn(&mut self) -> InternalPushTxnRequest {
        if self.has_internal_push_txn() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalPushTxnRequest::new()
        }
    }

    pub fn get_internal_push_txn<'a>(&'a self) -> &'a InternalPushTxnRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(ref v)) => v,
            _ => InternalPushTxnRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentRequest internal_resolve_intent = 34;

    pub fn clear_internal_resolve_intent(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent(&mut self, v: InternalResolveIntentRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent<'a>(&'a mut self) -> &'a mut InternalResolveIntentRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(InternalResolveIntentRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent(&mut self) -> InternalResolveIntentRequest {
        if self.has_internal_resolve_intent() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentRequest::new()
        }
    }

    pub fn get_internal_resolve_intent<'a>(&'a self) -> &'a InternalResolveIntentRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(ref v)) => v,
            _ => InternalResolveIntentRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalResolveIntentRangeRequest internal_resolve_intent_range = 35;

    pub fn clear_internal_resolve_intent_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_resolve_intent_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_resolve_intent_range(&mut self, v: InternalResolveIntentRangeRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_resolve_intent_range<'a>(&'a mut self) -> &'a mut InternalResolveIntentRangeRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(InternalResolveIntentRangeRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_resolve_intent_range(&mut self) -> InternalResolveIntentRangeRequest {
        if self.has_internal_resolve_intent_range() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalResolveIntentRangeRequest::new()
        }
    }

    pub fn get_internal_resolve_intent_range<'a>(&'a self) -> &'a InternalResolveIntentRangeRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(ref v)) => v,
            _ => InternalResolveIntentRangeRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalMergeRequest internal_merge_response = 36;

    pub fn clear_internal_merge_response(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_merge_response(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_merge_response(&mut self, v: InternalMergeRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_merge_response<'a>(&'a mut self) -> &'a mut InternalMergeRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(InternalMergeRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_merge_response(&mut self) -> InternalMergeRequest {
        if self.has_internal_merge_response() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalMergeRequest::new()
        }
    }

    pub fn get_internal_merge_response<'a>(&'a self) -> &'a InternalMergeRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(ref v)) => v,
            _ => InternalMergeRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalTruncateLogRequest internal_truncate_log = 37;

    pub fn clear_internal_truncate_log(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_truncate_log(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_truncate_log(&mut self, v: InternalTruncateLogRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_truncate_log<'a>(&'a mut self) -> &'a mut InternalTruncateLogRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(InternalTruncateLogRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_truncate_log(&mut self) -> InternalTruncateLogRequest {
        if self.has_internal_truncate_log() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalTruncateLogRequest::new()
        }
    }

    pub fn get_internal_truncate_log<'a>(&'a self) -> &'a InternalTruncateLogRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(ref v)) => v,
            _ => InternalTruncateLogRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalGCRequest internal_gc = 38;

    pub fn clear_internal_gc(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_gc(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_gc(&mut self, v: InternalGCRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_gc<'a>(&'a mut self) -> &'a mut InternalGCRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(InternalGCRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_gc(&mut self) -> InternalGCRequest {
        if self.has_internal_gc() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalGCRequest::new()
        }
    }

    pub fn get_internal_gc<'a>(&'a self) -> &'a InternalGCRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(ref v)) => v,
            _ => InternalGCRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalLeaderLeaseRequest internal_lease = 39;

    pub fn clear_internal_lease(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_lease(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_lease(&mut self, v: InternalLeaderLeaseRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_lease<'a>(&'a mut self) -> &'a mut InternalLeaderLeaseRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(InternalLeaderLeaseRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_lease(&mut self) -> InternalLeaderLeaseRequest {
        if self.has_internal_lease() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalLeaderLeaseRequest::new()
        }
    }

    pub fn get_internal_lease<'a>(&'a self) -> &'a InternalLeaderLeaseRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(ref v)) => v,
            _ => InternalLeaderLeaseRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.InternalBatchRequest internal_batch = 40;

    pub fn clear_internal_batch(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_internal_batch(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_internal_batch(&mut self, v: InternalBatchRequest) {
        self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_batch<'a>(&'a mut self) -> &'a mut InternalBatchRequest {
        if let ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(InternalBatchRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_internal_batch(&mut self) -> InternalBatchRequest {
        if self.has_internal_batch() {
            match self.value.take() {
                ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(v)) => v,
                _ => panic!(),
            }
        } else {
            InternalBatchRequest::new()
        }
    }

    pub fn get_internal_batch<'a>(&'a self) -> &'a InternalBatchRequest {
        match self.value {
            ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(ref v)) => v,
            _ => InternalBatchRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for InternalRaftCommandUnion {
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
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::contains(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::get(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::put(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::conditional_put(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::increment(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::delete_range(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::scan(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::end_transaction(try!(is.read_message())));
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::batch(try!(is.read_message())));
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_range_lookup(try!(is.read_message())));
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(try!(is.read_message())));
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_push_txn(try!(is.read_message())));
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent(try!(is.read_message())));
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(try!(is.read_message())));
                },
                36 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_merge_response(try!(is.read_message())));
                },
                37 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_truncate_log(try!(is.read_message())));
                },
                38 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_gc(try!(is.read_message())));
                },
                39 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_lease(try!(is.read_message())));
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(InternalRaftCommandUnion_oneof_value::internal_batch(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &InternalRaftCommandUnion_oneof_value::contains(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::get(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::conditional_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::increment(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::delete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::scan(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::end_transaction(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::batch(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_range_lookup(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_push_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_resolve_intent(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_merge_response(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_truncate_log(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_gc(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_lease(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &InternalRaftCommandUnion_oneof_value::internal_batch(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &InternalRaftCommandUnion_oneof_value::contains(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::get(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::put(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::conditional_put(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::increment(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::delete(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::delete_range(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::scan(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::end_transaction(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::batch(ref v) => {
                    try!(os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_range_lookup(ref v) => {
                    try!(os.write_tag(31, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_heartbeat_txn(ref v) => {
                    try!(os.write_tag(32, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_push_txn(ref v) => {
                    try!(os.write_tag(33, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_resolve_intent(ref v) => {
                    try!(os.write_tag(34, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_resolve_intent_range(ref v) => {
                    try!(os.write_tag(35, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_merge_response(ref v) => {
                    try!(os.write_tag(36, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_truncate_log(ref v) => {
                    try!(os.write_tag(37, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_gc(ref v) => {
                    try!(os.write_tag(38, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_lease(ref v) => {
                    try!(os.write_tag(39, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &InternalRaftCommandUnion_oneof_value::internal_batch(ref v) => {
                    try!(os.write_tag(40, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<InternalRaftCommandUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalRaftCommandUnion {
    fn new() -> InternalRaftCommandUnion {
        InternalRaftCommandUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalRaftCommandUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contains",
                    InternalRaftCommandUnion::has_contains,
                    InternalRaftCommandUnion::get_contains,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get",
                    InternalRaftCommandUnion::has_get,
                    InternalRaftCommandUnion::get_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put",
                    InternalRaftCommandUnion::has_put,
                    InternalRaftCommandUnion::get_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "conditional_put",
                    InternalRaftCommandUnion::has_conditional_put,
                    InternalRaftCommandUnion::get_conditional_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "increment",
                    InternalRaftCommandUnion::has_increment,
                    InternalRaftCommandUnion::get_increment,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete",
                    InternalRaftCommandUnion::has_delete,
                    InternalRaftCommandUnion::get_delete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_range",
                    InternalRaftCommandUnion::has_delete_range,
                    InternalRaftCommandUnion::get_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scan",
                    InternalRaftCommandUnion::has_scan,
                    InternalRaftCommandUnion::get_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end_transaction",
                    InternalRaftCommandUnion::has_end_transaction,
                    InternalRaftCommandUnion::get_end_transaction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "batch",
                    InternalRaftCommandUnion::has_batch,
                    InternalRaftCommandUnion::get_batch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_range_lookup",
                    InternalRaftCommandUnion::has_internal_range_lookup,
                    InternalRaftCommandUnion::get_internal_range_lookup,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_heartbeat_txn",
                    InternalRaftCommandUnion::has_internal_heartbeat_txn,
                    InternalRaftCommandUnion::get_internal_heartbeat_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_push_txn",
                    InternalRaftCommandUnion::has_internal_push_txn,
                    InternalRaftCommandUnion::get_internal_push_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent",
                    InternalRaftCommandUnion::has_internal_resolve_intent,
                    InternalRaftCommandUnion::get_internal_resolve_intent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_resolve_intent_range",
                    InternalRaftCommandUnion::has_internal_resolve_intent_range,
                    InternalRaftCommandUnion::get_internal_resolve_intent_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_merge_response",
                    InternalRaftCommandUnion::has_internal_merge_response,
                    InternalRaftCommandUnion::get_internal_merge_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_truncate_log",
                    InternalRaftCommandUnion::has_internal_truncate_log,
                    InternalRaftCommandUnion::get_internal_truncate_log,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_gc",
                    InternalRaftCommandUnion::has_internal_gc,
                    InternalRaftCommandUnion::get_internal_gc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_lease",
                    InternalRaftCommandUnion::has_internal_lease,
                    InternalRaftCommandUnion::get_internal_lease,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_batch",
                    InternalRaftCommandUnion::has_internal_batch,
                    InternalRaftCommandUnion::get_internal_batch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalRaftCommandUnion>(
                    "InternalRaftCommandUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalRaftCommandUnion {
    fn clear(&mut self) {
        self.clear_contains();
        self.clear_get();
        self.clear_put();
        self.clear_conditional_put();
        self.clear_increment();
        self.clear_delete();
        self.clear_delete_range();
        self.clear_scan();
        self.clear_end_transaction();
        self.clear_batch();
        self.clear_internal_range_lookup();
        self.clear_internal_heartbeat_txn();
        self.clear_internal_push_txn();
        self.clear_internal_resolve_intent();
        self.clear_internal_resolve_intent_range();
        self.clear_internal_merge_response();
        self.clear_internal_truncate_log();
        self.clear_internal_gc();
        self.clear_internal_lease();
        self.clear_internal_batch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalRaftCommandUnion {
    fn eq(&self, other: &InternalRaftCommandUnion) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalRaftCommandUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalRaftCommand {
    // message fields
    raft_id: ::std::option::Option<i64>,
    origin_node_id: ::std::option::Option<u64>,
    cmd: ::protobuf::SingularPtrField<InternalRaftCommandUnion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalRaftCommand {
    pub fn new() -> InternalRaftCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalRaftCommand {
        static mut instance: ::protobuf::lazy::Lazy<InternalRaftCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalRaftCommand,
        };
        unsafe {
            instance.get(|| {
                InternalRaftCommand {
                    raft_id: ::std::option::Option::None,
                    origin_node_id: ::std::option::Option::None,
                    cmd: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 raft_id = 1;

    pub fn clear_raft_id(&mut self) {
        self.raft_id = ::std::option::Option::None;
    }

    pub fn has_raft_id(&self) -> bool {
        self.raft_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raft_id(&mut self, v: i64) {
        self.raft_id = ::std::option::Option::Some(v);
    }

    pub fn get_raft_id<'a>(&self) -> i64 {
        self.raft_id.unwrap_or(0)
    }

    // optional uint64 origin_node_id = 2;

    pub fn clear_origin_node_id(&mut self) {
        self.origin_node_id = ::std::option::Option::None;
    }

    pub fn has_origin_node_id(&self) -> bool {
        self.origin_node_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_node_id(&mut self, v: u64) {
        self.origin_node_id = ::std::option::Option::Some(v);
    }

    pub fn get_origin_node_id<'a>(&self) -> u64 {
        self.origin_node_id.unwrap_or(0)
    }

    // optional .cockroach.proto.InternalRaftCommandUnion cmd = 3;

    pub fn clear_cmd(&mut self) {
        self.cmd.clear();
    }

    pub fn has_cmd(&self) -> bool {
        self.cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: InternalRaftCommandUnion) {
        self.cmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd<'a>(&'a mut self) -> &'a mut InternalRaftCommandUnion {
        if self.cmd.is_none() {
            self.cmd.set_default();
        };
        self.cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd(&mut self) -> InternalRaftCommandUnion {
        self.cmd.take().unwrap_or_else(|| InternalRaftCommandUnion::new())
    }

    pub fn get_cmd<'a>(&'a self) -> &'a InternalRaftCommandUnion {
        self.cmd.as_ref().unwrap_or_else(|| InternalRaftCommandUnion::default_instance())
    }
}

impl ::protobuf::Message for InternalRaftCommand {
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
                    let tmp = try!(is.read_int64());
                    self.raft_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.origin_node_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cmd.set_default();
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
        for value in self.raft_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.origin_node_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cmd.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.raft_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.origin_node_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.cmd.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalRaftCommand>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalRaftCommand {
    fn new() -> InternalRaftCommand {
        InternalRaftCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalRaftCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "raft_id",
                    InternalRaftCommand::has_raft_id,
                    InternalRaftCommand::get_raft_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "origin_node_id",
                    InternalRaftCommand::has_origin_node_id,
                    InternalRaftCommand::get_origin_node_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd",
                    InternalRaftCommand::has_cmd,
                    InternalRaftCommand::get_cmd,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalRaftCommand>(
                    "InternalRaftCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalRaftCommand {
    fn clear(&mut self) {
        self.clear_raft_id();
        self.clear_origin_node_id();
        self.clear_cmd();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalRaftCommand {
    fn eq(&self, other: &InternalRaftCommand) -> bool {
        self.raft_id == other.raft_id &&
        self.origin_node_id == other.origin_node_id &&
        self.cmd == other.cmd &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalRaftCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RaftMessageRequest {
    // message fields
    group_id: ::std::option::Option<u64>,
    msg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RaftMessageRequest {
    pub fn new() -> RaftMessageRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftMessageRequest {
        static mut instance: ::protobuf::lazy::Lazy<RaftMessageRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftMessageRequest,
        };
        unsafe {
            instance.get(|| {
                RaftMessageRequest {
                    group_id: ::std::option::Option::None,
                    msg: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 group_id = 1;

    pub fn clear_group_id(&mut self) {
        self.group_id = ::std::option::Option::None;
    }

    pub fn has_group_id(&self) -> bool {
        self.group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: u64) {
        self.group_id = ::std::option::Option::Some(v);
    }

    pub fn get_group_id<'a>(&self) -> u64 {
        self.group_id.unwrap_or(0)
    }

    // optional bytes msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::vec::Vec<u8> {
        self.msg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_msg<'a>(&'a self) -> &'a [u8] {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RaftMessageRequest {
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
                    let tmp = try!(is.read_uint64());
                    self.group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.msg.set_default();
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
        for value in self.group_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.msg.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.group_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.msg.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RaftMessageRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RaftMessageRequest {
    fn new() -> RaftMessageRequest {
        RaftMessageRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftMessageRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "group_id",
                    RaftMessageRequest::has_group_id,
                    RaftMessageRequest::get_group_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "msg",
                    RaftMessageRequest::has_msg,
                    RaftMessageRequest::get_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftMessageRequest>(
                    "RaftMessageRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftMessageRequest {
    fn clear(&mut self) {
        self.clear_group_id();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RaftMessageRequest {
    fn eq(&self, other: &RaftMessageRequest) -> bool {
        self.group_id == other.group_id &&
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RaftMessageRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RaftMessageResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RaftMessageResponse {
    pub fn new() -> RaftMessageResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftMessageResponse {
        static mut instance: ::protobuf::lazy::Lazy<RaftMessageResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftMessageResponse,
        };
        unsafe {
            instance.get(|| {
                RaftMessageResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for RaftMessageResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<RaftMessageResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RaftMessageResponse {
    fn new() -> RaftMessageResponse {
        RaftMessageResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftMessageResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RaftMessageResponse>(
                    "RaftMessageResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftMessageResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RaftMessageResponse {
    fn eq(&self, other: &RaftMessageResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RaftMessageResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalTimeSeriesData {
    // message fields
    start_timestamp_nanos: ::std::option::Option<i64>,
    sample_duration_nanos: ::std::option::Option<i64>,
    samples: ::protobuf::RepeatedField<InternalTimeSeriesSample>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalTimeSeriesData {
    pub fn new() -> InternalTimeSeriesData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalTimeSeriesData {
        static mut instance: ::protobuf::lazy::Lazy<InternalTimeSeriesData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalTimeSeriesData,
        };
        unsafe {
            instance.get(|| {
                InternalTimeSeriesData {
                    start_timestamp_nanos: ::std::option::Option::None,
                    sample_duration_nanos: ::std::option::Option::None,
                    samples: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 start_timestamp_nanos = 1;

    pub fn clear_start_timestamp_nanos(&mut self) {
        self.start_timestamp_nanos = ::std::option::Option::None;
    }

    pub fn has_start_timestamp_nanos(&self) -> bool {
        self.start_timestamp_nanos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_timestamp_nanos(&mut self, v: i64) {
        self.start_timestamp_nanos = ::std::option::Option::Some(v);
    }

    pub fn get_start_timestamp_nanos<'a>(&self) -> i64 {
        self.start_timestamp_nanos.unwrap_or(0)
    }

    // optional int64 sample_duration_nanos = 2;

    pub fn clear_sample_duration_nanos(&mut self) {
        self.sample_duration_nanos = ::std::option::Option::None;
    }

    pub fn has_sample_duration_nanos(&self) -> bool {
        self.sample_duration_nanos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_duration_nanos(&mut self, v: i64) {
        self.sample_duration_nanos = ::std::option::Option::Some(v);
    }

    pub fn get_sample_duration_nanos<'a>(&self) -> i64 {
        self.sample_duration_nanos.unwrap_or(0)
    }

    // repeated .cockroach.proto.InternalTimeSeriesSample samples = 3;

    pub fn clear_samples(&mut self) {
        self.samples.clear();
    }

    // Param is passed by value, moved
    pub fn set_samples(&mut self, v: ::protobuf::RepeatedField<InternalTimeSeriesSample>) {
        self.samples = v;
    }

    // Mutable pointer to the field.
    pub fn mut_samples<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<InternalTimeSeriesSample> {
        &mut self.samples
    }

    // Take field
    pub fn take_samples(&mut self) -> ::protobuf::RepeatedField<InternalTimeSeriesSample> {
        ::std::mem::replace(&mut self.samples, ::protobuf::RepeatedField::new())
    }

    pub fn get_samples<'a>(&'a self) -> &'a [InternalTimeSeriesSample] {
        &self.samples
    }
}

impl ::protobuf::Message for InternalTimeSeriesData {
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
                    let tmp = try!(is.read_int64());
                    self.start_timestamp_nanos = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.sample_duration_nanos = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.samples));
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
        for value in self.start_timestamp_nanos.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sample_duration_nanos.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.samples.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_timestamp_nanos {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.sample_duration_nanos {
            try!(os.write_int64(2, v));
        };
        for v in self.samples.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<InternalTimeSeriesData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalTimeSeriesData {
    fn new() -> InternalTimeSeriesData {
        InternalTimeSeriesData::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalTimeSeriesData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "start_timestamp_nanos",
                    InternalTimeSeriesData::has_start_timestamp_nanos,
                    InternalTimeSeriesData::get_start_timestamp_nanos,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sample_duration_nanos",
                    InternalTimeSeriesData::has_sample_duration_nanos,
                    InternalTimeSeriesData::get_sample_duration_nanos,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "samples",
                    InternalTimeSeriesData::get_samples,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalTimeSeriesData>(
                    "InternalTimeSeriesData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalTimeSeriesData {
    fn clear(&mut self) {
        self.clear_start_timestamp_nanos();
        self.clear_sample_duration_nanos();
        self.clear_samples();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalTimeSeriesData {
    fn eq(&self, other: &InternalTimeSeriesData) -> bool {
        self.start_timestamp_nanos == other.start_timestamp_nanos &&
        self.sample_duration_nanos == other.sample_duration_nanos &&
        self.samples == other.samples &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalTimeSeriesData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct InternalTimeSeriesSample {
    // message fields
    offset: ::std::option::Option<i32>,
    count: ::std::option::Option<u32>,
    sum: ::std::option::Option<f64>,
    max: ::std::option::Option<f64>,
    min: ::std::option::Option<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl InternalTimeSeriesSample {
    pub fn new() -> InternalTimeSeriesSample {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InternalTimeSeriesSample {
        static mut instance: ::protobuf::lazy::Lazy<InternalTimeSeriesSample> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InternalTimeSeriesSample,
        };
        unsafe {
            instance.get(|| {
                InternalTimeSeriesSample {
                    offset: ::std::option::Option::None,
                    count: ::std::option::Option::None,
                    sum: ::std::option::Option::None,
                    max: ::std::option::Option::None,
                    min: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 offset = 1;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: i32) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset<'a>(&self) -> i32 {
        self.offset.unwrap_or(0)
    }

    // optional uint32 count = 6;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count<'a>(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    // optional double sum = 7;

    pub fn clear_sum(&mut self) {
        self.sum = ::std::option::Option::None;
    }

    pub fn has_sum(&self) -> bool {
        self.sum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sum(&mut self, v: f64) {
        self.sum = ::std::option::Option::Some(v);
    }

    pub fn get_sum<'a>(&self) -> f64 {
        self.sum.unwrap_or(0.)
    }

    // optional double max = 8;

    pub fn clear_max(&mut self) {
        self.max = ::std::option::Option::None;
    }

    pub fn has_max(&self) -> bool {
        self.max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max(&mut self, v: f64) {
        self.max = ::std::option::Option::Some(v);
    }

    pub fn get_max<'a>(&self) -> f64 {
        self.max.unwrap_or(0.)
    }

    // optional double min = 9;

    pub fn clear_min(&mut self) {
        self.min = ::std::option::Option::None;
    }

    pub fn has_min(&self) -> bool {
        self.min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min(&mut self, v: f64) {
        self.min = ::std::option::Option::Some(v);
    }

    pub fn get_min<'a>(&self) -> f64 {
        self.min.unwrap_or(0.)
    }
}

impl ::protobuf::Message for InternalTimeSeriesSample {
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
                    self.offset = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.count = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.sum = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.max = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.min = ::std::option::Option::Some(tmp);
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
        for value in self.offset.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.count.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sum.is_some() {
            my_size += 9;
        };
        if self.max.is_some() {
            my_size += 9;
        };
        if self.min.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offset {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.count {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.sum {
            try!(os.write_double(7, v));
        };
        if let Some(v) = self.max {
            try!(os.write_double(8, v));
        };
        if let Some(v) = self.min {
            try!(os.write_double(9, v));
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
        ::std::any::TypeId::of::<InternalTimeSeriesSample>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InternalTimeSeriesSample {
    fn new() -> InternalTimeSeriesSample {
        InternalTimeSeriesSample::new()
    }

    fn descriptor_static(_: ::std::option::Option<InternalTimeSeriesSample>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "offset",
                    InternalTimeSeriesSample::has_offset,
                    InternalTimeSeriesSample::get_offset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "count",
                    InternalTimeSeriesSample::has_count,
                    InternalTimeSeriesSample::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "sum",
                    InternalTimeSeriesSample::has_sum,
                    InternalTimeSeriesSample::get_sum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "max",
                    InternalTimeSeriesSample::has_max,
                    InternalTimeSeriesSample::get_max,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "min",
                    InternalTimeSeriesSample::has_min,
                    InternalTimeSeriesSample::get_min,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InternalTimeSeriesSample>(
                    "InternalTimeSeriesSample",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InternalTimeSeriesSample {
    fn clear(&mut self) {
        self.clear_offset();
        self.clear_count();
        self.clear_sum();
        self.clear_max();
        self.clear_min();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for InternalTimeSeriesSample {
    fn eq(&self, other: &InternalTimeSeriesSample) -> bool {
        self.offset == other.offset &&
        self.count == other.count &&
        self.sum == other.sum &&
        self.max == other.max &&
        self.min == other.min &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for InternalTimeSeriesSample {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RaftTruncatedState {
    // message fields
    index: ::std::option::Option<u64>,
    term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RaftTruncatedState {
    pub fn new() -> RaftTruncatedState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftTruncatedState {
        static mut instance: ::protobuf::lazy::Lazy<RaftTruncatedState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftTruncatedState,
        };
        unsafe {
            instance.get(|| {
                RaftTruncatedState {
                    index: ::std::option::Option::None,
                    term: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index<'a>(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    // optional uint64 term = 2;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term<'a>(&self) -> u64 {
        self.term.unwrap_or(0)
    }
}

impl ::protobuf::Message for RaftTruncatedState {
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
                    let tmp = try!(is.read_uint64());
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.term = ::std::option::Option::Some(tmp);
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
        for value in self.index.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.term.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.term {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<RaftTruncatedState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RaftTruncatedState {
    fn new() -> RaftTruncatedState {
        RaftTruncatedState::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftTruncatedState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "index",
                    RaftTruncatedState::has_index,
                    RaftTruncatedState::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "term",
                    RaftTruncatedState::has_term,
                    RaftTruncatedState::get_term,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftTruncatedState>(
                    "RaftTruncatedState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftTruncatedState {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RaftTruncatedState {
    fn eq(&self, other: &RaftTruncatedState) -> bool {
        self.index == other.index &&
        self.term == other.term &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RaftTruncatedState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RaftSnapshotData {
    // message fields
    KV: ::protobuf::RepeatedField<RaftSnapshotData_KeyValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RaftSnapshotData {
    pub fn new() -> RaftSnapshotData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftSnapshotData {
        static mut instance: ::protobuf::lazy::Lazy<RaftSnapshotData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftSnapshotData,
        };
        unsafe {
            instance.get(|| {
                RaftSnapshotData {
                    KV: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .cockroach.proto.RaftSnapshotData.KeyValue KV = 1;

    pub fn clear_KV(&mut self) {
        self.KV.clear();
    }

    // Param is passed by value, moved
    pub fn set_KV(&mut self, v: ::protobuf::RepeatedField<RaftSnapshotData_KeyValue>) {
        self.KV = v;
    }

    // Mutable pointer to the field.
    pub fn mut_KV<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<RaftSnapshotData_KeyValue> {
        &mut self.KV
    }

    // Take field
    pub fn take_KV(&mut self) -> ::protobuf::RepeatedField<RaftSnapshotData_KeyValue> {
        ::std::mem::replace(&mut self.KV, ::protobuf::RepeatedField::new())
    }

    pub fn get_KV<'a>(&'a self) -> &'a [RaftSnapshotData_KeyValue] {
        &self.KV
    }
}

impl ::protobuf::Message for RaftSnapshotData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.KV));
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
        for value in self.KV.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.KV.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<RaftSnapshotData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RaftSnapshotData {
    fn new() -> RaftSnapshotData {
        RaftSnapshotData::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftSnapshotData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "KV",
                    RaftSnapshotData::get_KV,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftSnapshotData>(
                    "RaftSnapshotData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftSnapshotData {
    fn clear(&mut self) {
        self.clear_KV();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RaftSnapshotData {
    fn eq(&self, other: &RaftSnapshotData) -> bool {
        self.KV == other.KV &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RaftSnapshotData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RaftSnapshotData_KeyValue {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RaftSnapshotData_KeyValue {
    pub fn new() -> RaftSnapshotData_KeyValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftSnapshotData_KeyValue {
        static mut instance: ::protobuf::lazy::Lazy<RaftSnapshotData_KeyValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftSnapshotData_KeyValue,
        };
        unsafe {
            instance.get(|| {
                RaftSnapshotData_KeyValue {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RaftSnapshotData_KeyValue {
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
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<RaftSnapshotData_KeyValue>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RaftSnapshotData_KeyValue {
    fn new() -> RaftSnapshotData_KeyValue {
        RaftSnapshotData_KeyValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftSnapshotData_KeyValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RaftSnapshotData_KeyValue::has_key,
                    RaftSnapshotData_KeyValue::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    RaftSnapshotData_KeyValue::has_value,
                    RaftSnapshotData_KeyValue::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftSnapshotData_KeyValue>(
                    "RaftSnapshotData_KeyValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftSnapshotData_KeyValue {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RaftSnapshotData_KeyValue {
    fn eq(&self, other: &RaftSnapshotData_KeyValue) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RaftSnapshotData_KeyValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum PushTxnType {
    PUSH_TIMESTAMP = 0,
    ABORT_TXN = 1,
    CLEANUP_TXN = 2,
}

impl ::protobuf::ProtobufEnum for PushTxnType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PushTxnType> {
        match value {
            0 => ::std::option::Option::Some(PushTxnType::PUSH_TIMESTAMP),
            1 => ::std::option::Option::Some(PushTxnType::ABORT_TXN),
            2 => ::std::option::Option::Some(PushTxnType::CLEANUP_TXN),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<PushTxnType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PushTxnType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PushTxnType {
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum InternalValueType {
    _CR_TS = 1,
}

impl ::protobuf::ProtobufEnum for InternalValueType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<InternalValueType> {
        match value {
            1 => ::std::option::Option::Some(InternalValueType::_CR_TS),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<InternalValueType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("InternalValueType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for InternalValueType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0f, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x19, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2f, 0x61, 0x70, 0x69, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1a, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x64, 0x61, 0x74, 0x61,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x78, 0x0a, 0x1a, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x0a, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x16, 0x0a, 0x0e, 0x69, 0x67, 0x6e, 0x6f,
    0x72, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08,
    0x22, 0x80, 0x01, 0x0a, 0x1b, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x12, 0x30, 0x0a, 0x06, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x6f, 0x72, 0x22, 0x4d, 0x0a, 0x1b, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48,
    0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x22, 0x4f, 0x0a, 0x1c, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48, 0x65,
    0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x22, 0xee, 0x01, 0x0a, 0x11, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x07, 0x67, 0x63, 0x5f,
    0x6d, 0x65, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x43, 0x4d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x36, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x47, 0x43, 0x4b, 0x65, 0x79, 0x1a,
    0x43, 0x0a, 0x05, 0x47, 0x43, 0x4b, 0x65, 0x79, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x2d, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x22, 0x45, 0x0a, 0x12, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x47, 0x43, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0xc1, 0x01, 0x0a, 0x16,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x30, 0x0a, 0x0a, 0x70, 0x75, 0x73, 0x68, 0x65, 0x65,
    0x5f, 0x74, 0x78, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2f, 0x0a, 0x09, 0x70, 0x75, 0x73, 0x68,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x50, 0x75,
    0x73, 0x68, 0x54, 0x78, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x14, 0x0a, 0x0c, 0x72, 0x61, 0x6e,
    0x67, 0x65, 0x5f, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x22,
    0x7c, 0x0a, 0x17, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54,
    0x78, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x30, 0x0a, 0x0a, 0x70,
    0x75, 0x73, 0x68, 0x65, 0x65, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x4e, 0x0a,
    0x1c, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a,
    0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x50, 0x0a,
    0x1d, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f,
    0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22,
    0x53, 0x0a, 0x21, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x22, 0x55, 0x0a, 0x22, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x6d, 0x0a, 0x14, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x48, 0x0a, 0x15, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x22, 0x5b, 0x0a, 0x1a, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x4c, 0x6f, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x22, 0x4e, 0x0a, 0x1b, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x72, 0x75,
    0x6e, 0x63, 0x61, 0x74, 0x65, 0x4c, 0x6f, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x22, 0x73, 0x0a, 0x1a, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12,
    0x25, 0x0a, 0x05, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x22, 0x4e, 0x0a, 0x1b, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0xfd, 0x05, 0x0a, 0x14, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12,
    0x34, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x2a, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48,
    0x00, 0x12, 0x2a, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x41, 0x0a,
    0x0f, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x70, 0x75, 0x74,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00,
    0x12, 0x36, 0x0a, 0x09, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x30, 0x0a, 0x06, 0x64, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x3b, 0x0a, 0x0c, 0x64, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x23, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x2c, 0x0a, 0x04, 0x73, 0x63, 0x61, 0x6e, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x41, 0x0a, 0x0f, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x44, 0x0a, 0x11, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x70, 0x75, 0x73, 0x68, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x1e, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75,
    0x73, 0x68, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x50,
    0x0a, 0x17, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x1f, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x2d, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00,
    0x12, 0x5b, 0x0a, 0x1d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x18, 0x20, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52,
    0x61, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x42, 0x07, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x8a, 0x06, 0x0a, 0x15, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e,
    0x12, 0x35, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x21, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x2b, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x48, 0x00, 0x12, 0x2b, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48,
    0x00, 0x12, 0x42, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x5f, 0x70, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x37, 0x0a, 0x09, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x63, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x31,
    0x0a, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48,
    0x00, 0x12, 0x3c, 0x0a, 0x0c, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12,
    0x2d, 0x0a, 0x04, 0x73, 0x63, 0x61, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x42,
    0x0a, 0x0f, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x48, 0x00, 0x12, 0x45, 0x0a, 0x11, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x70,
    0x75, 0x73, 0x68, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x51, 0x0a, 0x17, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x5f, 0x69, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x18, 0x1f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x5c, 0x0a, 0x1d,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x5f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x20, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x22, 0x7f, 0x0a, 0x14, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x42,
    0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x37, 0x0a, 0x08, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55,
    0x6e, 0x69, 0x6f, 0x6e, 0x22, 0x83, 0x01, 0x0a, 0x15, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f,
    0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12,
    0x39, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x26, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x22, 0xe3, 0x07, 0x0a, 0x14, 0x52,
    0x65, 0x61, 0x64, 0x57, 0x72, 0x69, 0x74, 0x65, 0x43, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x2b, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00,
    0x12, 0x42, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f,
    0x70, 0x75, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x48, 0x00, 0x12, 0x37, 0x0a, 0x09, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x31, 0x0a,
    0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00,
    0x12, 0x3c, 0x0a, 0x0c, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52,
    0x61, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x42,
    0x0a, 0x0f, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x48, 0x00, 0x12, 0x4f, 0x0a, 0x16, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x68,
    0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48, 0x65, 0x61,
    0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x48, 0x00, 0x12, 0x45, 0x0a, 0x11, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f,
    0x70, 0x75, 0x73, 0x68, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x51, 0x0a, 0x17, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x5f, 0x69,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x5c, 0x0a,
    0x1d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x0d,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x40, 0x0a, 0x0e, 0x69,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4d, 0x65,
    0x72, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x4d, 0x0a,
    0x15, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61,
    0x74, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x4c,
    0x6f, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x3a, 0x0a, 0x0b,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x67, 0x63, 0x18, 0x10, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x23, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x47, 0x43, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x4d, 0x0a, 0x15, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x61, 0x73,
    0x65, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0xaa, 0x0a, 0x0a, 0x18, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x61, 0x66,
    0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x34, 0x0a,
    0x08, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x48, 0x00, 0x12, 0x2a, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12,
    0x2a, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x50,
    0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x41, 0x0a, 0x0f, 0x63,
    0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x70, 0x75, 0x74, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x36,
    0x0a, 0x09, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x21, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x30, 0x0a, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x3b, 0x0a, 0x0c, 0x64, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x2c, 0x0a, 0x04, 0x73, 0x63, 0x61, 0x6e, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x48, 0x00, 0x12, 0x41, 0x0a, 0x0f, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x45,
    0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x2e, 0x0a, 0x05, 0x62, 0x61, 0x74, 0x63, 0x68, 0x18,
    0x1e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x4c, 0x0a, 0x15, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x18,
    0x1f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x52, 0x61, 0x6e, 0x67, 0x65, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x00, 0x12, 0x4e, 0x0a, 0x16, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x5f, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x20,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48,
    0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x00, 0x12, 0x44, 0x0a, 0x11, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x5f, 0x70, 0x75, 0x73, 0x68, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x21, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78,
    0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x50, 0x0a, 0x17, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x5f, 0x69,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x22, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x5b, 0x0a, 0x1d,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x5f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x23, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x48, 0x0a, 0x17, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x18, 0x24, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x48, 0x00, 0x12, 0x4c, 0x0a, 0x15, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f,
    0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x18, 0x25, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x72, 0x75,
    0x6e, 0x63, 0x61, 0x74, 0x65, 0x4c, 0x6f, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48,
    0x00, 0x12, 0x39, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x67, 0x63,
    0x18, 0x26, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x45, 0x0a, 0x0e,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x18, 0x27,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x48, 0x00, 0x12, 0x3f, 0x0a, 0x0e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f,
    0x62, 0x61, 0x74, 0x63, 0x68, 0x18, 0x28, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x00, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x76, 0x0a,
    0x13, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x61, 0x66, 0x74, 0x43, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x16, 0x0a, 0x0e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f,
    0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x12, 0x36, 0x0a,
    0x03, 0x63, 0x6d, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x61, 0x66, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x22, 0x33, 0x0a, 0x12, 0x52, 0x61, 0x66, 0x74, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x67,
    0x72, 0x6f, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12, 0x0b, 0x0a,
    0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x15, 0x0a, 0x13, 0x52, 0x61,
    0x66, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x92, 0x01, 0x0a, 0x16, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x69,
    0x6d, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x44, 0x61, 0x74, 0x61, 0x12, 0x1d, 0x0a, 0x15,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f,
    0x6e, 0x61, 0x6e, 0x6f, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x1d, 0x0a, 0x15, 0x73,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e,
    0x61, 0x6e, 0x6f, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x12, 0x3a, 0x0a, 0x07, 0x73, 0x61,
    0x6d, 0x70, 0x6c, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73,
    0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x22, 0x60, 0x0a, 0x18, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x53, 0x61, 0x6d, 0x70,
    0x6c, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x75, 0x6d, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0b,
    0x0a, 0x03, 0x6d, 0x61, 0x78, 0x18, 0x08, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0b, 0x0a, 0x03, 0x6d,
    0x69, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x22, 0x31, 0x0a, 0x12, 0x52, 0x61, 0x66, 0x74,
    0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0d,
    0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12, 0x0c, 0x0a,
    0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x22, 0x72, 0x0a, 0x10, 0x52,
    0x61, 0x66, 0x74, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x36, 0x0a, 0x02, 0x4b, 0x56, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x61,
    0x66, 0x74, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4b,
    0x65, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x1a, 0x26, 0x0a, 0x08, 0x4b, 0x65, 0x79, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x2a,
    0x41, 0x0a, 0x0b, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12,
    0x0a, 0x0e, 0x50, 0x55, 0x53, 0x48, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x53, 0x54, 0x41, 0x4d, 0x50,
    0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x5f, 0x54, 0x58, 0x4e, 0x10,
    0x01, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x4c, 0x45, 0x41, 0x4e, 0x55, 0x50, 0x5f, 0x54, 0x58, 0x4e,
    0x10, 0x02, 0x2a, 0x1f, 0x0a, 0x11, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x5f, 0x43, 0x52, 0x5f, 0x54,
    0x53, 0x10, 0x01, 0x42, 0x07, 0x5a, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x4a, 0x86, 0x98, 0x01,
    0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x9b, 0x03, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x02, 0x07, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x03, 0x07, 0x25, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x04, 0x07, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x05, 0x08, 0x17, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x07, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x07, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x07, 0x12, 0x03, 0x07, 0x14, 0x1b, 0x0a, 0x56, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0b, 0x00,
    0x12, 0x01, 0x1a, 0x4a, 0x20, 0x54, 0x78, 0x6e, 0x50, 0x75, 0x73, 0x68, 0x54, 0x79, 0x70, 0x65,
    0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x77, 0x68, 0x61, 0x74,
    0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x20,
    0x77, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x75, 0x73, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x0a, 0x20,
    0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x05, 0x10, 0x0a, 0x59, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x15, 0x1a, 0x4c, 0x20, 0x50, 0x75, 0x73, 0x68, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x66, 0x6f, 0x72,
    0x77, 0x61, 0x72, 0x64, 0x20, 0x69, 0x66, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x64, 0x61, 0x74, 0x65, 0x20,
    0x61, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0d, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x13,
    0x14, 0x0a, 0x54, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x10, 0x1a, 0x47,
    0x20, 0x41, 0x62, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x66, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62,
    0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x64, 0x61, 0x74,
    0x65, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x77,
    0x72, 0x69, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0f, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x0f, 0x0e, 0x0f, 0x0a, 0x53, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x11, 0x02, 0x12,
    0x1a, 0x46, 0x20, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6c,
    0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x2f,
    0x61, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x66, 0x20, 0x74,
    0x6f, 0x6f, 0x20, 0x6f, 0x6c, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x11, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x11, 0x10, 0x11, 0x0a, 0xd3, 0x02, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x19, 0x00, 0x1c,
    0x01, 0x1a, 0xc6, 0x02, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x54, 0x79, 0x70, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x63,
    0x6f, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x73, 0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x22, 0x74, 0x61, 0x67, 0x22, 0x20, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x6c, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65, 0x0a, 0x20, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x65, 0x6e,
    0x75, 0x6d, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x6f, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x62, 0x6c, 0x79, 0x20, 0x62, 0x65,
    0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x6f, 0x75, 0x72, 0x20, 0x47, 0x6f, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x43, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x67,
    0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x20, 0x52, 0x6f, 0x63, 0x6b, 0x73, 0x44, 0x42, 0x20, 0x4d, 0x65, 0x72, 0x67,
    0x65, 0x20, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x65,
    0x72, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65,
    0x64, 0x20, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01,
    0x01, 0x12, 0x03, 0x19, 0x05, 0x16, 0x0a, 0x50, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x1b, 0x02, 0x0d, 0x1a, 0x43, 0x20, 0x5f, 0x43, 0x52, 0x5f, 0x54, 0x53, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x69,
    0x65, 0x73, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x1b, 0x0b, 0x0c, 0x0a, 0xf2, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x24, 0x00, 0x2d,
    0x01, 0x1a, 0xe5, 0x02, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x52, 0x61, 0x6e, 0x67, 0x65, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x28, 0x29, 0x20, 0x6d,
    0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20,
    0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x2c,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x20, 0x74, 0x6f, 0x74,
    0x61, 0x6c, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x6f, 0x72, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x2c, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x20, 0x61, 0x64, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x75, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x20,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x2e, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20,
    0x6d, 0x61, 0x78, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x3e, 0x20, 0x31, 0x0a, 0x20,
    0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x65, 0x2d, 0x66, 0x69, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f,
    0x72, 0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x24, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x25,
    0x02, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x25, 0x0b, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x26, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x26, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x11,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26, 0x1e, 0x1f, 0x0a,
    0xd4, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2c, 0x02, 0x23, 0x1a, 0xc6, 0x02,
    0x20, 0x49, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x20,
    0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73,
    0x20, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x77,
    0x68, 0x69, 0x6c, 0x65, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x70, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x64, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20,
    0x20, 0x62, 0x65, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x67, 0x65, 0x6e,
    0x65, 0x72, 0x61, 0x6c, 0x2c, 0x20, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x20,
    0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x75, 0x73, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x20, 0x72, 0x65,
    0x63, 0x6f, 0x72, 0x64, 0x73, 0x2e, 0x20, 0x41, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x69, 0x6e,
    0x67, 0x0a, 0x20, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x20, 0x69,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63,
    0x61, 0x73, 0x65, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x69, 0x6e, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x65, 0x20, 0x72, 0x65, 0x63, 0x75, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2c,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x10, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x21, 0x22, 0x0a, 0xb7,
    0x02, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x34, 0x00, 0x37, 0x01, 0x1a, 0xaa, 0x02, 0x20, 0x41,
    0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4c,
    0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70,
    0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x0a, 0x20, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x6f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66,
    0x6f, 0x72, 0x0a, 0x20, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x75, 0x74, 0x69, 0x76, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x73, 0x20, 0x62, 0x65, 0x79, 0x6f, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x70, 0x72, 0x65, 0x2d, 0x66, 0x69, 0x6c, 0x6c, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72,
    0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x34, 0x08, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x35, 0x02,
    0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x35, 0x0b, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x2b, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x36, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x36,
    0x0b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x2c, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36, 0x35, 0x36, 0x0a, 0xad,
    0x02, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x3e, 0x00, 0x40, 0x01, 0x1a, 0xa0, 0x02, 0x20, 0x41,
    0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62,
    0x65, 0x61, 0x74, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48, 0x65, 0x61, 0x72,
    0x74, 0x62, 0x65, 0x61, 0x74, 0x54, 0x78, 0x6e, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f,
    0x64, 0x2e, 0x20, 0x49, 0x74, 0x27, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20, 0x63, 0x6f,
    0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x65,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x20, 0x6b, 0x6e, 0x6f,
    0x77, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x69, 0x6c, 0x6c, 0x0a,
    0x20, 0x20, 0x6f, 0x6e, 0x67, 0x6f, 0x69, 0x6e, 0x67, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62,
    0x65, 0x61, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64,
    0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x6f, 0x73,
    0x73, 0x69, 0x70, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x3f, 0x0b, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x2a,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x33, 0x34, 0x0a,
    0xa6, 0x02, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x47, 0x00, 0x49, 0x01, 0x1a, 0x99, 0x02, 0x20,
    0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48, 0x65, 0x61, 0x72, 0x74,
    0x62, 0x65, 0x61, 0x74, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74,
    0x54, 0x78, 0x6e, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74,
    0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x69, 0x6e,
    0x0a, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20,
    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x6c, 0x65, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x69,
    0x6e, 0x61, 0x74, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x64, 0x69, 0x73, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x28, 0x69, 0x2e, 0x65, 0x2e, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x70, 0x65,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x47, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x48, 0x02,
    0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x48, 0x0b, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x2b, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x48, 0x34, 0x35, 0x0a, 0xa2, 0x01, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x4e, 0x00, 0x56, 0x01, 0x1a, 0x95, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x47, 0x43, 0x28, 0x29, 0x20,
    0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x27, 0x73, 0x0a, 0x20, 0x20, 0x73,
    0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x73, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x73, 0x63, 0x61, 0x6e, 0x6e,
    0x69, 0x6e, 0x67, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x74,
    0x6f, 0x20, 0x66, 0x69, 0x6e, 0x64, 0x20, 0x65, 0x78, 0x70, 0x69, 0x72, 0x65, 0x64, 0x0a, 0x20,
    0x20, 0x4d, 0x56, 0x43, 0x43, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04,
    0x03, 0x00, 0x12, 0x04, 0x4f, 0x02, 0x52, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x4f, 0x0a, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x50, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x50, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x50, 0x13, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x50, 0x19, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x51, 0x04, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x51, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x51, 0x0d, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x51, 0x28, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x51, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x53, 0x02,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x53, 0x0b, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x54, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x54,
    0x0b, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x27, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x31, 0x32, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x55, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x55, 0x0b, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x55, 0x34, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x55, 0x3b, 0x3c, 0x0a, 0x57, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5a, 0x00, 0x5c, 0x01, 0x1a,
    0x4b, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x47, 0x43, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x47, 0x43, 0x28,
    0x29, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x5b, 0x02, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x5b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5b, 0x0b,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x2b, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x34, 0x35, 0x0a, 0x83, 0x06,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x6a, 0x00, 0x78, 0x01, 0x1a, 0xf6, 0x05, 0x20, 0x41, 0x6e,
    0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x28, 0x29, 0x0a, 0x20, 0x20,
    0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x27, 0x73, 0x20, 0x73, 0x65, 0x6e,
    0x74, 0x20, 0x62, 0x79, 0x20, 0x72, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x20, 0x6f, 0x72, 0x20,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x72, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x68, 0x61,
    0x76, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x61,
    0x6e, 0x0a, 0x20, 0x20, 0x22, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x22, 0x20, 0x6c, 0x61, 0x69,
    0x64, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x67, 0x6f, 0x61, 0x6c, 0x20, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x6c, 0x69, 0x63, 0x74, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x61, 0x72, 0x67, 0x73, 0x2e, 0x4b, 0x65, 0x79, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78,
    0x6e, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x20, 0x61, 0x72, 0x67, 0x73, 0x2e, 0x50,
    0x75, 0x73, 0x68, 0x65, 0x65, 0x54, 0x78, 0x6e, 0x2c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x61, 0x72,
    0x67, 0x73, 0x2e, 0x54, 0x78, 0x6e, 0x2c, 0x20, 0x61, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73,
    0x75, 0x61, 0x6c, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x52, 0x50, 0x43, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
    0x6f, 0x77, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x75, 0x73, 0x68, 0x65, 0x65, 0x27,
    0x73, 0x20, 0x74, 0x78, 0x6e, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x2e, 0x0a, 0x20, 0x20,
    0x52, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x74, 0x72,
    0x69, 0x76, 0x69, 0x61, 0x6c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78, 0x6e,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6f, 0x77, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x68, 0x61, 0x73, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65,
    0x72, 0x0a, 0x20, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74,
    0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x61, 0x6c,
    0x72, 0x65, 0x61, 0x64, 0x79, 0x2e, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x78, 0x6e, 0x20, 0x63, 0x61, 0x6e, 0x0a, 0x20, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20,
    0x62, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x28, 0x66, 0x6f, 0x72, 0x20,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x2f, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x6c, 0x69, 0x63, 0x74, 0x73, 0x29, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x74, 0x73, 0x20, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x0a, 0x20, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x20, 0x66,
    0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x20, 0x28, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x61, 0x64,
    0x2f, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x6c, 0x69, 0x63, 0x74, 0x73,
    0x29, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x75, 0x72, 0x73, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x74,
    0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x70, 0x75, 0x73, 0x68, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x62, 0x79, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6f, 0x77, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x78, 0x6e, 0x27, 0x73, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74,
    0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x02, 0x35, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x6b, 0x0b, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x6b, 0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x6b, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x02,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x6c, 0x0b, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c, 0x28, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6c, 0x35, 0x36, 0x0a, 0xe2, 0x02, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x02, 0x12, 0x03, 0x73, 0x02, 0x36, 0x1a, 0xd4, 0x02, 0x20, 0x52, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x50,
    0x55, 0x53, 0x48, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x53, 0x54, 0x41, 0x4d, 0x50, 0x20, 0x74, 0x6f,
    0x20, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x50, 0x75, 0x73, 0x68, 0x65, 0x65, 0x54, 0x78, 0x6e, 0x27,
    0x73, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x0a, 0x20, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x20, 0x57, 0x72,
    0x69, 0x74, 0x65, 0x72, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x5f, 0x54, 0x58, 0x4e, 0x20, 0x74, 0x6f, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x0a, 0x20, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x62, 0x6f,
    0x72, 0x74, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65,
    0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x64, 0x6f, 0x6e, 0x65, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x0a, 0x20,
    0x20, 0x61, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x6c, 0x69,
    0x63, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x50, 0x75, 0x73, 0x68, 0x65,
    0x65, 0x54, 0x78, 0x6e, 0x2e, 0x20, 0x49, 0x6e, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65,
    0x6e, 0x74, 0x20, 0x72, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x20, 0x73, 0x65, 0x74, 0x0a, 0x20,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x43, 0x4c, 0x45, 0x41, 0x4e, 0x55, 0x50,
    0x5f, 0x54, 0x58, 0x4e, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e,
    0x65, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x64, 0x61, 0x6e, 0x67, 0x6c, 0x69,
    0x6e, 0x67, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x20, 0x20, 0x6d, 0x61, 0x79,
    0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x73, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x73, 0x28, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x73, 0x34, 0x35, 0x0a, 0xc2, 0x01, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12,
    0x03, 0x77, 0x02, 0x21, 0x1a, 0xb4, 0x01, 0x20, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6c, 0x6f,
    0x6f, 0x6b, 0x75, 0x70, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x77,
    0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x77, 0x65, 0x27, 0x72, 0x65, 0x20, 0x70, 0x75, 0x73,
    0x68, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x74, 0x78, 0x6e, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75,
    0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x77, 0x68,
    0x69, 0x6c, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e,
    0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20,
    0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x2e, 0x20, 0x53, 0x65, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x77, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x77, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x77, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x77, 0x1f, 0x20, 0x0a, 0xcd, 0x02, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0x80, 0x01, 0x00, 0x85,
    0x01, 0x01, 0x1a, 0xbe, 0x02, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x28,
    0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x72, 0x65, 0x74,
    0x75, 0x72, 0x6e, 0x73, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x20,
    0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x50, 0x75, 0x73, 0x68, 0x65, 0x65,
    0x54, 0x78, 0x6e, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x6c,
    0x69, 0x63, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x66, 0x61, 0x76, 0x6f, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x72, 0x3b, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x61, 0x6c, 0x6c, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x73, 0x75, 0x62,
    0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65,
    0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x28, 0x29, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x6c, 0x69, 0x63, 0x74, 0x65, 0x64, 0x20, 0x6b, 0x65, 0x79,
    0x2e, 0x20, 0x49, 0x74, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73,
    0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0x80, 0x01, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0x81, 0x01, 0x02, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x34, 0x35, 0x0a, 0x79, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x01, 0x12, 0x04, 0x84, 0x01, 0x02, 0x37, 0x1a, 0x6b, 0x20, 0x54, 0x78, 0x6e, 0x20, 0x69,
    0x73, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x6e, 0x69, 0x6c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x0a, 0x20, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x84, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x04, 0x84,
    0x01, 0x0b, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x04, 0x84, 0x01,
    0x28, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x04, 0x84, 0x01, 0x35,
    0x36, 0x0a, 0xfc, 0x01, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x8b, 0x01, 0x00, 0x8d, 0x01, 0x01,
    0x1a, 0xed, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x28,
    0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x6f,
    0x72, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x73, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x73, 0x68, 0x54, 0x78, 0x6e, 0x20, 0x74, 0x6f, 0x20,
    0x63, 0x6c, 0x65, 0x61, 0x6e, 0x20, 0x75, 0x70, 0x0a, 0x20, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x3a, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x6d, 0x20,
    0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x20, 0x74, 0x68, 0x65, 0x6d, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x8c, 0x01, 0x33, 0x34, 0x0a, 0x6f, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0x91,
    0x01, 0x00, 0x93, 0x01, 0x01, 0x1a, 0x61, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f,
    0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x28, 0x29, 0x20,
    0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12,
    0x04, 0x91, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0x92,
    0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x92, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x04, 0x92, 0x01, 0x0b,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x92, 0x01, 0x2b, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x92, 0x01, 0x34, 0x35, 0x0a,
    0xc8, 0x01, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0x98, 0x01, 0x00, 0x9a, 0x01, 0x01, 0x1a, 0xb9,
    0x01, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x20, 0x77, 0x72, 0x69,
    0x74, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x20, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x61, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x6b, 0x65, 0x79, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x20, 0x6f, 0x70, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x04, 0x98, 0x01, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12,
    0x04, 0x99, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x99, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x99,
    0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x01,
    0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x01, 0x33,
    0x34, 0x0a, 0x74, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x9e, 0x01, 0x00, 0xa0, 0x01, 0x01, 0x1a,
    0x66, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x28, 0x29, 0x20, 0x6d,
    0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04,
    0x9e, 0x01, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x9f, 0x01,
    0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9f, 0x01, 0x0b, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x2b, 0x31, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x34, 0x35, 0x0a, 0xb9,
    0x01, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xa8, 0x01, 0x01, 0x1a, 0xaa, 0x01,
    0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x72, 0x67,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x72, 0x67, 0x65,
    0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x0a, 0x20, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x61, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x65, 0x72, 0x67,
    0x65, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x69, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x61, 0x74, 0x0a, 0x20, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c,
    0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12,
    0x04, 0xa6, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xa6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa6,
    0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01,
    0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x33,
    0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa7, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x22, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x2a, 0x2b, 0x0a, 0x56, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x06, 0xab, 0x01, 0x00, 0xad, 0x01, 0x01, 0x1a, 0x48, 0x20, 0x49, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x4d, 0x65, 0x72, 0x67, 0x65, 0x28, 0x29, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x1d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0xac, 0x01, 0x02, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xac, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xac, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xac, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xac, 0x01, 0x34, 0x35, 0x0a, 0xb0, 0x03, 0x0a, 0x02, 0x04,
    0x0e, 0x12, 0x06, 0xb4, 0x01, 0x00, 0xb8, 0x01, 0x01, 0x1a, 0xa1, 0x03, 0x20, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x4c, 0x6f, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x61, 0x20, 0x70, 0x72, 0x65, 0x66,
    0x69, 0x78, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20, 0x6c,
    0x6f, 0x67, 0x2e, 0x20, 0x57, 0x68, 0x69, 0x6c, 0x65, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x0a,
    0x20, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x63, 0x74, 0x6e,
    0x65, 0x73, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x66,
    0x74, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x62, 0x65, 0x20, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x65, 0x64,
    0x20, 0x61, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x73, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x69, 0x63, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x79, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6c, 0x6c,
    0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x61, 0x73, 0x20, 0x63, 0x6c, 0x6f, 0x73,
    0x65, 0x0a, 0x20, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x6c,
    0x20, 0x61, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x2e, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x63, 0x61,
    0x6e, 0x20, 0x61, 0x6c, 0x73, 0x6f, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x64, 0x65,
    0x63, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x75, 0x74, 0x6f, 0x66, 0x66, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x0a, 0x20,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x69, 0x74, 0x73, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6c, 0x65,
    0x64, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x73, 0x27, 0x20, 0x61, 0x63, 0x6b, 0x6e, 0x6f, 0x77, 0x6c, 0x65, 0x64, 0x67, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x00, 0x12, 0x04, 0xb5, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xb5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xb5, 0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xb5, 0x01, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xb5, 0x01, 0x33, 0x34, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0xb7, 0x01,
    0x02, 0x1c, 0x1a, 0x2f, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73,
    0x20, 0x3c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x64, 0x69, 0x73, 0x63, 0x61, 0x72, 0x64, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb7, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x12, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x1a, 0x1b, 0x0a,
    0x62, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xbb, 0x01, 0x00, 0xbd, 0x01, 0x01, 0x1a, 0x54, 0x20,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65,
    0x4c, 0x6f, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61,
    0x74, 0x65, 0x4c, 0x6f, 0x67, 0x28, 0x29, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x01, 0x02, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbc, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x34, 0x35, 0x0a, 0xd3, 0x01, 0x0a, 0x02, 0x04,
    0x10, 0x12, 0x06, 0xc2, 0x01, 0x00, 0xc5, 0x01, 0x01, 0x1a, 0xc4, 0x01, 0x20, 0x41, 0x6e, 0x20,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x65,
    0x61, 0x73, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72,
    0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x65, 0x61,
    0x73, 0x65, 0x28, 0x29, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x62, 0x65, 0x68, 0x61, 0x6c, 0x66,
    0x20, 0x6f, 0x66, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x74, 0x73, 0x20, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x75, 0x70, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69,
    0x70, 0x74, 0x0a, 0x20, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x20, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xc3, 0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc3, 0x01, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12,
    0x04, 0xc4, 0x01, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xc4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc4,
    0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc4, 0x01,
    0x22, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x2a,
    0x2b, 0x0a, 0x67, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xc9, 0x01, 0x00, 0xcb, 0x01, 0x01, 0x1a,
    0x59, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x4c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x28, 0x29, 0x0a, 0x20, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11,
    0x01, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12,
    0x04, 0xca, 0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xca, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xca,
    0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xca, 0x01,
    0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xca, 0x01, 0x34,
    0x35, 0x0a, 0x97, 0x01, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xcf, 0x01, 0x00, 0xde, 0x01, 0x01,
    0x1a, 0x88, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x73, 0x20, 0x65, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x20, 0x4e, 0x6f, 0x6e,
    0x2d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73,
    0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x61,
    0x64, 0x64, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x12, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x12, 0x08, 0x00,
    0x12, 0x06, 0xd0, 0x01, 0x03, 0xdd, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x08, 0x00,
    0x01, 0x12, 0x04, 0xd0, 0x01, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12,
    0x04, 0xd1, 0x01, 0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xd1, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd1,
    0x01, 0x25, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd1, 0x01,
    0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x04, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd2, 0x01, 0x04, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x20, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x20, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x03,
    0x12, 0x04, 0xd4, 0x01, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x06, 0x12,
    0x04, 0xd4, 0x01, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xd4, 0x01, 0x2b, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd4,
    0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x04,
    0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x06, 0x12, 0x04, 0xd5, 0x01, 0x04, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x26, 0x2f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x32, 0x33, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x04, 0x2e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x05, 0x06, 0x12, 0x04, 0xd6, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x23, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x05, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02,
    0x06, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x06, 0x06,
    0x12, 0x04, 0xd7, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xd7, 0x01, 0x28, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x06, 0x03, 0x12, 0x04,
    0xd7, 0x01, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x07, 0x12, 0x04, 0xd8, 0x01,
    0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x07, 0x06, 0x12, 0x04, 0xd8, 0x01, 0x04,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x21, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x07, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x28, 0x29, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x08, 0x12, 0x04, 0xd9, 0x01, 0x04, 0x3f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x08, 0x06, 0x12, 0x04, 0xd9, 0x01, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x08, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x2b, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x08, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12,
    0x02, 0x09, 0x12, 0x04, 0xda, 0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x09,
    0x06, 0x12, 0x04, 0xda, 0x01, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x09, 0x01,
    0x12, 0x04, 0xda, 0x01, 0x2c, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x09, 0x03, 0x12,
    0x04, 0xda, 0x01, 0x40, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x0a, 0x12, 0x04, 0xdb,
    0x01, 0x04, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xdb, 0x01,
    0x04, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x32,
    0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x4c, 0x4e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x0b, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x5a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x0b, 0x06, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x36, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x37, 0x54, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x57, 0x59, 0x0a, 0x9a, 0x01, 0x0a, 0x02,
    0x04, 0x13, 0x12, 0x06, 0xe2, 0x01, 0x00, 0xf1, 0x01, 0x01, 0x1a, 0x8b, 0x01, 0x20, 0x41, 0x6e,
    0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20,
    0x65, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x0a, 0x20, 0x20, 0x4e, 0x6f, 0x6e, 0x2d, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x64, 0x64,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e,
    0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x61, 0x64, 0x64, 0x65,
    0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12,
    0x04, 0xe2, 0x01, 0x08, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x13, 0x08, 0x00, 0x12, 0x06, 0xe3,
    0x01, 0x03, 0xf0, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x08, 0x00, 0x01, 0x12, 0x04,
    0xe3, 0x01, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xe4, 0x01,
    0x04, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe4, 0x01, 0x04,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x26, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe4, 0x01, 0x31, 0x32, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x04, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe5, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x21, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe5, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13,
    0x02, 0x02, 0x12, 0x04, 0xe6, 0x01, 0x04, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xe6, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xe6, 0x01, 0x21, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xe6, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0xe7,
    0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x06, 0x12, 0x04, 0xe7, 0x01,
    0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x2c,
    0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x3e, 0x3f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x04, 0x12, 0x04, 0xe8, 0x01, 0x04, 0x35, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x06, 0x12, 0x04, 0xe8, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x27, 0x30, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x04, 0x03, 0x12, 0x04, 0xe8, 0x01, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x05, 0x12, 0x04, 0xe9, 0x01, 0x04, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x05, 0x06, 0x12, 0x04, 0xe9, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xe9, 0x01, 0x24, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x03,
    0x12, 0x04, 0xe9, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x06, 0x12, 0x04,
    0xea, 0x01, 0x04, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x06, 0x12, 0x04, 0xea,
    0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x01, 0x12, 0x04, 0xea, 0x01,
    0x29, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x03, 0x12, 0x04, 0xea, 0x01, 0x38,
    0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x07, 0x12, 0x04, 0xeb, 0x01, 0x04, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x07, 0x06, 0x12, 0x04, 0xeb, 0x01, 0x04, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x07, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x22, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x07, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x08, 0x12, 0x04, 0xec, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x08, 0x06, 0x12, 0x04, 0xec, 0x01, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x08, 0x01, 0x12, 0x04, 0xec, 0x01, 0x2c, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x08,
    0x03, 0x12, 0x04, 0xec, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x09, 0x12,
    0x04, 0xed, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x09, 0x06, 0x12, 0x04,
    0xed, 0x01, 0x04, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x09, 0x01, 0x12, 0x04, 0xed,
    0x01, 0x2d, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x09, 0x03, 0x12, 0x04, 0xed, 0x01,
    0x41, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x0a, 0x12, 0x04, 0xee, 0x01, 0x04, 0x50,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xee, 0x01, 0x04, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xee, 0x01, 0x33, 0x4a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xee, 0x01, 0x4d, 0x4f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x0b, 0x12, 0x04, 0xef, 0x01, 0x04, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x0b, 0x06, 0x12, 0x04, 0xef, 0x01, 0x04, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x0b, 0x01, 0x12, 0x04, 0xef, 0x01, 0x38, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x0b, 0x03, 0x12, 0x04, 0xef, 0x01, 0x58, 0x5a, 0x0a, 0x9e, 0x01, 0x0a, 0x02, 0x04, 0x14, 0x12,
    0x06, 0xf6, 0x01, 0x00, 0xf9, 0x01, 0x01, 0x1a, 0x8f, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x73, 0x75,
    0x70, 0x65, 0x72, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
    0x64, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x0a, 0x20, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x63, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x2e, 0x0a, 0x20, 0x20, 0x53, 0x65, 0x65, 0x20, 0x63, 0x6f,
    0x6d, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01,
    0x12, 0x04, 0xf6, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04,
    0xf7, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf7,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf7, 0x01,
    0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x2a,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x33, 0x34,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x02, 0x3e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf8, 0x01, 0x0b, 0x30, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x31, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x3c, 0x3d, 0x0a, 0x66, 0x0a, 0x02, 0x04, 0x15,
    0x12, 0x06, 0xfd, 0x01, 0x00, 0x80, 0x02, 0x01, 0x1a, 0x58, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x0a,
    0x20, 0x20, 0x53, 0x65, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xfe, 0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfe, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x01, 0x12, 0x04, 0xff, 0x01, 0x02, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xff, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xff, 0x01, 0x0b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xff, 0x01, 0x32, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xff,
    0x01, 0x3e, 0x3f, 0x0a, 0xc9, 0x01, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x85, 0x02, 0x00, 0x96,
    0x02, 0x01, 0x1a, 0xba, 0x01, 0x20, 0x41, 0x20, 0x52, 0x65, 0x61, 0x64, 0x57, 0x72, 0x69, 0x74,
    0x65, 0x43, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65,
    0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x0a, 0x20, 0x20, 0x6d, 0x75, 0x74, 0x61, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x2e, 0x20, 0x4e, 0x6f,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x65, 0x6e, 0x74, 0x72,
    0x79, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x6d, 0x75, 0x73,
    0x74, 0x20, 0x62, 0x65, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x69,
    0x6e, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x2f, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65,
    0x2f, 0x64, 0x62, 0x2e, 0x63, 0x63, 0x20, 0x69, 0x6e, 0x20, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x28, 0x29, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x85, 0x02, 0x08, 0x1c, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x16, 0x08, 0x00, 0x12, 0x06, 0x86, 0x02, 0x03, 0x95, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x08, 0x00, 0x01, 0x12, 0x04, 0x86, 0x02, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x00, 0x12, 0x04, 0x87, 0x02, 0x04, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x87, 0x02, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x87, 0x02, 0x21, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x87, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04,
    0x88, 0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x06, 0x12, 0x04, 0x88,
    0x02, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x88, 0x02,
    0x2c, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x88, 0x02, 0x3e,
    0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x02, 0x12, 0x04, 0x89, 0x02, 0x04, 0x35, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x06, 0x12, 0x04, 0x89, 0x02, 0x04, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01, 0x12, 0x04, 0x89, 0x02, 0x27, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12, 0x04, 0x89, 0x02, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x03, 0x06, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x24, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x8a, 0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x04, 0x12,
    0x04, 0x8b, 0x02, 0x04, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x8b, 0x02, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8b,
    0x02, 0x29, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8b, 0x02,
    0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x05, 0x12, 0x04, 0x8c, 0x02, 0x04, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x06, 0x12, 0x04, 0x8c, 0x02, 0x04, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x2c, 0x3b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x16, 0x02, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x06, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x06, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x32, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x06, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x4b, 0x4d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x07,
    0x12, 0x04, 0x8e, 0x02, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x06, 0x12,
    0x04, 0x8e, 0x02, 0x04, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x01, 0x12, 0x04,
    0x8e, 0x02, 0x2d, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x03, 0x12, 0x04, 0x8e,
    0x02, 0x41, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x08, 0x12, 0x04, 0x8f, 0x02, 0x04,
    0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x06, 0x12, 0x04, 0x8f, 0x02, 0x04, 0x32,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x33, 0x4a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x03, 0x12, 0x04, 0x8f, 0x02, 0x4d, 0x4f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x16, 0x02, 0x09, 0x12, 0x04, 0x90, 0x02, 0x04, 0x5b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x09, 0x06, 0x12, 0x04, 0x90, 0x02, 0x04, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x09, 0x01, 0x12, 0x04, 0x90, 0x02, 0x38, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x09, 0x03, 0x12, 0x04, 0x90, 0x02, 0x58, 0x5a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02,
    0x0a, 0x12, 0x04, 0x91, 0x02, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0a, 0x06,
    0x12, 0x04, 0x91, 0x02, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0a, 0x01, 0x12,
    0x04, 0x91, 0x02, 0x2b, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0a, 0x03, 0x12, 0x04,
    0x91, 0x02, 0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x0b, 0x12, 0x04, 0x92, 0x02,
    0x04, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0b, 0x06, 0x12, 0x04, 0x92, 0x02, 0x04,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x92, 0x02, 0x31, 0x46,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x92, 0x02, 0x49, 0x4b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x0c, 0x12, 0x04, 0x93, 0x02, 0x04, 0x39, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x0c, 0x06, 0x12, 0x04, 0x93, 0x02, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x93, 0x02, 0x28, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x93, 0x02, 0x36, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16,
    0x02, 0x0d, 0x12, 0x04, 0x94, 0x02, 0x04, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d,
    0x06, 0x12, 0x04, 0x94, 0x02, 0x04, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d, 0x01,
    0x12, 0x04, 0x94, 0x02, 0x31, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d, 0x03, 0x12,
    0x04, 0x94, 0x02, 0x49, 0x4b, 0x0a, 0x66, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x9a, 0x02, 0x00,
    0xb4, 0x02, 0x01, 0x1a, 0x58, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61,
    0x6c, 0x52, 0x61, 0x66, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x55, 0x6e, 0x69, 0x6f,
    0x6e, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x77,
    0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x20, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x76, 0x69, 0x61, 0x20, 0x72, 0x61, 0x66, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x08, 0x20, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x17,
    0x08, 0x00, 0x12, 0x06, 0x9b, 0x02, 0x03, 0xb3, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x08, 0x00, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x09, 0x0e, 0x0a, 0x58, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x00, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x32, 0x1a, 0x4a, 0x20, 0x4e, 0x6f, 0x6e, 0x2d, 0x62, 0x61,
    0x74, 0x63, 0x68, 0x65, 0x64, 0x20, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d,
    0x65, 0x20, 0x61, 0x73, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9d, 0x02,
    0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x25,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x30, 0x31,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x04, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9e, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x20, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x02, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x9f, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x9f, 0x02, 0x20, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x9f, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04,
    0xa0, 0x02, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa0,
    0x02, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa0, 0x02,
    0x2b, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x3d,
    0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x04, 0x12, 0x04, 0xa1, 0x02, 0x04, 0x34, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x06, 0x12, 0x04, 0xa1, 0x02, 0x04, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x26, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x17, 0x02, 0x05, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x05, 0x06, 0x12, 0x04, 0xa2, 0x02, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x05, 0x01, 0x12, 0x04, 0xa2, 0x02, 0x23, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05,
    0x03, 0x12, 0x04, 0xa2, 0x02, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x06, 0x12,
    0x04, 0xa3, 0x02, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06, 0x06, 0x12, 0x04,
    0xa3, 0x02, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06, 0x01, 0x12, 0x04, 0xa3,
    0x02, 0x28, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06, 0x03, 0x12, 0x04, 0xa3, 0x02,
    0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x07, 0x12, 0x04, 0xa4, 0x02, 0x04, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x06, 0x12, 0x04, 0xa4, 0x02, 0x04, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x21, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x17, 0x02, 0x08, 0x12, 0x04, 0xa5, 0x02, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x08, 0x06, 0x12, 0x04, 0xa5, 0x02, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x2b, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xa5, 0x02, 0x3d, 0x3e, 0x0a, 0x77, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x09,
    0x12, 0x04, 0xa8, 0x02, 0x04, 0x2d, 0x1a, 0x69, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x20, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x61,
    0x20, 0x67, 0x61, 0x70, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x61, 0x67, 0x20, 0x6e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x20, 0x73, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x65, 0x76, 0x69,
    0x6f, 0x75, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x0a, 0x20, 0x20, 0x62,
    0x65, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x2f, 0x70, 0x61, 0x73, 0x74, 0x65, 0x64, 0x20, 0x66, 0x72,
    0x6f, 0x6d, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x09, 0x06, 0x12, 0x04, 0xa8, 0x02, 0x04, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x09, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x22, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x09, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x2a, 0x2c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x17, 0x02, 0x0a, 0x12, 0x04, 0xa9, 0x02, 0x04, 0x4b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xa9, 0x02, 0x04, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x30, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x0a, 0x03, 0x12, 0x04, 0xa9, 0x02, 0x48, 0x4a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x0b, 0x12, 0x04, 0xaa, 0x02, 0x04, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0b, 0x06,
    0x12, 0x04, 0xaa, 0x02, 0x04, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0b, 0x01, 0x12,
    0x04, 0xaa, 0x02, 0x31, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0b, 0x03, 0x12, 0x04,
    0xaa, 0x02, 0x4a, 0x4c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x0c, 0x12, 0x04, 0xab, 0x02,
    0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0c, 0x06, 0x12, 0x04, 0xab, 0x02, 0x04,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xab, 0x02, 0x2c, 0x3d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xab, 0x02, 0x40, 0x42, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x0d, 0x12, 0x04, 0xac, 0x02, 0x04, 0x4f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x0d, 0x06, 0x12, 0x04, 0xac, 0x02, 0x04, 0x31, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xac, 0x02, 0x32, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xac, 0x02, 0x4c, 0x4e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x0e, 0x12, 0x04, 0xad, 0x02, 0x04, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0e,
    0x06, 0x12, 0x04, 0xad, 0x02, 0x04, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0e, 0x01,
    0x12, 0x04, 0xad, 0x02, 0x37, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0e, 0x03, 0x12,
    0x04, 0xad, 0x02, 0x57, 0x59, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x0f, 0x12, 0x04, 0xae,
    0x02, 0x04, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0f, 0x06, 0x12, 0x04, 0xae, 0x02,
    0x04, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xae, 0x02, 0x2a,
    0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xae, 0x02, 0x44, 0x46,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x10, 0x12, 0x04, 0xaf, 0x02, 0x04, 0x4b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x10, 0x06, 0x12, 0x04, 0xaf, 0x02, 0x04, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x10, 0x01, 0x12, 0x04, 0xaf, 0x02, 0x30, 0x45, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x10, 0x03, 0x12, 0x04, 0xaf, 0x02, 0x48, 0x4a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x11, 0x12, 0x04, 0xb0, 0x02, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x11, 0x06, 0x12, 0x04, 0xb0, 0x02, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x11,
    0x01, 0x12, 0x04, 0xb0, 0x02, 0x27, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x11, 0x03,
    0x12, 0x04, 0xb0, 0x02, 0x35, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x12, 0x12, 0x04,
    0xb1, 0x02, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x12, 0x06, 0x12, 0x04, 0xb1,
    0x02, 0x04, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x12, 0x01, 0x12, 0x04, 0xb1, 0x02,
    0x30, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x12, 0x03, 0x12, 0x04, 0xb1, 0x02, 0x41,
    0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x13, 0x12, 0x04, 0xb2, 0x02, 0x04, 0x3e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x13, 0x06, 0x12, 0x04, 0xb2, 0x02, 0x04, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x13, 0x01, 0x12, 0x04, 0xb2, 0x02, 0x2a, 0x38, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x13, 0x03, 0x12, 0x04, 0xb2, 0x02, 0x3b, 0x3d, 0x0a, 0x60, 0x0a, 0x02,
    0x04, 0x18, 0x12, 0x06, 0xb8, 0x02, 0x00, 0xbc, 0x02, 0x01, 0x1a, 0x52, 0x20, 0x41, 0x6e, 0x20,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x61, 0x66, 0x74, 0x43, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65,
    0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x20, 0x73,
    0x65, 0x6e, 0x74, 0x20, 0x76, 0x69, 0x61, 0x20, 0x72, 0x61, 0x66, 0x74, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x18, 0x02, 0x00, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xb9, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xb9, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xb9, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0xba,
    0x02, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xba, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12, 0x04, 0xba, 0x02, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xba, 0x02, 0x12, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xba, 0x02, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x02, 0x12, 0x04, 0xbb, 0x02, 0x02, 0x3d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x02, 0x06, 0x12, 0x04, 0xbb, 0x02, 0x0b, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x02, 0x35, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x02, 0x3b, 0x3c, 0x0a, 0xd0, 0x02, 0x0a, 0x02, 0x04, 0x19,
    0x12, 0x06, 0xc3, 0x02, 0x00, 0xc9, 0x02, 0x01, 0x1a, 0xc1, 0x02, 0x20, 0x52, 0x61, 0x66, 0x74,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x6f,
    0x75, 0x72, 0x0a, 0x20, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2d, 0x62, 0x61,
    0x73, 0x65, 0x64, 0x20, 0x52, 0x50, 0x43, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x2e, 0x20, 0x55,
    0x6e, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a,
    0x20, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x70, 0x69, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2c,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x65,
    0x70, 0x61, 0x72, 0x61, 0x74, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x67, 0x6f, 0x2e,
    0x0a, 0x20, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65,
    0x71, 0x75, 0x69, 0x76, 0x61, 0x6c, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x6d, 0x75,
    0x6c, 0x74, 0x69, 0x72, 0x61, 0x66, 0x74, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x19, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02,
    0x00, 0x12, 0x04, 0xc4, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xc4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xc4, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xc4, 0x02, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc4,
    0x02, 0x1d, 0x1e, 0x0a, 0xad, 0x01, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x02,
    0x02, 0x19, 0x1a, 0x9e, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20, 0x70,
    0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64,
    0x65, 0x64, 0x20, 0x72, 0x61, 0x66, 0x74, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x2e, 0x20, 0x57, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x6d, 0x69, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x61, 0x73, 0x0a, 0x20, 0x20,
    0x61, 0x6e, 0x20, 0x6f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x62, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70,
    0x6c, 0x65, 0x78, 0x69, 0x74, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x20,
    0x61, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x0a, 0x20, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc8, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc8, 0x02, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc8, 0x02, 0x11, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc8, 0x02, 0x17, 0x18, 0x0a,
    0x4e, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xcc, 0x02, 0x00, 0xcd, 0x02, 0x01, 0x1a, 0x40, 0x20,
    0x52, 0x61, 0x66, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20, 0x52, 0x50, 0x43, 0x73, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xcc, 0x02, 0x08, 0x1b, 0x0a, 0xa7, 0x08, 0x0a,
    0x02, 0x04, 0x1b, 0x12, 0x06, 0xde, 0x02, 0x00, 0xe7, 0x02, 0x01, 0x1a, 0x98, 0x08, 0x20, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65,
    0x73, 0x44, 0x61, 0x74, 0x61, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x61,
    0x6d, 0x70, 0x6c, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x6d,
    0x65, 0x61, 0x73, 0x75, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x0a, 0x20, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x2c, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x73, 0x61,
    0x6d, 0x70, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x6e, 0x20, 0x6f, 0x76,
    0x65, 0x72, 0x20, 0x61, 0x20, 0x75, 0x6e, 0x69, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x74, 0x69, 0x6d,
    0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x74, 0x73,
    0x65, 0x6c, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x28,
    0x69, 0x6e, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x78, 0x0a, 0x20, 0x20, 0x65, 0x70, 0x6f, 0x63,
    0x68, 0x29, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20,
    0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x28, 0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c,
    0x6c, 0x69, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x29, 0x2e, 0x20, 0x45, 0x61, 0x63, 0x68,
    0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x66, 0x73,
    0x65, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x0a, 0x20, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x69, 0x6d,
    0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x0a, 0x20, 0x20, 0x62, 0x65, 0x67, 0x61, 0x6e, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x77, 0x68, 0x6f, 0x6c, 0x65,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x73, 0x2e, 0x20, 0x46, 0x6f, 0x72,
    0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x36, 0x30, 0x30, 0x30, 0x30, 0x20, 0x28, 0x69, 0x6e, 0x64,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x31, 0x20, 0x6d, 0x69, 0x6e, 0x75, 0x74, 0x65,
    0x29, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x65, 0x64, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a,
    0x20, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x35, 0x20, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x73, 0x20, 0x28, 0x35,
    0x2a, 0x36, 0x30, 0x30, 0x30, 0x30, 0x6d, 0x73, 0x20, 0x3d, 0x20, 0x33, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x6d, 0x73, 0x20, 0x3d, 0x20, 0x35, 0x20, 0x6d, 0x69, 0x6e, 0x75, 0x74, 0x65, 0x73, 0x29,
    0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65,
    0x20, 0x61, 0x6e, 0x20, 0x65, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x73, 0x65,
    0x72, 0x69, 0x65, 0x73, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2c, 0x0a, 0x20, 0x20, 0x65, 0x6e, 0x73,
    0x75, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x76, 0x65, 0x72, 0x79, 0x20,
    0x6c, 0x69, 0x74, 0x74, 0x6c, 0x65, 0x20, 0x72, 0x65, 0x64, 0x75, 0x6e, 0x64, 0x61, 0x6e, 0x74,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20,
    0x6f, 0x6e, 0x20, 0x64, 0x69, 0x73, 0x6b, 0x2e, 0x20, 0x57, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x67, 0x6f, 0x61, 0x6c, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x20, 0x6d, 0x69, 0x6e,
    0x64, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66,
    0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x77,
    0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79,
    0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65,
    0x64, 0x3b, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x0a, 0x20, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xde,
    0x02, 0x08, 0x1e, 0x0a, 0xaa, 0x01, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xe2, 0x02,
    0x02, 0x2b, 0x1a, 0x9b, 0x01, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x20, 0x77, 0x61,
    0x6c, 0x6c, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x75, 0x6e, 0x69, 0x78, 0x20, 0x65, 0x70, 0x6f,
    0x63, 0x68, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x6e, 0x61, 0x6e, 0x6f, 0x73,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x0a, 0x20, 0x20, 0x72,
    0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x61,
    0x72, 0x6c, 0x69, 0x65, 0x73, 0x74, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe2, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe2, 0x02, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x11, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe2, 0x02, 0x29, 0x2a, 0x0a, 0x4f, 0x0a, 0x04,
    0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0xe4, 0x02, 0x02, 0x2b, 0x1a, 0x41, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x61, 0x63,
    0x68, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x6e, 0x61, 0x6e, 0x6f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe4, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe4, 0x02, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xe4, 0x02, 0x29, 0x2a, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x1b, 0x02,
    0x02, 0x12, 0x04, 0xe6, 0x02, 0x02, 0x41, 0x1a, 0x2a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x61, 0x63,
    0x74, 0x75, 0x61, 0x6c, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69,
    0x63, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe6, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x06, 0x12, 0x04, 0xe6, 0x02, 0x0b,
    0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe6, 0x02, 0x35, 0x3c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe6, 0x02, 0x3f, 0x40, 0x0a,
    0xbe, 0x07, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xf9, 0x02, 0x00, 0x87, 0x03, 0x01, 0x1a, 0xaf,
    0x07, 0x20, 0x41, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65,
    0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x72, 0x65, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x67, 0x61, 0x74,
    0x68, 0x65, 0x72, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69,
    0x70, 0x6c, 0x65, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x61, 0x20, 0x67, 0x69,
    0x76, 0x65, 0x6e, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69,
    0x6d, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x6f,
    0x66, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x54, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x44, 0x61, 0x74, 0x61, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x3b, 0x20, 0x61, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65,
    0x0a, 0x20, 0x20, 0x63, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x70, 0x72, 0x65, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x63, 0x74,
    0x6c, 0x79, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x20, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x0a, 0x20, 0x20, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x64, 0x61,
    0x74, 0x61, 0x20, 0x67, 0x61, 0x74, 0x68, 0x65, 0x72, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61,
    0x6d, 0x65, 0x0a, 0x20, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x2c, 0x20, 0x61,
    0x73, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x61, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x6f, 0x73, 0x65, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x0a, 0x20, 0x20,
    0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x70,
    0x6c, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x20, 0x73, 0x65, 0x76, 0x65, 0x72, 0x61,
    0x6c, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20, 0x6d,
    0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x3a, 0x0a, 0x20, 0x20, 0x2d,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x75, 0x6d, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20,
    0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x0a,
    0x20, 0x20, 0x2d, 0x20, 0x41, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x6c, 0x6c, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20,
    0x74, 0x61, 0x6b, 0x65, 0x6e, 0x0a, 0x20, 0x20, 0x2d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61,
    0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c,
    0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x65, 0x65,
    0x6e, 0x0a, 0x20, 0x20, 0x2d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75,
    0x6d, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x20, 0x6d, 0x65, 0x61,
    0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x65, 0x65, 0x6e, 0x0a, 0x20, 0x20,
    0x49, 0x66, 0x20, 0x7a, 0x65, 0x72, 0x6f, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74,
    0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x74, 0x68,
    0x65, 0x6e, 0x20, 0x69, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x6f, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x72, 0x65,
    0x6c, 0x79, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x63, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x74, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x61, 0x20, 0x70, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x2e, 0x0a, 0x20, 0x20,
    0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x31, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x6d, 0x69, 0x6e, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62,
    0x65, 0x20, 0x6f, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x64, 0x20, 0x65, 0x71, 0x75, 0x61, 0x6c, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x6d, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x20, 0x0a, 0x88, 0x02,
    0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xfe, 0x02, 0x02, 0x1c, 0x1a, 0xf9, 0x01, 0x20,
    0x54, 0x65, 0x6d, 0x70, 0x6f, 0x72, 0x61, 0x6c, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x22, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x53, 0x65,
    0x72, 0x69, 0x65, 0x73, 0x44, 0x61, 0x74, 0x61, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x69, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x20, 0x69, 0x6e,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x20,
    0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x22,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x22, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54, 0x69, 0x6d,
    0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x44, 0x61, 0x74, 0x61, 0x20, 0x63, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xfe, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xfe, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xfe, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xfe, 0x02, 0x1a, 0x1b, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04, 0x80, 0x03,
    0x02, 0x1c, 0x1a, 0x31, 0x20, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65,
    0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x6e,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x80, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x80,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x03,
    0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x03, 0x1a,
    0x1b, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12, 0x04, 0x82, 0x03, 0x02, 0x1a, 0x1a,
    0x1a, 0x20, 0x53, 0x75, 0x6d, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6d, 0x65, 0x61,
    0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x82, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x02, 0x05, 0x12, 0x04, 0x82, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x02, 0x01, 0x12, 0x04, 0x82, 0x03, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x82, 0x03, 0x18, 0x19, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x03, 0x12,
    0x04, 0x84, 0x03, 0x02, 0x1a, 0x1a, 0x31, 0x20, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20,
    0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x61, 0x73,
    0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x84, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x05,
    0x12, 0x04, 0x84, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x84, 0x03, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x84, 0x03, 0x18, 0x19, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x04, 0x12, 0x04, 0x86, 0x03,
    0x02, 0x1a, 0x1a, 0x31, 0x20, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x65, 0x6e, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x86, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x04, 0x05, 0x12, 0x04, 0x86,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x04, 0x01, 0x12, 0x04, 0x86, 0x03,
    0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x04, 0x03, 0x12, 0x04, 0x86, 0x03, 0x18,
    0x19, 0x0a, 0xdc, 0x01, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0x8c, 0x03, 0x00, 0x91, 0x03, 0x01,
    0x1a, 0xcd, 0x01, 0x20, 0x52, 0x61, 0x66, 0x74, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65,
    0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20,
    0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x70, 0x6f, 0x72,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x66, 0x74,
    0x20, 0x6c, 0x6f, 0x67, 0x2e, 0x0a, 0x20, 0x20, 0x52, 0x61, 0x66, 0x74, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x73, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x65, 0x72, 0x6d, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6c, 0x61, 0x73, 0x74, 0x20, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6c,
    0x6f, 0x67, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x20, 0x61, 0x66,
    0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x73, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x68, 0x61, 0x73, 0x20,
    0x62, 0x65, 0x65, 0x6e, 0x20, 0x64, 0x69, 0x73, 0x63, 0x61, 0x72, 0x64, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0x8c, 0x03, 0x08, 0x1a, 0x0a, 0x45, 0x0a,
    0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x03, 0x02, 0x1c, 0x1a, 0x37, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x68, 0x69, 0x67, 0x68, 0x65, 0x73, 0x74, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x68, 0x61, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x72, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c,
    0x6f, 0x67, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8e,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8e, 0x03,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x12,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8e, 0x03, 0x1a, 0x1b,
    0x0a, 0x32, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0x90, 0x03, 0x02, 0x1b, 0x1a, 0x24,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x65, 0x72, 0x6d, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x27, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x27, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x90,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x05, 0x12, 0x04, 0x90, 0x03,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x90, 0x03, 0x12,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x90, 0x03, 0x19, 0x1a,
    0x0a, 0xb5, 0x01, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0x95, 0x03, 0x00, 0x9b, 0x03, 0x01, 0x1a,
    0xa6, 0x01, 0x20, 0x52, 0x61, 0x66, 0x74, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x44,
    0x61, 0x74, 0x61, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f,
    0x61, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x72, 0x61, 0x66, 0x74, 0x70, 0x62, 0x2e, 0x53,
    0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x72, 0x61, 0x77, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20,
    0x6f, 0x66, 0x0a, 0x20, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x27, 0x73, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20, 0x6c, 0x6f,
    0x67, 0x2c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x61, 0x63, 0x68,
    0x65, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12,
    0x04, 0x95, 0x03, 0x08, 0x18, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1e, 0x03, 0x00, 0x12, 0x06, 0x96,
    0x03, 0x02, 0x99, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x03, 0x00, 0x01, 0x12, 0x04,
    0x96, 0x03, 0x0a, 0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x97, 0x03, 0x04, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x97, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x97, 0x03, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x97, 0x03, 0x13, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x97, 0x03, 0x19, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x04, 0x98, 0x03, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x98, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x98, 0x03, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x98, 0x03, 0x13, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1e, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x98, 0x03, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0x9a, 0x03, 0x02, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x9a, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x9a, 0x03, 0x0b, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x9a, 0x03, 0x36, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x9a, 0x03, 0x3b, 0x3c,
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
