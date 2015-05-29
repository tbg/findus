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
pub struct NotLeaderError {
    // message fields
    replica: ::protobuf::SingularPtrField<Replica>,
    leader: ::protobuf::SingularPtrField<Replica>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl NotLeaderError {
    pub fn new() -> NotLeaderError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NotLeaderError {
        static mut instance: ::protobuf::lazy::Lazy<NotLeaderError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NotLeaderError,
        };
        unsafe {
            instance.get(|| {
                NotLeaderError {
                    replica: ::protobuf::SingularPtrField::none(),
                    leader: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Replica replica = 1;

    pub fn clear_replica(&mut self) {
        self.replica.clear();
    }

    pub fn has_replica(&self) -> bool {
        self.replica.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replica(&mut self, v: Replica) {
        self.replica = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replica<'a>(&'a mut self) -> &'a mut Replica {
        if self.replica.is_none() {
            self.replica.set_default();
        };
        self.replica.as_mut().unwrap()
    }

    // Take field
    pub fn take_replica(&mut self) -> Replica {
        self.replica.take().unwrap_or_else(|| Replica::new())
    }

    pub fn get_replica<'a>(&'a self) -> &'a Replica {
        self.replica.as_ref().unwrap_or_else(|| Replica::default_instance())
    }

    // optional .cockroach.proto.Replica leader = 2;

    pub fn clear_leader(&mut self) {
        self.leader.clear();
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: Replica) {
        self.leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader<'a>(&'a mut self) -> &'a mut Replica {
        if self.leader.is_none() {
            self.leader.set_default();
        };
        self.leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader(&mut self) -> Replica {
        self.leader.take().unwrap_or_else(|| Replica::new())
    }

    pub fn get_leader<'a>(&'a self) -> &'a Replica {
        self.leader.as_ref().unwrap_or_else(|| Replica::default_instance())
    }
}

impl ::protobuf::Message for NotLeaderError {
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
                    let tmp = self.replica.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.leader.set_default();
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
        for value in self.replica.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.leader.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replica.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.leader.as_ref() {
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
        ::std::any::TypeId::of::<NotLeaderError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NotLeaderError {
    fn new() -> NotLeaderError {
        NotLeaderError::new()
    }

    fn descriptor_static(_: ::std::option::Option<NotLeaderError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "replica",
                    NotLeaderError::has_replica,
                    NotLeaderError::get_replica,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "leader",
                    NotLeaderError::has_leader,
                    NotLeaderError::get_leader,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NotLeaderError>(
                    "NotLeaderError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NotLeaderError {
    fn clear(&mut self) {
        self.clear_replica();
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NotLeaderError {
    fn eq(&self, other: &NotLeaderError) -> bool {
        self.replica == other.replica &&
        self.leader == other.leader &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NotLeaderError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RangeNotFoundError {
    // message fields
    raft_id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RangeNotFoundError {
    pub fn new() -> RangeNotFoundError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RangeNotFoundError {
        static mut instance: ::protobuf::lazy::Lazy<RangeNotFoundError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RangeNotFoundError,
        };
        unsafe {
            instance.get(|| {
                RangeNotFoundError {
                    raft_id: ::std::option::Option::None,
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
}

impl ::protobuf::Message for RangeNotFoundError {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.raft_id {
            try!(os.write_int64(1, v));
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
        ::std::any::TypeId::of::<RangeNotFoundError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RangeNotFoundError {
    fn new() -> RangeNotFoundError {
        RangeNotFoundError::new()
    }

    fn descriptor_static(_: ::std::option::Option<RangeNotFoundError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "raft_id",
                    RangeNotFoundError::has_raft_id,
                    RangeNotFoundError::get_raft_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RangeNotFoundError>(
                    "RangeNotFoundError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RangeNotFoundError {
    fn clear(&mut self) {
        self.clear_raft_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RangeNotFoundError {
    fn eq(&self, other: &RangeNotFoundError) -> bool {
        self.raft_id == other.raft_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RangeNotFoundError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RangeKeyMismatchError {
    // message fields
    request_start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    request_end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    range: ::protobuf::SingularPtrField<RangeDescriptor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RangeKeyMismatchError {
    pub fn new() -> RangeKeyMismatchError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RangeKeyMismatchError {
        static mut instance: ::protobuf::lazy::Lazy<RangeKeyMismatchError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RangeKeyMismatchError,
        };
        unsafe {
            instance.get(|| {
                RangeKeyMismatchError {
                    request_start_key: ::protobuf::SingularField::none(),
                    request_end_key: ::protobuf::SingularField::none(),
                    range: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes request_start_key = 1;

    pub fn clear_request_start_key(&mut self) {
        self.request_start_key.clear();
    }

    pub fn has_request_start_key(&self) -> bool {
        self.request_start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.request_start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_start_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.request_start_key.is_none() {
            self.request_start_key.set_default();
        };
        self.request_start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_request_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.request_start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_request_start_key<'a>(&'a self) -> &'a [u8] {
        match self.request_start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes request_end_key = 2;

    pub fn clear_request_end_key(&mut self) {
        self.request_end_key.clear();
    }

    pub fn has_request_end_key(&self) -> bool {
        self.request_end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.request_end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_end_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.request_end_key.is_none() {
            self.request_end_key.set_default();
        };
        self.request_end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_request_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.request_end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_request_end_key<'a>(&'a self) -> &'a [u8] {
        match self.request_end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .cockroach.proto.RangeDescriptor range = 3;

    pub fn clear_range(&mut self) {
        self.range.clear();
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: RangeDescriptor) {
        self.range = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range<'a>(&'a mut self) -> &'a mut RangeDescriptor {
        if self.range.is_none() {
            self.range.set_default();
        };
        self.range.as_mut().unwrap()
    }

    // Take field
    pub fn take_range(&mut self) -> RangeDescriptor {
        self.range.take().unwrap_or_else(|| RangeDescriptor::new())
    }

    pub fn get_range<'a>(&'a self) -> &'a RangeDescriptor {
        self.range.as_ref().unwrap_or_else(|| RangeDescriptor::default_instance())
    }
}

impl ::protobuf::Message for RangeKeyMismatchError {
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
                    let tmp = self.request_start_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.request_end_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.range.set_default();
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
        for value in self.request_start_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.request_end_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.range.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_start_key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.request_end_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.range.as_ref() {
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
        ::std::any::TypeId::of::<RangeKeyMismatchError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RangeKeyMismatchError {
    fn new() -> RangeKeyMismatchError {
        RangeKeyMismatchError::new()
    }

    fn descriptor_static(_: ::std::option::Option<RangeKeyMismatchError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "request_start_key",
                    RangeKeyMismatchError::has_request_start_key,
                    RangeKeyMismatchError::get_request_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "request_end_key",
                    RangeKeyMismatchError::has_request_end_key,
                    RangeKeyMismatchError::get_request_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "range",
                    RangeKeyMismatchError::has_range,
                    RangeKeyMismatchError::get_range,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RangeKeyMismatchError>(
                    "RangeKeyMismatchError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RangeKeyMismatchError {
    fn clear(&mut self) {
        self.clear_request_start_key();
        self.clear_request_end_key();
        self.clear_range();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RangeKeyMismatchError {
    fn eq(&self, other: &RangeKeyMismatchError) -> bool {
        self.request_start_key == other.request_start_key &&
        self.request_end_key == other.request_end_key &&
        self.range == other.range &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RangeKeyMismatchError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReadWithinUncertaintyIntervalError {
    // message fields
    timestamp: ::protobuf::SingularPtrField<Timestamp>,
    existing_timestamp: ::protobuf::SingularPtrField<Timestamp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReadWithinUncertaintyIntervalError {
    pub fn new() -> ReadWithinUncertaintyIntervalError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadWithinUncertaintyIntervalError {
        static mut instance: ::protobuf::lazy::Lazy<ReadWithinUncertaintyIntervalError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadWithinUncertaintyIntervalError,
        };
        unsafe {
            instance.get(|| {
                ReadWithinUncertaintyIntervalError {
                    timestamp: ::protobuf::SingularPtrField::none(),
                    existing_timestamp: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Timestamp timestamp = 1;

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

    // optional .cockroach.proto.Timestamp existing_timestamp = 2;

    pub fn clear_existing_timestamp(&mut self) {
        self.existing_timestamp.clear();
    }

    pub fn has_existing_timestamp(&self) -> bool {
        self.existing_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_existing_timestamp(&mut self, v: Timestamp) {
        self.existing_timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_existing_timestamp<'a>(&'a mut self) -> &'a mut Timestamp {
        if self.existing_timestamp.is_none() {
            self.existing_timestamp.set_default();
        };
        self.existing_timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_existing_timestamp(&mut self) -> Timestamp {
        self.existing_timestamp.take().unwrap_or_else(|| Timestamp::new())
    }

    pub fn get_existing_timestamp<'a>(&'a self) -> &'a Timestamp {
        self.existing_timestamp.as_ref().unwrap_or_else(|| Timestamp::default_instance())
    }
}

impl ::protobuf::Message for ReadWithinUncertaintyIntervalError {
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
                    let tmp = self.timestamp.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.existing_timestamp.set_default();
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
        for value in self.timestamp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.existing_timestamp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.existing_timestamp.as_ref() {
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
        ::std::any::TypeId::of::<ReadWithinUncertaintyIntervalError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadWithinUncertaintyIntervalError {
    fn new() -> ReadWithinUncertaintyIntervalError {
        ReadWithinUncertaintyIntervalError::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadWithinUncertaintyIntervalError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "timestamp",
                    ReadWithinUncertaintyIntervalError::has_timestamp,
                    ReadWithinUncertaintyIntervalError::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "existing_timestamp",
                    ReadWithinUncertaintyIntervalError::has_existing_timestamp,
                    ReadWithinUncertaintyIntervalError::get_existing_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadWithinUncertaintyIntervalError>(
                    "ReadWithinUncertaintyIntervalError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadWithinUncertaintyIntervalError {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_existing_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReadWithinUncertaintyIntervalError {
    fn eq(&self, other: &ReadWithinUncertaintyIntervalError) -> bool {
        self.timestamp == other.timestamp &&
        self.existing_timestamp == other.existing_timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReadWithinUncertaintyIntervalError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TransactionAbortedError {
    // message fields
    txn: ::protobuf::SingularPtrField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TransactionAbortedError {
    pub fn new() -> TransactionAbortedError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionAbortedError {
        static mut instance: ::protobuf::lazy::Lazy<TransactionAbortedError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionAbortedError,
        };
        unsafe {
            instance.get(|| {
                TransactionAbortedError {
                    txn: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Transaction txn = 1;

    pub fn clear_txn(&mut self) {
        self.txn.clear();
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: Transaction) {
        self.txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txn<'a>(&'a mut self) -> &'a mut Transaction {
        if self.txn.is_none() {
            self.txn.set_default();
        };
        self.txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_txn(&mut self) -> Transaction {
        self.txn.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_txn<'a>(&'a self) -> &'a Transaction {
        self.txn.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }
}

impl ::protobuf::Message for TransactionAbortedError {
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
                    let tmp = self.txn.set_default();
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
        for value in self.txn.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txn.as_ref() {
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
        ::std::any::TypeId::of::<TransactionAbortedError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionAbortedError {
    fn new() -> TransactionAbortedError {
        TransactionAbortedError::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionAbortedError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "txn",
                    TransactionAbortedError::has_txn,
                    TransactionAbortedError::get_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionAbortedError>(
                    "TransactionAbortedError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionAbortedError {
    fn clear(&mut self) {
        self.clear_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TransactionAbortedError {
    fn eq(&self, other: &TransactionAbortedError) -> bool {
        self.txn == other.txn &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TransactionAbortedError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TransactionPushError {
    // message fields
    txn: ::protobuf::SingularPtrField<Transaction>,
    pushee_txn: ::protobuf::SingularPtrField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TransactionPushError {
    pub fn new() -> TransactionPushError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionPushError {
        static mut instance: ::protobuf::lazy::Lazy<TransactionPushError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionPushError,
        };
        unsafe {
            instance.get(|| {
                TransactionPushError {
                    txn: ::protobuf::SingularPtrField::none(),
                    pushee_txn: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Transaction txn = 1;

    pub fn clear_txn(&mut self) {
        self.txn.clear();
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: Transaction) {
        self.txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txn<'a>(&'a mut self) -> &'a mut Transaction {
        if self.txn.is_none() {
            self.txn.set_default();
        };
        self.txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_txn(&mut self) -> Transaction {
        self.txn.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_txn<'a>(&'a self) -> &'a Transaction {
        self.txn.as_ref().unwrap_or_else(|| Transaction::default_instance())
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

impl ::protobuf::Message for TransactionPushError {
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
                    let tmp = self.txn.set_default();
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
        for value in self.txn.iter() {
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
        if let Some(v) = self.txn.as_ref() {
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
        ::std::any::TypeId::of::<TransactionPushError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionPushError {
    fn new() -> TransactionPushError {
        TransactionPushError::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionPushError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "txn",
                    TransactionPushError::has_txn,
                    TransactionPushError::get_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pushee_txn",
                    TransactionPushError::has_pushee_txn,
                    TransactionPushError::get_pushee_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionPushError>(
                    "TransactionPushError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionPushError {
    fn clear(&mut self) {
        self.clear_txn();
        self.clear_pushee_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TransactionPushError {
    fn eq(&self, other: &TransactionPushError) -> bool {
        self.txn == other.txn &&
        self.pushee_txn == other.pushee_txn &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TransactionPushError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TransactionRetryError {
    // message fields
    txn: ::protobuf::SingularPtrField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TransactionRetryError {
    pub fn new() -> TransactionRetryError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionRetryError {
        static mut instance: ::protobuf::lazy::Lazy<TransactionRetryError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionRetryError,
        };
        unsafe {
            instance.get(|| {
                TransactionRetryError {
                    txn: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Transaction txn = 1;

    pub fn clear_txn(&mut self) {
        self.txn.clear();
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: Transaction) {
        self.txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txn<'a>(&'a mut self) -> &'a mut Transaction {
        if self.txn.is_none() {
            self.txn.set_default();
        };
        self.txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_txn(&mut self) -> Transaction {
        self.txn.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_txn<'a>(&'a self) -> &'a Transaction {
        self.txn.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }
}

impl ::protobuf::Message for TransactionRetryError {
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
                    let tmp = self.txn.set_default();
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
        for value in self.txn.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txn.as_ref() {
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
        ::std::any::TypeId::of::<TransactionRetryError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionRetryError {
    fn new() -> TransactionRetryError {
        TransactionRetryError::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionRetryError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "txn",
                    TransactionRetryError::has_txn,
                    TransactionRetryError::get_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionRetryError>(
                    "TransactionRetryError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionRetryError {
    fn clear(&mut self) {
        self.clear_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TransactionRetryError {
    fn eq(&self, other: &TransactionRetryError) -> bool {
        self.txn == other.txn &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TransactionRetryError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TransactionStatusError {
    // message fields
    txn: ::protobuf::SingularPtrField<Transaction>,
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TransactionStatusError {
    pub fn new() -> TransactionStatusError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransactionStatusError {
        static mut instance: ::protobuf::lazy::Lazy<TransactionStatusError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionStatusError,
        };
        unsafe {
            instance.get(|| {
                TransactionStatusError {
                    txn: ::protobuf::SingularPtrField::none(),
                    msg: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Transaction txn = 1;

    pub fn clear_txn(&mut self) {
        self.txn.clear();
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: Transaction) {
        self.txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txn<'a>(&'a mut self) -> &'a mut Transaction {
        if self.txn.is_none() {
            self.txn.set_default();
        };
        self.txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_txn(&mut self) -> Transaction {
        self.txn.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_txn<'a>(&'a self) -> &'a Transaction {
        self.txn.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }

    // optional string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg<'a>(&'a self) -> &'a str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for TransactionStatusError {
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
                    let tmp = self.txn.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.msg.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.txn.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.msg.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txn.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.msg.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<TransactionStatusError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransactionStatusError {
    fn new() -> TransactionStatusError {
        TransactionStatusError::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransactionStatusError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "txn",
                    TransactionStatusError::has_txn,
                    TransactionStatusError::get_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg",
                    TransactionStatusError::has_msg,
                    TransactionStatusError::get_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionStatusError>(
                    "TransactionStatusError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransactionStatusError {
    fn clear(&mut self) {
        self.clear_txn();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TransactionStatusError {
    fn eq(&self, other: &TransactionStatusError) -> bool {
        self.txn == other.txn &&
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TransactionStatusError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WriteIntentError {
    // message fields
    intents: ::protobuf::RepeatedField<WriteIntentError_Intent>,
    resolved: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WriteIntentError {
    pub fn new() -> WriteIntentError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteIntentError {
        static mut instance: ::protobuf::lazy::Lazy<WriteIntentError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteIntentError,
        };
        unsafe {
            instance.get(|| {
                WriteIntentError {
                    intents: ::protobuf::RepeatedField::new(),
                    resolved: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .cockroach.proto.WriteIntentError.Intent intents = 1;

    pub fn clear_intents(&mut self) {
        self.intents.clear();
    }

    // Param is passed by value, moved
    pub fn set_intents(&mut self, v: ::protobuf::RepeatedField<WriteIntentError_Intent>) {
        self.intents = v;
    }

    // Mutable pointer to the field.
    pub fn mut_intents<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<WriteIntentError_Intent> {
        &mut self.intents
    }

    // Take field
    pub fn take_intents(&mut self) -> ::protobuf::RepeatedField<WriteIntentError_Intent> {
        ::std::mem::replace(&mut self.intents, ::protobuf::RepeatedField::new())
    }

    pub fn get_intents<'a>(&'a self) -> &'a [WriteIntentError_Intent] {
        &self.intents
    }

    // optional bool resolved = 2;

    pub fn clear_resolved(&mut self) {
        self.resolved = ::std::option::Option::None;
    }

    pub fn has_resolved(&self) -> bool {
        self.resolved.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolved(&mut self, v: bool) {
        self.resolved = ::std::option::Option::Some(v);
    }

    pub fn get_resolved<'a>(&self) -> bool {
        self.resolved.unwrap_or(false)
    }
}

impl ::protobuf::Message for WriteIntentError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.intents));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.resolved = ::std::option::Option::Some(tmp);
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
        for value in self.intents.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.resolved.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.intents.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.resolved {
            try!(os.write_bool(2, v));
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
        ::std::any::TypeId::of::<WriteIntentError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteIntentError {
    fn new() -> WriteIntentError {
        WriteIntentError::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteIntentError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "intents",
                    WriteIntentError::get_intents,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "resolved",
                    WriteIntentError::has_resolved,
                    WriteIntentError::get_resolved,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteIntentError>(
                    "WriteIntentError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteIntentError {
    fn clear(&mut self) {
        self.clear_intents();
        self.clear_resolved();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WriteIntentError {
    fn eq(&self, other: &WriteIntentError) -> bool {
        self.intents == other.intents &&
        self.resolved == other.resolved &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WriteIntentError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WriteIntentError_Intent {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    txn: ::protobuf::SingularPtrField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WriteIntentError_Intent {
    pub fn new() -> WriteIntentError_Intent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteIntentError_Intent {
        static mut instance: ::protobuf::lazy::Lazy<WriteIntentError_Intent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteIntentError_Intent,
        };
        unsafe {
            instance.get(|| {
                WriteIntentError_Intent {
                    key: ::protobuf::SingularField::none(),
                    txn: ::protobuf::SingularPtrField::none(),
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

    // optional .cockroach.proto.Transaction txn = 2;

    pub fn clear_txn(&mut self) {
        self.txn.clear();
    }

    pub fn has_txn(&self) -> bool {
        self.txn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn(&mut self, v: Transaction) {
        self.txn = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txn<'a>(&'a mut self) -> &'a mut Transaction {
        if self.txn.is_none() {
            self.txn.set_default();
        };
        self.txn.as_mut().unwrap()
    }

    // Take field
    pub fn take_txn(&mut self) -> Transaction {
        self.txn.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_txn<'a>(&'a self) -> &'a Transaction {
        self.txn.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }
}

impl ::protobuf::Message for WriteIntentError_Intent {
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
                    let tmp = self.txn.set_default();
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
        for value in self.txn.iter() {
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
        if let Some(v) = self.txn.as_ref() {
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
        ::std::any::TypeId::of::<WriteIntentError_Intent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteIntentError_Intent {
    fn new() -> WriteIntentError_Intent {
        WriteIntentError_Intent::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteIntentError_Intent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    WriteIntentError_Intent::has_key,
                    WriteIntentError_Intent::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "txn",
                    WriteIntentError_Intent::has_txn,
                    WriteIntentError_Intent::get_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteIntentError_Intent>(
                    "WriteIntentError_Intent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteIntentError_Intent {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WriteIntentError_Intent {
    fn eq(&self, other: &WriteIntentError_Intent) -> bool {
        self.key == other.key &&
        self.txn == other.txn &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WriteIntentError_Intent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WriteTooOldError {
    // message fields
    timestamp: ::protobuf::SingularPtrField<Timestamp>,
    existing_timestamp: ::protobuf::SingularPtrField<Timestamp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WriteTooOldError {
    pub fn new() -> WriteTooOldError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteTooOldError {
        static mut instance: ::protobuf::lazy::Lazy<WriteTooOldError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteTooOldError,
        };
        unsafe {
            instance.get(|| {
                WriteTooOldError {
                    timestamp: ::protobuf::SingularPtrField::none(),
                    existing_timestamp: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Timestamp timestamp = 1;

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

    // optional .cockroach.proto.Timestamp existing_timestamp = 2;

    pub fn clear_existing_timestamp(&mut self) {
        self.existing_timestamp.clear();
    }

    pub fn has_existing_timestamp(&self) -> bool {
        self.existing_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_existing_timestamp(&mut self, v: Timestamp) {
        self.existing_timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_existing_timestamp<'a>(&'a mut self) -> &'a mut Timestamp {
        if self.existing_timestamp.is_none() {
            self.existing_timestamp.set_default();
        };
        self.existing_timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_existing_timestamp(&mut self) -> Timestamp {
        self.existing_timestamp.take().unwrap_or_else(|| Timestamp::new())
    }

    pub fn get_existing_timestamp<'a>(&'a self) -> &'a Timestamp {
        self.existing_timestamp.as_ref().unwrap_or_else(|| Timestamp::default_instance())
    }
}

impl ::protobuf::Message for WriteTooOldError {
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
                    let tmp = self.timestamp.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.existing_timestamp.set_default();
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
        for value in self.timestamp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.existing_timestamp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.existing_timestamp.as_ref() {
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
        ::std::any::TypeId::of::<WriteTooOldError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteTooOldError {
    fn new() -> WriteTooOldError {
        WriteTooOldError::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteTooOldError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "timestamp",
                    WriteTooOldError::has_timestamp,
                    WriteTooOldError::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "existing_timestamp",
                    WriteTooOldError::has_existing_timestamp,
                    WriteTooOldError::get_existing_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteTooOldError>(
                    "WriteTooOldError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteTooOldError {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_existing_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WriteTooOldError {
    fn eq(&self, other: &WriteTooOldError) -> bool {
        self.timestamp == other.timestamp &&
        self.existing_timestamp == other.existing_timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WriteTooOldError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpRequiresTxnError {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpRequiresTxnError {
    pub fn new() -> OpRequiresTxnError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpRequiresTxnError {
        static mut instance: ::protobuf::lazy::Lazy<OpRequiresTxnError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpRequiresTxnError,
        };
        unsafe {
            instance.get(|| {
                OpRequiresTxnError {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for OpRequiresTxnError {
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
        ::std::any::TypeId::of::<OpRequiresTxnError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpRequiresTxnError {
    fn new() -> OpRequiresTxnError {
        OpRequiresTxnError::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpRequiresTxnError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<OpRequiresTxnError>(
                    "OpRequiresTxnError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpRequiresTxnError {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpRequiresTxnError {
    fn eq(&self, other: &OpRequiresTxnError) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpRequiresTxnError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ConditionFailedError {
    // message fields
    actual_value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ConditionFailedError {
    pub fn new() -> ConditionFailedError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConditionFailedError {
        static mut instance: ::protobuf::lazy::Lazy<ConditionFailedError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConditionFailedError,
        };
        unsafe {
            instance.get(|| {
                ConditionFailedError {
                    actual_value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Value actual_value = 1;

    pub fn clear_actual_value(&mut self) {
        self.actual_value.clear();
    }

    pub fn has_actual_value(&self) -> bool {
        self.actual_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_actual_value(&mut self, v: Value) {
        self.actual_value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_actual_value<'a>(&'a mut self) -> &'a mut Value {
        if self.actual_value.is_none() {
            self.actual_value.set_default();
        };
        self.actual_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_actual_value(&mut self) -> Value {
        self.actual_value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_actual_value<'a>(&'a self) -> &'a Value {
        self.actual_value.as_ref().unwrap_or_else(|| Value::default_instance())
    }
}

impl ::protobuf::Message for ConditionFailedError {
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
                    let tmp = self.actual_value.set_default();
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
        for value in self.actual_value.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.actual_value.as_ref() {
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
        ::std::any::TypeId::of::<ConditionFailedError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConditionFailedError {
    fn new() -> ConditionFailedError {
        ConditionFailedError::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConditionFailedError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "actual_value",
                    ConditionFailedError::has_actual_value,
                    ConditionFailedError::get_actual_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConditionFailedError>(
                    "ConditionFailedError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConditionFailedError {
    fn clear(&mut self) {
        self.clear_actual_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ConditionFailedError {
    fn eq(&self, other: &ConditionFailedError) -> bool {
        self.actual_value == other.actual_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ConditionFailedError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LeaseRejectedError {
    // message fields
    Requested: ::protobuf::SingularPtrField<Lease>,
    Existing: ::protobuf::SingularPtrField<Lease>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl LeaseRejectedError {
    pub fn new() -> LeaseRejectedError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeaseRejectedError {
        static mut instance: ::protobuf::lazy::Lazy<LeaseRejectedError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeaseRejectedError,
        };
        unsafe {
            instance.get(|| {
                LeaseRejectedError {
                    Requested: ::protobuf::SingularPtrField::none(),
                    Existing: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Lease Requested = 1;

    pub fn clear_Requested(&mut self) {
        self.Requested.clear();
    }

    pub fn has_Requested(&self) -> bool {
        self.Requested.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Requested(&mut self, v: Lease) {
        self.Requested = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Requested<'a>(&'a mut self) -> &'a mut Lease {
        if self.Requested.is_none() {
            self.Requested.set_default();
        };
        self.Requested.as_mut().unwrap()
    }

    // Take field
    pub fn take_Requested(&mut self) -> Lease {
        self.Requested.take().unwrap_or_else(|| Lease::new())
    }

    pub fn get_Requested<'a>(&'a self) -> &'a Lease {
        self.Requested.as_ref().unwrap_or_else(|| Lease::default_instance())
    }

    // optional .cockroach.proto.Lease Existing = 2;

    pub fn clear_Existing(&mut self) {
        self.Existing.clear();
    }

    pub fn has_Existing(&self) -> bool {
        self.Existing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Existing(&mut self, v: Lease) {
        self.Existing = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Existing<'a>(&'a mut self) -> &'a mut Lease {
        if self.Existing.is_none() {
            self.Existing.set_default();
        };
        self.Existing.as_mut().unwrap()
    }

    // Take field
    pub fn take_Existing(&mut self) -> Lease {
        self.Existing.take().unwrap_or_else(|| Lease::new())
    }

    pub fn get_Existing<'a>(&'a self) -> &'a Lease {
        self.Existing.as_ref().unwrap_or_else(|| Lease::default_instance())
    }
}

impl ::protobuf::Message for LeaseRejectedError {
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
                    let tmp = self.Requested.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.Existing.set_default();
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
        for value in self.Requested.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.Existing.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Requested.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.Existing.as_ref() {
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
        ::std::any::TypeId::of::<LeaseRejectedError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LeaseRejectedError {
    fn new() -> LeaseRejectedError {
        LeaseRejectedError::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeaseRejectedError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "Requested",
                    LeaseRejectedError::has_Requested,
                    LeaseRejectedError::get_Requested,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "Existing",
                    LeaseRejectedError::has_Existing,
                    LeaseRejectedError::get_Existing,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeaseRejectedError>(
                    "LeaseRejectedError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeaseRejectedError {
    fn clear(&mut self) {
        self.clear_Requested();
        self.clear_Existing();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LeaseRejectedError {
    fn eq(&self, other: &LeaseRejectedError) -> bool {
        self.Requested == other.Requested &&
        self.Existing == other.Existing &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LeaseRejectedError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ErrorDetail {
    // message fields
    // message oneof groups
    value: ::std::option::Option<ErrorDetail_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum ErrorDetail_oneof_value {
    not_leader(NotLeaderError),
    range_not_found(RangeNotFoundError),
    range_key_mismatch(RangeKeyMismatchError),
    read_within_uncertainty_interval(ReadWithinUncertaintyIntervalError),
    transaction_aborted(TransactionAbortedError),
    transaction_push(TransactionPushError),
    transaction_retry(TransactionRetryError),
    transaction_status(TransactionStatusError),
    write_intent(WriteIntentError),
    write_too_old(WriteTooOldError),
    op_requires_txn(OpRequiresTxnError),
    condition_failed(ConditionFailedError),
    lease_rejected(LeaseRejectedError),
}

impl ErrorDetail {
    pub fn new() -> ErrorDetail {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ErrorDetail {
        static mut instance: ::protobuf::lazy::Lazy<ErrorDetail> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorDetail,
        };
        unsafe {
            instance.get(|| {
                ErrorDetail {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.NotLeaderError not_leader = 1;

    pub fn clear_not_leader(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_not_leader(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_not_leader(&mut self, v: NotLeaderError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_not_leader<'a>(&'a mut self) -> &'a mut NotLeaderError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(NotLeaderError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_not_leader(&mut self) -> NotLeaderError {
        if self.has_not_leader() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(v)) => v,
                _ => panic!(),
            }
        } else {
            NotLeaderError::new()
        }
    }

    pub fn get_not_leader<'a>(&'a self) -> &'a NotLeaderError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(ref v)) => v,
            _ => NotLeaderError::default_instance(),
        }
    }

    // optional .cockroach.proto.RangeNotFoundError range_not_found = 2;

    pub fn clear_range_not_found(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_range_not_found(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_range_not_found(&mut self, v: RangeNotFoundError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_not_found<'a>(&'a mut self) -> &'a mut RangeNotFoundError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(RangeNotFoundError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_range_not_found(&mut self) -> RangeNotFoundError {
        if self.has_range_not_found() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(v)) => v,
                _ => panic!(),
            }
        } else {
            RangeNotFoundError::new()
        }
    }

    pub fn get_range_not_found<'a>(&'a self) -> &'a RangeNotFoundError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(ref v)) => v,
            _ => RangeNotFoundError::default_instance(),
        }
    }

    // optional .cockroach.proto.RangeKeyMismatchError range_key_mismatch = 3;

    pub fn clear_range_key_mismatch(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_range_key_mismatch(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_range_key_mismatch(&mut self, v: RangeKeyMismatchError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_key_mismatch<'a>(&'a mut self) -> &'a mut RangeKeyMismatchError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(RangeKeyMismatchError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_range_key_mismatch(&mut self) -> RangeKeyMismatchError {
        if self.has_range_key_mismatch() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(v)) => v,
                _ => panic!(),
            }
        } else {
            RangeKeyMismatchError::new()
        }
    }

    pub fn get_range_key_mismatch<'a>(&'a self) -> &'a RangeKeyMismatchError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(ref v)) => v,
            _ => RangeKeyMismatchError::default_instance(),
        }
    }

    // optional .cockroach.proto.ReadWithinUncertaintyIntervalError read_within_uncertainty_interval = 4;

    pub fn clear_read_within_uncertainty_interval(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_read_within_uncertainty_interval(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_read_within_uncertainty_interval(&mut self, v: ReadWithinUncertaintyIntervalError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_read_within_uncertainty_interval<'a>(&'a mut self) -> &'a mut ReadWithinUncertaintyIntervalError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(ReadWithinUncertaintyIntervalError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_read_within_uncertainty_interval(&mut self) -> ReadWithinUncertaintyIntervalError {
        if self.has_read_within_uncertainty_interval() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(v)) => v,
                _ => panic!(),
            }
        } else {
            ReadWithinUncertaintyIntervalError::new()
        }
    }

    pub fn get_read_within_uncertainty_interval<'a>(&'a self) -> &'a ReadWithinUncertaintyIntervalError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(ref v)) => v,
            _ => ReadWithinUncertaintyIntervalError::default_instance(),
        }
    }

    // optional .cockroach.proto.TransactionAbortedError transaction_aborted = 5;

    pub fn clear_transaction_aborted(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_transaction_aborted(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_aborted(&mut self, v: TransactionAbortedError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_aborted<'a>(&'a mut self) -> &'a mut TransactionAbortedError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(TransactionAbortedError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction_aborted(&mut self) -> TransactionAbortedError {
        if self.has_transaction_aborted() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(v)) => v,
                _ => panic!(),
            }
        } else {
            TransactionAbortedError::new()
        }
    }

    pub fn get_transaction_aborted<'a>(&'a self) -> &'a TransactionAbortedError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(ref v)) => v,
            _ => TransactionAbortedError::default_instance(),
        }
    }

    // optional .cockroach.proto.TransactionPushError transaction_push = 6;

    pub fn clear_transaction_push(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_transaction_push(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_push(&mut self, v: TransactionPushError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_push<'a>(&'a mut self) -> &'a mut TransactionPushError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(TransactionPushError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction_push(&mut self) -> TransactionPushError {
        if self.has_transaction_push() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(v)) => v,
                _ => panic!(),
            }
        } else {
            TransactionPushError::new()
        }
    }

    pub fn get_transaction_push<'a>(&'a self) -> &'a TransactionPushError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(ref v)) => v,
            _ => TransactionPushError::default_instance(),
        }
    }

    // optional .cockroach.proto.TransactionRetryError transaction_retry = 7;

    pub fn clear_transaction_retry(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_transaction_retry(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_retry(&mut self, v: TransactionRetryError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_retry<'a>(&'a mut self) -> &'a mut TransactionRetryError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(TransactionRetryError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction_retry(&mut self) -> TransactionRetryError {
        if self.has_transaction_retry() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(v)) => v,
                _ => panic!(),
            }
        } else {
            TransactionRetryError::new()
        }
    }

    pub fn get_transaction_retry<'a>(&'a self) -> &'a TransactionRetryError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(ref v)) => v,
            _ => TransactionRetryError::default_instance(),
        }
    }

    // optional .cockroach.proto.TransactionStatusError transaction_status = 8;

    pub fn clear_transaction_status(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_transaction_status(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transaction_status(&mut self, v: TransactionStatusError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_status<'a>(&'a mut self) -> &'a mut TransactionStatusError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(TransactionStatusError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transaction_status(&mut self) -> TransactionStatusError {
        if self.has_transaction_status() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(v)) => v,
                _ => panic!(),
            }
        } else {
            TransactionStatusError::new()
        }
    }

    pub fn get_transaction_status<'a>(&'a self) -> &'a TransactionStatusError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(ref v)) => v,
            _ => TransactionStatusError::default_instance(),
        }
    }

    // optional .cockroach.proto.WriteIntentError write_intent = 9;

    pub fn clear_write_intent(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_write_intent(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_write_intent(&mut self, v: WriteIntentError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_write_intent<'a>(&'a mut self) -> &'a mut WriteIntentError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(WriteIntentError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_write_intent(&mut self) -> WriteIntentError {
        if self.has_write_intent() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(v)) => v,
                _ => panic!(),
            }
        } else {
            WriteIntentError::new()
        }
    }

    pub fn get_write_intent<'a>(&'a self) -> &'a WriteIntentError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(ref v)) => v,
            _ => WriteIntentError::default_instance(),
        }
    }

    // optional .cockroach.proto.WriteTooOldError write_too_old = 10;

    pub fn clear_write_too_old(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_write_too_old(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_write_too_old(&mut self, v: WriteTooOldError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_write_too_old<'a>(&'a mut self) -> &'a mut WriteTooOldError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(WriteTooOldError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_write_too_old(&mut self) -> WriteTooOldError {
        if self.has_write_too_old() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(v)) => v,
                _ => panic!(),
            }
        } else {
            WriteTooOldError::new()
        }
    }

    pub fn get_write_too_old<'a>(&'a self) -> &'a WriteTooOldError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(ref v)) => v,
            _ => WriteTooOldError::default_instance(),
        }
    }

    // optional .cockroach.proto.OpRequiresTxnError op_requires_txn = 11;

    pub fn clear_op_requires_txn(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_op_requires_txn(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_op_requires_txn(&mut self, v: OpRequiresTxnError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_op_requires_txn<'a>(&'a mut self) -> &'a mut OpRequiresTxnError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(OpRequiresTxnError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_op_requires_txn(&mut self) -> OpRequiresTxnError {
        if self.has_op_requires_txn() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(v)) => v,
                _ => panic!(),
            }
        } else {
            OpRequiresTxnError::new()
        }
    }

    pub fn get_op_requires_txn<'a>(&'a self) -> &'a OpRequiresTxnError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(ref v)) => v,
            _ => OpRequiresTxnError::default_instance(),
        }
    }

    // optional .cockroach.proto.ConditionFailedError condition_failed = 12;

    pub fn clear_condition_failed(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_condition_failed(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_condition_failed(&mut self, v: ConditionFailedError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_condition_failed<'a>(&'a mut self) -> &'a mut ConditionFailedError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(ConditionFailedError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_condition_failed(&mut self) -> ConditionFailedError {
        if self.has_condition_failed() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(v)) => v,
                _ => panic!(),
            }
        } else {
            ConditionFailedError::new()
        }
    }

    pub fn get_condition_failed<'a>(&'a self) -> &'a ConditionFailedError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(ref v)) => v,
            _ => ConditionFailedError::default_instance(),
        }
    }

    // optional .cockroach.proto.LeaseRejectedError lease_rejected = 13;

    pub fn clear_lease_rejected(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_lease_rejected(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_lease_rejected(&mut self, v: LeaseRejectedError) {
        self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lease_rejected<'a>(&'a mut self) -> &'a mut LeaseRejectedError {
        if let ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(LeaseRejectedError::new()));
        }
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_lease_rejected(&mut self) -> LeaseRejectedError {
        if self.has_lease_rejected() {
            match self.value.take() {
                ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(v)) => v,
                _ => panic!(),
            }
        } else {
            LeaseRejectedError::new()
        }
    }

    pub fn get_lease_rejected<'a>(&'a self) -> &'a LeaseRejectedError {
        match self.value {
            ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(ref v)) => v,
            _ => LeaseRejectedError::default_instance(),
        }
    }
}

impl ::protobuf::Message for ErrorDetail {
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
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::not_leader(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::range_not_found(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::range_key_mismatch(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::read_within_uncertainty_interval(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_aborted(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_push(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_retry(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::transaction_status(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::write_intent(try!(is.read_message())));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::write_too_old(try!(is.read_message())));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::op_requires_txn(try!(is.read_message())));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::condition_failed(try!(is.read_message())));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ErrorDetail_oneof_value::lease_rejected(try!(is.read_message())));
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
                &ErrorDetail_oneof_value::not_leader(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::range_not_found(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::range_key_mismatch(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::read_within_uncertainty_interval(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::transaction_aborted(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::transaction_push(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::transaction_retry(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::transaction_status(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::write_intent(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::write_too_old(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::op_requires_txn(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::condition_failed(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ErrorDetail_oneof_value::lease_rejected(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
                &ErrorDetail_oneof_value::not_leader(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::range_not_found(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::range_key_mismatch(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::read_within_uncertainty_interval(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::transaction_aborted(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::transaction_push(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::transaction_retry(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::transaction_status(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::write_intent(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::write_too_old(ref v) => {
                    try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::op_requires_txn(ref v) => {
                    try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::condition_failed(ref v) => {
                    try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ErrorDetail_oneof_value::lease_rejected(ref v) => {
                    try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ErrorDetail>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ErrorDetail {
    fn new() -> ErrorDetail {
        ErrorDetail::new()
    }

    fn descriptor_static(_: ::std::option::Option<ErrorDetail>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "not_leader",
                    ErrorDetail::has_not_leader,
                    ErrorDetail::get_not_leader,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "range_not_found",
                    ErrorDetail::has_range_not_found,
                    ErrorDetail::get_range_not_found,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "range_key_mismatch",
                    ErrorDetail::has_range_key_mismatch,
                    ErrorDetail::get_range_key_mismatch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "read_within_uncertainty_interval",
                    ErrorDetail::has_read_within_uncertainty_interval,
                    ErrorDetail::get_read_within_uncertainty_interval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "transaction_aborted",
                    ErrorDetail::has_transaction_aborted,
                    ErrorDetail::get_transaction_aborted,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "transaction_push",
                    ErrorDetail::has_transaction_push,
                    ErrorDetail::get_transaction_push,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "transaction_retry",
                    ErrorDetail::has_transaction_retry,
                    ErrorDetail::get_transaction_retry,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "transaction_status",
                    ErrorDetail::has_transaction_status,
                    ErrorDetail::get_transaction_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "write_intent",
                    ErrorDetail::has_write_intent,
                    ErrorDetail::get_write_intent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "write_too_old",
                    ErrorDetail::has_write_too_old,
                    ErrorDetail::get_write_too_old,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "op_requires_txn",
                    ErrorDetail::has_op_requires_txn,
                    ErrorDetail::get_op_requires_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "condition_failed",
                    ErrorDetail::has_condition_failed,
                    ErrorDetail::get_condition_failed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "lease_rejected",
                    ErrorDetail::has_lease_rejected,
                    ErrorDetail::get_lease_rejected,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ErrorDetail>(
                    "ErrorDetail",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ErrorDetail {
    fn clear(&mut self) {
        self.clear_not_leader();
        self.clear_range_not_found();
        self.clear_range_key_mismatch();
        self.clear_read_within_uncertainty_interval();
        self.clear_transaction_aborted();
        self.clear_transaction_push();
        self.clear_transaction_retry();
        self.clear_transaction_status();
        self.clear_write_intent();
        self.clear_write_too_old();
        self.clear_op_requires_txn();
        self.clear_condition_failed();
        self.clear_lease_rejected();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ErrorDetail {
    fn eq(&self, other: &ErrorDetail) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ErrorDetail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Error {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    retryable: ::std::option::Option<bool>,
    transaction_restart: ::std::option::Option<TransactionRestart>,
    detail: ::protobuf::SingularPtrField<ErrorDetail>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Error {
    pub fn new() -> Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Error {
        static mut instance: ::protobuf::lazy::Lazy<Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Error,
        };
        unsafe {
            instance.get(|| {
                Error {
                    message: ::protobuf::SingularField::none(),
                    retryable: ::std::option::Option::None,
                    transaction_restart: ::std::option::Option::None,
                    detail: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool retryable = 2;

    pub fn clear_retryable(&mut self) {
        self.retryable = ::std::option::Option::None;
    }

    pub fn has_retryable(&self) -> bool {
        self.retryable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retryable(&mut self, v: bool) {
        self.retryable = ::std::option::Option::Some(v);
    }

    pub fn get_retryable<'a>(&self) -> bool {
        self.retryable.unwrap_or(false)
    }

    // optional .cockroach.proto.TransactionRestart transaction_restart = 4;

    pub fn clear_transaction_restart(&mut self) {
        self.transaction_restart = ::std::option::Option::None;
    }

    pub fn has_transaction_restart(&self) -> bool {
        self.transaction_restart.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction_restart(&mut self, v: TransactionRestart) {
        self.transaction_restart = ::std::option::Option::Some(v);
    }

    pub fn get_transaction_restart<'a>(&self) -> TransactionRestart {
        self.transaction_restart.unwrap_or(TransactionRestart::ABORT)
    }

    // optional .cockroach.proto.ErrorDetail detail = 3;

    pub fn clear_detail(&mut self) {
        self.detail.clear();
    }

    pub fn has_detail(&self) -> bool {
        self.detail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_detail(&mut self, v: ErrorDetail) {
        self.detail = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_detail<'a>(&'a mut self) -> &'a mut ErrorDetail {
        if self.detail.is_none() {
            self.detail.set_default();
        };
        self.detail.as_mut().unwrap()
    }

    // Take field
    pub fn take_detail(&mut self) -> ErrorDetail {
        self.detail.take().unwrap_or_else(|| ErrorDetail::new())
    }

    pub fn get_detail<'a>(&'a self) -> &'a ErrorDetail {
        self.detail.as_ref().unwrap_or_else(|| ErrorDetail::default_instance())
    }
}

impl ::protobuf::Message for Error {
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
                    let tmp = self.message.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.retryable = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.transaction_restart = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.detail.set_default();
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
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.retryable.is_some() {
            my_size += 2;
        };
        for value in self.transaction_restart.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in self.detail.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.retryable {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.transaction_restart {
            try!(os.write_enum(4, v as i32));
        };
        if let Some(v) = self.detail.as_ref() {
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
        ::std::any::TypeId::of::<Error>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Error {
    fn new() -> Error {
        Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    Error::has_message,
                    Error::get_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "retryable",
                    Error::has_retryable,
                    Error::get_retryable,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "transaction_restart",
                    Error::has_transaction_restart,
                    Error::get_transaction_restart,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "detail",
                    Error::has_detail,
                    Error::get_detail,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Error>(
                    "Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        self.clear_message();
        self.clear_retryable();
        self.clear_transaction_restart();
        self.clear_detail();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.message == other.message &&
        self.retryable == other.retryable &&
        self.transaction_restart == other.transaction_restart &&
        self.detail == other.detail &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum TransactionRestart {
    ABORT = 0,
    BACKOFF = 1,
    IMMEDIATE = 2,
}

impl ::protobuf::ProtobufEnum for TransactionRestart {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TransactionRestart> {
        match value {
            0 => ::std::option::Option::Some(TransactionRestart::ABORT),
            1 => ::std::option::Option::Some(TransactionRestart::BACKOFF),
            2 => ::std::option::Option::Some(TransactionRestart::IMMEDIATE),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<TransactionRestart>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TransactionRestart", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TransactionRestart {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1c, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1a, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x64,
    0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x65, 0x0a, 0x0e, 0x4e, 0x6f, 0x74,
    0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x29, 0x0a, 0x07, 0x72,
    0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52,
    0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x12, 0x28, 0x0a, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x22, 0x25, 0x0a, 0x12, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e,
    0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x22, 0x7c, 0x0a, 0x15, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x4b, 0x65, 0x79, 0x4d, 0x69, 0x73, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x12, 0x19, 0x0a, 0x11, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x17, 0x0a, 0x0f, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x2f, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x22, 0x8b, 0x01, 0x0a, 0x22, 0x52, 0x65, 0x61, 0x64, 0x57, 0x69,
    0x74, 0x68, 0x69, 0x6e, 0x55, 0x6e, 0x63, 0x65, 0x72, 0x74, 0x61, 0x69, 0x6e, 0x74, 0x79, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x2d, 0x0a, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x36, 0x0a, 0x12, 0x65,
    0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x22, 0x44, 0x0a, 0x17, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x41, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x29,
    0x0a, 0x03, 0x74, 0x78, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x73, 0x0a, 0x14, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x75, 0x73, 0x68, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x12, 0x29, 0x0a, 0x03, 0x74, 0x78, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x30, 0x0a, 0x0a,
    0x70, 0x75, 0x73, 0x68, 0x65, 0x65, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x42,
    0x0a, 0x15, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x74,
    0x72, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x29, 0x0a, 0x03, 0x74, 0x78, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x22, 0x50, 0x0a, 0x16, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x29, 0x0a, 0x03,
    0x74, 0x78, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x22, 0xa1, 0x01, 0x0a, 0x10, 0x57, 0x72, 0x69, 0x74, 0x65, 0x49, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x39, 0x0a, 0x07, 0x69, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x57, 0x72, 0x69,
    0x74, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x49, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x1a, 0x40, 0x0a, 0x06, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x29, 0x0a,
    0x03, 0x74, 0x78, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x79, 0x0a, 0x10, 0x57, 0x72, 0x69, 0x74,
    0x65, 0x54, 0x6f, 0x6f, 0x4f, 0x6c, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x2d, 0x0a, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x36, 0x0a, 0x12, 0x65,
    0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x22, 0x14, 0x0a, 0x12, 0x4f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65,
    0x73, 0x54, 0x78, 0x6e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x44, 0x0a, 0x14, 0x43, 0x6f, 0x6e,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x12, 0x2c, 0x0a, 0x0c, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x5f, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0x69, 0x0a, 0x12, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x52, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x29, 0x0a, 0x09, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4c, 0x65, 0x61, 0x73, 0x65,
    0x12, 0x28, 0x0a, 0x08, 0x45, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x22, 0x85, 0x07, 0x0a, 0x0b, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x12, 0x35, 0x0a, 0x0a, 0x6e, 0x6f,
    0x74, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x4e, 0x6f, 0x74, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48,
    0x00, 0x12, 0x3e, 0x0a, 0x0f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6e, 0x6f, 0x74, 0x5f, 0x66,
    0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x4e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48,
    0x00, 0x12, 0x44, 0x0a, 0x12, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x6d,
    0x69, 0x73, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x52, 0x61, 0x6e, 0x67, 0x65, 0x4b, 0x65, 0x79, 0x4d, 0x69, 0x73, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x5f, 0x0a, 0x20, 0x72, 0x65, 0x61, 0x64, 0x5f,
    0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x5f, 0x75, 0x6e, 0x63, 0x65, 0x72, 0x74, 0x61, 0x69, 0x6e,
    0x74, 0x79, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x33, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x61, 0x64, 0x57, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x55, 0x6e,
    0x63, 0x65, 0x72, 0x74, 0x61, 0x69, 0x6e, 0x74, 0x79, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x47, 0x0a, 0x13, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x41, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48,
    0x00, 0x12, 0x41, 0x0a, 0x10, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x70, 0x75, 0x73, 0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x75, 0x73, 0x68, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x48, 0x00, 0x12, 0x43, 0x0a, 0x11, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x74, 0x72, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x26, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x74,
    0x72, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x45, 0x0a, 0x12, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00,
    0x12, 0x39, 0x0a, 0x0c, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x57, 0x72, 0x69, 0x74, 0x65, 0x49, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x3a, 0x0a, 0x0d, 0x77,
    0x72, 0x69, 0x74, 0x65, 0x5f, 0x74, 0x6f, 0x6f, 0x5f, 0x6f, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x21, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x57, 0x72, 0x69, 0x74, 0x65, 0x54, 0x6f, 0x6f, 0x4f, 0x6c, 0x64,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x3e, 0x0a, 0x0f, 0x6f, 0x70, 0x5f, 0x72, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x5f, 0x74, 0x78, 0x6e, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x23, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x4f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x54, 0x78, 0x6e,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x41, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x64, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x25, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x61, 0x69,
    0x6c, 0x65, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x12, 0x3d, 0x0a, 0x0e, 0x6c, 0x65,
    0x61, 0x73, 0x65, 0x5f, 0x72, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x0d, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x23, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4c, 0x65, 0x61, 0x73, 0x65, 0x52, 0x65, 0x6a, 0x65, 0x63, 0x74,
    0x65, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x48, 0x00, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x22, 0x9b, 0x01, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x0f, 0x0a, 0x07,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a,
    0x09, 0x72, 0x65, 0x74, 0x72, 0x79, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x40, 0x0a, 0x13, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x23, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x12, 0x2c, 0x0a, 0x06, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c,
    0x2a, 0x3b, 0x0a, 0x12, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x12, 0x09, 0x0a, 0x05, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x10,
    0x00, 0x12, 0x0b, 0x0a, 0x07, 0x42, 0x41, 0x43, 0x4b, 0x4f, 0x46, 0x46, 0x10, 0x01, 0x12, 0x0d,
    0x0a, 0x09, 0x49, 0x4d, 0x4d, 0x45, 0x44, 0x49, 0x41, 0x54, 0x45, 0x10, 0x02, 0x42, 0x07, 0x5a,
    0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x4a, 0xd8, 0x37, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xa6,
    0x01, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x25, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x03, 0x07, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x04,
    0x08, 0x17, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x06, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x06, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12,
    0x03, 0x06, 0x14, 0x1b, 0x0a, 0x67, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x14, 0x01,
    0x1a, 0x5b, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20,
    0x68, 0x6f, 0x77, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x05, 0x17, 0x0a, 0x72, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0d, 0x02, 0x0c, 0x1a, 0x65, 0x20, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x20, 0x28,
    0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x29, 0x20, 0x69, 0x73, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x70,
    0x65, 0x72, 0x6d, 0x61, 0x6e, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x0a, 0x0b, 0x0a, 0x73, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x10, 0x02, 0x0e, 0x1a, 0x66, 0x20, 0x42, 0x41, 0x43, 0x4b, 0x4f, 0x46, 0x46,
    0x20, 0x69, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20,
    0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x70, 0x6f, 0x6e, 0x65, 0x6e,
    0x74, 0x69, 0x61, 0x6c, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x6f, 0x66, 0x66, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x10, 0x0c, 0x0d, 0x0a, 0x67, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x13, 0x02, 0x10, 0x1a, 0x5a, 0x20, 0x49, 0x4d, 0x4d, 0x45, 0x44, 0x49,
    0x41, 0x54, 0x45, 0x20, 0x69, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65,
    0x74, 0x72, 0x69, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c,
    0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x02,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x13, 0x0e, 0x0f, 0x0a,
    0x8e, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x18, 0x00, 0x1b, 0x01, 0x1a, 0x81, 0x01, 0x20,
    0x41, 0x20, 0x4e, 0x6f, 0x74, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x6c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x2c, 0x20, 0x69,
    0x74, 0x73, 0x20, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x19, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x19, 0x24, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x2e,
    0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1a, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x24, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x1a, 0x2d, 0x2e, 0x0a, 0x74, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1f, 0x00,
    0x21, 0x01, 0x1a, 0x68, 0x20, 0x41, 0x20, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x4e, 0x6f, 0x74, 0x46,
    0x6f, 0x75, 0x6e, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61,
    0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61,
    0x6e, 0x64, 0x20, 0x77, 0x61, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x0a, 0x20, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69,
    0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x20, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x1b, 0x1c, 0x0a, 0x90, 0x01,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x25, 0x00, 0x29, 0x01, 0x1a, 0x83, 0x01, 0x20, 0x41, 0x20,
    0x52, 0x61, 0x6e, 0x67, 0x65, 0x4b, 0x65, 0x79, 0x4d, 0x69, 0x73, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x77,
    0x61, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x64, 0x69, 0x64, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b,
    0x65, 0x79, 0x28, 0x73, 0x29, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x26, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x26, 0x11, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x25,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x11, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x27, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x28, 0x02, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x28, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x28, 0x0b, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x2c, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x28, 0x34, 0x35, 0x0a, 0xf3, 0x01, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x2f, 0x00, 0x32, 0x01, 0x1a, 0xe6, 0x01, 0x20, 0x41, 0x20, 0x52, 0x65,
    0x61, 0x64, 0x57, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x55, 0x6e, 0x63, 0x65, 0x72, 0x74, 0x61, 0x69,
    0x6e, 0x74, 0x79, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x61, 0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x61, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x0a, 0x20, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65,
    0x64, 0x20, 0x61, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x20, 0x61, 0x74, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x63, 0x65, 0x72, 0x74, 0x61, 0x69, 0x6e, 0x74, 0x79,
    0x0a, 0x20, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x64, 0x20, 0x61, 0x74, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74,
    0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2b, 0x31, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x30, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x30, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x30, 0x26, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30,
    0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x31, 0x02, 0x3d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x31, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x31, 0x26, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x31, 0x3b, 0x3c, 0x0a, 0x77, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x36,
    0x00, 0x38, 0x01, 0x1a, 0x6b, 0x20, 0x41, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x41, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20,
    0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77,
    0x61, 0x73, 0x0a, 0x20, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x36, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x37, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x37, 0x28, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x2e,
    0x2f, 0x0a, 0xb7, 0x01, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x3d, 0x00, 0x42, 0x01, 0x1a, 0xaa,
    0x01, 0x20, 0x41, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50,
    0x75, 0x73, 0x68, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e, 0x6f,
    0x74, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x69, 0x6e, 0x75, 0x65, 0x20, 0x62, 0x65, 0x63,
    0x61, 0x75, 0x73, 0x65, 0x20, 0x69, 0x74, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65,
    0x72, 0x65, 0x64, 0x20, 0x61, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x0a,
    0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x68,
    0x69, 0x63, 0x68, 0x20, 0x69, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x75, 0x6e, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x75, 0x73, 0x68, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x1c, 0x0a, 0x64, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x40, 0x02, 0x30, 0x1a, 0x57, 0x20, 0x74, 0x78, 0x6e, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62,
    0x65, 0x20, 0x6e, 0x75, 0x6c, 0x6c, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x75, 0x73, 0x68, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x68, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x0a, 0x20, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x40, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x40, 0x28, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x40, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x41,
    0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x41, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x41, 0x0b, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x41, 0x28, 0x32, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x41, 0x35, 0x36, 0x0a, 0xc5, 0x01, 0x0a, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x47, 0x00, 0x49, 0x01, 0x1a, 0xb8, 0x01, 0x20, 0x41, 0x20, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x74, 0x72, 0x79, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x74, 0x72,
    0x69, 0x65, 0x64, 0x2c, 0x20, 0x75, 0x73, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x61, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x74,
    0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65,
    0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x47, 0x08, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x48, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x48, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x48, 0x28, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x48, 0x2e, 0x2f, 0x0a, 0xf3, 0x02, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x51, 0x00, 0x54, 0x01,
    0x1a, 0xe6, 0x02, 0x20, 0x41, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64,
    0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x74,
    0x69, 0x62, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6d, 0x65,
    0x61, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79,
    0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x2e,
    0x20, 0x49, 0x74, 0x20, 0x6d, 0x69, 0x67, 0x68, 0x74, 0x20, 0x61, 0x6c, 0x73, 0x6f, 0x20, 0x62,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x73, 0x65, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x64,
    0x75, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x67, 0x72, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2c, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d, 0x61, 0x79, 0x0a, 0x20, 0x20, 0x6f, 0x6e, 0x6c, 0x79,
    0x20, 0x6d, 0x6f, 0x6e, 0x6f, 0x74, 0x6f, 0x6e, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x69,
    0x6e, 0x63, 0x72, 0x65, 0x61, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01,
    0x12, 0x03, 0x51, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x52,
    0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x52, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x52, 0x0b, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x28, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x01, 0x12, 0x03, 0x53, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x53, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x53, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x53, 0x18, 0x19, 0x0a,
    0xd3, 0x03, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x5e, 0x00, 0x65, 0x01, 0x1a, 0xc6, 0x03, 0x20,
    0x41, 0x20, 0x57, 0x72, 0x69, 0x74, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x77, 0x72,
    0x69, 0x74, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x20, 0x62, 0x65, 0x6c,
    0x6f, 0x6e, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x65,
    0x72, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x6c,
    0x65, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x72, 0x65,
    0x61, 0x64, 0x2f, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x72, 0x69, 0x74,
    0x65, 0x2f, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x6c, 0x69, 0x63, 0x74,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x61, 0x74, 0x20, 0x77, 0x68,
    0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x20,
    0x20, 0x77, 0x61, 0x73, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x61, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78, 0x6e, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x73,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73,
    0x27, 0x0a, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x20, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20,
    0x77, 0x61, 0x73, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x66, 0x75, 0x6c, 0x6c, 0x79,
    0x0a, 0x20, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x2c, 0x20, 0x6d, 0x65, 0x61,
    0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x6d, 0x61, 0x79, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70,
    0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69,
    0x61, 0x74, 0x65, 0x6c, 0x79, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x64, 0x20, 0x69, 0x73, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62,
    0x61, 0x63, 0x6b, 0x20, 0x6f, 0x66, 0x66, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x20, 0x72, 0x65,
    0x74, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x5e, 0x08,
    0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x03, 0x00, 0x12, 0x04, 0x5f, 0x02, 0x62, 0x03, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x03, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x0a, 0x10, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x60, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x60, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x60, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x13, 0x16, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x19, 0x1a, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x61, 0x04, 0x32, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x61, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x61, 0x0d, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x61, 0x2a, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x61, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x63, 0x02, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x63, 0x0b, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63,
    0x34, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x3e, 0x3f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x64, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x64, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x64, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x64, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x64, 0x1b, 0x1c, 0x0a, 0xd2, 0x01, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x6a, 0x00,
    0x6d, 0x01, 0x1a, 0xc5, 0x01, 0x20, 0x41, 0x20, 0x57, 0x72, 0x69, 0x74, 0x65, 0x54, 0x6f, 0x6f,
    0x4f, 0x6c, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20,
    0x65, 0x6e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x61, 0x20, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20,
    0x6e, 0x65, 0x77, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x69, 0x74, 0x73, 0x20, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2c, 0x20, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67,
    0x20, 0x69, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x0a, 0x20, 0x20, 0x68, 0x69, 0x73, 0x74,
    0x6f, 0x72, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x64,
    0x20, 0x61, 0x74, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2b, 0x31, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09,
    0x01, 0x12, 0x03, 0x6a, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03,
    0x6b, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6b, 0x0b, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x26, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x6c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x6c, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c,
    0x26, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6c, 0x3b, 0x3c,
    0x0a, 0xfe, 0x01, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x73, 0x00, 0x74, 0x01, 0x1a, 0xf1, 0x01,
    0x20, 0x41, 0x6e, 0x20, 0x4f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x54, 0x78,
    0x6e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x0a, 0x20,
    0x20, 0x63, 0x61, 0x72, 0x72, 0x69, 0x65, 0x64, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x69, 0x6e, 0x20,
    0x61, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x62, 0x75, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20,
    0x6e, 0x6f, 0x74, 0x2e, 0x0a, 0x20, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70,
    0x6c, 0x65, 0x2c, 0x20, 0x61, 0x20, 0x53, 0x63, 0x61, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68,
    0x20, 0x73, 0x70, 0x61, 0x6e, 0x73, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x20, 0x61, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x73, 0x08, 0x1a, 0x0a, 0xd6, 0x01,
    0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x7a, 0x00, 0x7c, 0x01, 0x1a, 0xc9, 0x01, 0x20, 0x41, 0x20,
    0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x0a, 0x20, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x43, 0x6f,
    0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x66, 0x6f, 0x75, 0x6e,
    0x64, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x20, 0x20, 0x62, 0x65, 0x63, 0x61,
    0x75, 0x73, 0x65, 0x20, 0x69, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x6d, 0x69, 0x73, 0x73, 0x69,
    0x6e, 0x67, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x71,
    0x75, 0x61, 0x6c, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66,
    0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x7a,
    0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x7b, 0x02, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7b, 0x0b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7b, 0x22, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x7b, 0x31, 0x32, 0x0a, 0x95, 0x01, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06,
    0x80, 0x01, 0x00, 0x83, 0x01, 0x01, 0x1a, 0x86, 0x01, 0x20, 0x41, 0x20, 0x4c, 0x65, 0x61, 0x73,
    0x65, 0x52, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69,
    0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x72, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x20, 0x63, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x61, 0x63, 0x71, 0x75, 0x69, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x69,
    0x72, 0x65, 0x64, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x80, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x81, 0x01, 0x02, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x81, 0x01, 0x22, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x81, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04,
    0x82, 0x01, 0x02, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x04, 0x82, 0x01,
    0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x01, 0x22,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x82, 0x01, 0x2d, 0x2e,
    0x0a, 0x4c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0x86, 0x01, 0x00, 0x96, 0x01, 0x01, 0x1a, 0x3e,
    0x20, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x61, 0x76, 0x61, 0x69,
    0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0x86, 0x01, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x0d, 0x08, 0x00, 0x12, 0x06, 0x87, 0x01, 0x03, 0x95, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x08, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x00, 0x12, 0x04, 0x88, 0x01, 0x04, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x88, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x88, 0x01, 0x24, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x88, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0x89,
    0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x04, 0x89, 0x01,
    0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x89, 0x01, 0x28,
    0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x89, 0x01, 0x3a, 0x3b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x42, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x2b, 0x3d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x03, 0x06, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x8b, 0x01, 0x38, 0x58, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03,
    0x12, 0x04, 0x8b, 0x01, 0x5b, 0x5c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04, 0x12, 0x04,
    0x8c, 0x01, 0x04, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x06, 0x12, 0x04, 0x8c,
    0x01, 0x04, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8c, 0x01,
    0x2d, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x43,
    0x44, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x3f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x06, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x2a, 0x3a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x06, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x06, 0x06, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x06, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x2b, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06,
    0x03, 0x12, 0x04, 0x8e, 0x01, 0x3f, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x07, 0x12,
    0x04, 0x8f, 0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x06, 0x12, 0x04,
    0x8f, 0x01, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x01, 0x12, 0x04, 0x8f,
    0x01, 0x2c, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x03, 0x12, 0x04, 0x8f, 0x01,
    0x41, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x08, 0x12, 0x04, 0x90, 0x01, 0x04, 0x37,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x06, 0x12, 0x04, 0x90, 0x01, 0x04, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x01, 0x12, 0x04, 0x90, 0x01, 0x26, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x03, 0x12, 0x04, 0x90, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x09, 0x12, 0x04, 0x91, 0x01, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x09, 0x06, 0x12, 0x04, 0x91, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x09, 0x01, 0x12, 0x04, 0x91, 0x01, 0x26, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x09, 0x03, 0x12, 0x04, 0x91, 0x01, 0x36, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0a,
    0x12, 0x04, 0x92, 0x01, 0x04, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x92, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x01, 0x12, 0x04,
    0x92, 0x01, 0x28, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x92,
    0x01, 0x3a, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0b, 0x12, 0x04, 0x93, 0x01, 0x04,
    0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x06, 0x12, 0x04, 0x93, 0x01, 0x04, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x93, 0x01, 0x2a, 0x3a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x93, 0x01, 0x3d, 0x3f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0c, 0x12, 0x04, 0x94, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x0c, 0x06, 0x12, 0x04, 0x94, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x94, 0x01, 0x28, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x0c, 0x03, 0x12, 0x04, 0x94, 0x01, 0x39, 0x3b, 0x0a, 0x72, 0x0a, 0x02, 0x04, 0x0e, 0x12,
    0x06, 0x9a, 0x01, 0x00, 0xa6, 0x01, 0x01, 0x1a, 0x64, 0x20, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x20,
    0x69, 0x73, 0x20, 0x61, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x72, 0x65, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x6c,
    0x75, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x20, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x72,
    0x65, 0x74, 0x72, 0x79, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x0d, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x00, 0x12, 0x04, 0x9c, 0x01, 0x02, 0x1e, 0x1a, 0x2c, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x68, 0x75, 0x6d, 0x61, 0x6e, 0x2d, 0x72, 0x65,
    0x61, 0x64, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x9c, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x9c, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c,
    0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x01,
    0x1c, 0x1d, 0x0a, 0x94, 0x01, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x02,
    0x1e, 0x1a, 0x85, 0x01, 0x20, 0x49, 0x66, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x0a, 0x20,
    0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62,
    0x65, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65, 0x64, 0x20, 0x28, 0x77, 0x69, 0x74, 0x68, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x9f, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x9f, 0x01, 0x10, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x9f, 0x01, 0x1c, 0x1d, 0x0a, 0x95, 0x01, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04,
    0xa2, 0x01, 0x02, 0x47, 0x1a, 0x86, 0x01, 0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x28, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74,
    0x20, 0x61, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x6f, 0x66, 0x66, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x02, 0x06, 0x12, 0x04, 0xa2, 0x01, 0x0b, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x2f, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x45, 0x46, 0x0a, 0x6a, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x03, 0x12, 0x04, 0xa5, 0x01, 0x02, 0x33, 0x1a, 0x5c, 0x20, 0x49, 0x66, 0x20, 0x61, 0x6e, 0x20,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x20, 0x69, 0x73, 0x20, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61,
    0x6c, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x0a, 0x20, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xa5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa5,
    0x01, 0x0b, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa5, 0x01,
    0x28, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x31,
    0x32,
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
