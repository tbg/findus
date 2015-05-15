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
use super::data::TimeSeriesDatapoint;
use super::data::TimeSeriesData;
use super::data::MVCCStats;
use super::data::ReplicaChangeType;
use super::data::IsolationType;
use super::data::TransactionStatus;
use super::errors::NotLeaderError;
use super::errors::RangeNotFoundError;
use super::errors::RangeKeyMismatchError;
use super::errors::ReadWithinUncertaintyIntervalError;
use super::errors::TransactionAbortedError;
use super::errors::TransactionPushError;
use super::errors::TransactionRetryError;
use super::errors::TransactionStatusError;
use super::errors::WriteIntentError;
use super::errors::WriteTooOldError;
use super::errors::OpRequiresTxnError;
use super::errors::ConditionFailedError;
use super::errors::ErrorDetail;
use super::errors::Error;
use super::errors::TransactionRestart;

#[derive(Clone,Default)]
pub struct ClientCmdID {
    // message fields
    wall_time: ::std::option::Option<i64>,
    random: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ClientCmdID {
    pub fn new() -> ClientCmdID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientCmdID {
        static mut instance: ::protobuf::lazy::Lazy<ClientCmdID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientCmdID,
        };
        unsafe {
            instance.get(|| {
                ClientCmdID {
                    wall_time: ::std::option::Option::None,
                    random: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 wall_time = 1;

    pub fn clear_wall_time(&mut self) {
        self.wall_time = ::std::option::Option::None;
    }

    pub fn has_wall_time(&self) -> bool {
        self.wall_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wall_time(&mut self, v: i64) {
        self.wall_time = ::std::option::Option::Some(v);
    }

    pub fn get_wall_time<'a>(&self) -> i64 {
        self.wall_time.unwrap_or(0)
    }

    // optional int64 random = 2;

    pub fn clear_random(&mut self) {
        self.random = ::std::option::Option::None;
    }

    pub fn has_random(&self) -> bool {
        self.random.is_some()
    }

    // Param is passed by value, moved
    pub fn set_random(&mut self, v: i64) {
        self.random = ::std::option::Option::Some(v);
    }

    pub fn get_random<'a>(&self) -> i64 {
        self.random.unwrap_or(0)
    }
}

impl ::protobuf::Message for ClientCmdID {
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
                    self.wall_time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.random = ::std::option::Option::Some(tmp);
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
        for value in self.wall_time.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.random.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.wall_time {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.random {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<ClientCmdID>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientCmdID {
    fn new() -> ClientCmdID {
        ClientCmdID::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientCmdID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "wall_time",
                    ClientCmdID::has_wall_time,
                    ClientCmdID::get_wall_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "random",
                    ClientCmdID::has_random,
                    ClientCmdID::get_random,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientCmdID>(
                    "ClientCmdID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientCmdID {
    fn clear(&mut self) {
        self.clear_wall_time();
        self.clear_random();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientCmdID {
    fn eq(&self, other: &ClientCmdID) -> bool {
        self.wall_time == other.wall_time &&
        self.random == other.random &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientCmdID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestHeader {
    // message fields
    timestamp: ::protobuf::SingularPtrField<Timestamp>,
    cmd_id: ::protobuf::SingularPtrField<ClientCmdID>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    user: ::protobuf::SingularField<::std::string::String>,
    replica: ::protobuf::SingularPtrField<Replica>,
    raft_id: ::std::option::Option<i64>,
    user_priority: ::std::option::Option<i32>,
    txn: ::protobuf::SingularPtrField<Transaction>,
    read_consistency: ::std::option::Option<ReadConsistencyType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RequestHeader {
    pub fn new() -> RequestHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestHeader {
        static mut instance: ::protobuf::lazy::Lazy<RequestHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestHeader,
        };
        unsafe {
            instance.get(|| {
                RequestHeader {
                    timestamp: ::protobuf::SingularPtrField::none(),
                    cmd_id: ::protobuf::SingularPtrField::none(),
                    key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
                    user: ::protobuf::SingularField::none(),
                    replica: ::protobuf::SingularPtrField::none(),
                    raft_id: ::std::option::Option::None,
                    user_priority: ::std::option::Option::None,
                    txn: ::protobuf::SingularPtrField::none(),
                    read_consistency: ::std::option::Option::None,
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

    // optional .cockroach.proto.ClientCmdID cmd_id = 2;

    pub fn clear_cmd_id(&mut self) {
        self.cmd_id.clear();
    }

    pub fn has_cmd_id(&self) -> bool {
        self.cmd_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_id(&mut self, v: ClientCmdID) {
        self.cmd_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_id<'a>(&'a mut self) -> &'a mut ClientCmdID {
        if self.cmd_id.is_none() {
            self.cmd_id.set_default();
        };
        self.cmd_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_id(&mut self) -> ClientCmdID {
        self.cmd_id.take().unwrap_or_else(|| ClientCmdID::new())
    }

    pub fn get_cmd_id<'a>(&'a self) -> &'a ClientCmdID {
        self.cmd_id.as_ref().unwrap_or_else(|| ClientCmdID::default_instance())
    }

    // optional bytes key = 3;

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

    // optional bytes end_key = 4;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    pub fn has_end_key(&self) -> bool {
        self.end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.end_key.is_none() {
            self.end_key.set_default();
        };
        self.end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end_key<'a>(&'a self) -> &'a [u8] {
        match self.end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string user = 5;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ::std::string::String) {
        self.user = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.user.is_none() {
            self.user.set_default();
        };
        self.user.as_mut().unwrap()
    }

    // Take field
    pub fn take_user(&mut self) -> ::std::string::String {
        self.user.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_user<'a>(&'a self) -> &'a str {
        match self.user.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .cockroach.proto.Replica replica = 6;

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

    // optional int64 raft_id = 7;

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

    // optional int32 user_priority = 8;

    pub fn clear_user_priority(&mut self) {
        self.user_priority = ::std::option::Option::None;
    }

    pub fn has_user_priority(&self) -> bool {
        self.user_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_priority(&mut self, v: i32) {
        self.user_priority = ::std::option::Option::Some(v);
    }

    pub fn get_user_priority<'a>(&self) -> i32 {
        self.user_priority.unwrap_or(1i32)
    }

    // optional .cockroach.proto.Transaction txn = 9;

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

    // optional .cockroach.proto.ReadConsistencyType read_consistency = 10;

    pub fn clear_read_consistency(&mut self) {
        self.read_consistency = ::std::option::Option::None;
    }

    pub fn has_read_consistency(&self) -> bool {
        self.read_consistency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_read_consistency(&mut self, v: ReadConsistencyType) {
        self.read_consistency = ::std::option::Option::Some(v);
    }

    pub fn get_read_consistency<'a>(&self) -> ReadConsistencyType {
        self.read_consistency.unwrap_or(ReadConsistencyType::CONSISTENT)
    }
}

impl ::protobuf::Message for RequestHeader {
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
                    let tmp = self.cmd_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.end_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.user.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.replica.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.raft_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.user_priority = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.txn.set_default();
                    try!(is.merge_message(tmp))
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.read_consistency = ::std::option::Option::Some(tmp);
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
        for value in self.cmd_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.end_key.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.user.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.replica.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.raft_id.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.user_priority.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.txn.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.read_consistency.iter() {
            my_size += ::protobuf::rt::enum_size(10, *value);
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
        if let Some(v) = self.cmd_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.user.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.replica.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.raft_id {
            try!(os.write_int64(7, v));
        };
        if let Some(v) = self.user_priority {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.txn.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.read_consistency {
            try!(os.write_enum(10, v as i32));
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
        ::std::any::TypeId::of::<RequestHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestHeader {
    fn new() -> RequestHeader {
        RequestHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "timestamp",
                    RequestHeader::has_timestamp,
                    RequestHeader::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_id",
                    RequestHeader::has_cmd_id,
                    RequestHeader::get_cmd_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RequestHeader::has_key,
                    RequestHeader::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    RequestHeader::has_end_key,
                    RequestHeader::get_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "user",
                    RequestHeader::has_user,
                    RequestHeader::get_user,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "replica",
                    RequestHeader::has_replica,
                    RequestHeader::get_replica,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "raft_id",
                    RequestHeader::has_raft_id,
                    RequestHeader::get_raft_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "user_priority",
                    RequestHeader::has_user_priority,
                    RequestHeader::get_user_priority,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "txn",
                    RequestHeader::has_txn,
                    RequestHeader::get_txn,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "read_consistency",
                    RequestHeader::has_read_consistency,
                    RequestHeader::get_read_consistency,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestHeader>(
                    "RequestHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestHeader {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_cmd_id();
        self.clear_key();
        self.clear_end_key();
        self.clear_user();
        self.clear_replica();
        self.clear_raft_id();
        self.clear_user_priority();
        self.clear_txn();
        self.clear_read_consistency();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestHeader {
    fn eq(&self, other: &RequestHeader) -> bool {
        self.timestamp == other.timestamp &&
        self.cmd_id == other.cmd_id &&
        self.key == other.key &&
        self.end_key == other.end_key &&
        self.user == other.user &&
        self.replica == other.replica &&
        self.raft_id == other.raft_id &&
        self.user_priority == other.user_priority &&
        self.txn == other.txn &&
        self.read_consistency == other.read_consistency &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResponseHeader {
    // message fields
    error: ::protobuf::SingularPtrField<Error>,
    timestamp: ::protobuf::SingularPtrField<Timestamp>,
    txn: ::protobuf::SingularPtrField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ResponseHeader {
    pub fn new() -> ResponseHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseHeader {
        static mut instance: ::protobuf::lazy::Lazy<ResponseHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseHeader,
        };
        unsafe {
            instance.get(|| {
                ResponseHeader {
                    error: ::protobuf::SingularPtrField::none(),
                    timestamp: ::protobuf::SingularPtrField::none(),
                    txn: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
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

    // optional .cockroach.proto.Transaction txn = 3;

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

impl ::protobuf::Message for ResponseHeader {
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
                    let tmp = self.error.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.timestamp.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
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
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.timestamp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.timestamp.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.txn.as_ref() {
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
        ::std::any::TypeId::of::<ResponseHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseHeader {
    fn new() -> ResponseHeader {
        ResponseHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ResponseHeader::has_error,
                    ResponseHeader::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "timestamp",
                    ResponseHeader::has_timestamp,
                    ResponseHeader::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "txn",
                    ResponseHeader::has_txn,
                    ResponseHeader::get_txn,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseHeader>(
                    "ResponseHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseHeader {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_timestamp();
        self.clear_txn();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseHeader {
    fn eq(&self, other: &ResponseHeader) -> bool {
        self.error == other.error &&
        self.timestamp == other.timestamp &&
        self.txn == other.txn &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResponseHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ContainsRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ContainsRequest {
    pub fn new() -> ContainsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContainsRequest {
        static mut instance: ::protobuf::lazy::Lazy<ContainsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContainsRequest,
        };
        unsafe {
            instance.get(|| {
                ContainsRequest {
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

impl ::protobuf::Message for ContainsRequest {
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
        ::std::any::TypeId::of::<ContainsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ContainsRequest {
    fn new() -> ContainsRequest {
        ContainsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContainsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    ContainsRequest::has_header,
                    ContainsRequest::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContainsRequest>(
                    "ContainsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContainsRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ContainsRequest {
    fn eq(&self, other: &ContainsRequest) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ContainsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ContainsResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    exists: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ContainsResponse {
    pub fn new() -> ContainsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContainsResponse {
        static mut instance: ::protobuf::lazy::Lazy<ContainsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContainsResponse,
        };
        unsafe {
            instance.get(|| {
                ContainsResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    exists: ::std::option::Option::None,
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

    // optional bool exists = 2;

    pub fn clear_exists(&mut self) {
        self.exists = ::std::option::Option::None;
    }

    pub fn has_exists(&self) -> bool {
        self.exists.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exists(&mut self, v: bool) {
        self.exists = ::std::option::Option::Some(v);
    }

    pub fn get_exists<'a>(&self) -> bool {
        self.exists.unwrap_or(false)
    }
}

impl ::protobuf::Message for ContainsResponse {
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
                    let tmp = try!(is.read_bool());
                    self.exists = ::std::option::Option::Some(tmp);
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
        if self.exists.is_some() {
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
        if let Some(v) = self.exists {
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
        ::std::any::TypeId::of::<ContainsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ContainsResponse {
    fn new() -> ContainsResponse {
        ContainsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContainsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    ContainsResponse::has_header,
                    ContainsResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "exists",
                    ContainsResponse::has_exists,
                    ContainsResponse::get_exists,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContainsResponse>(
                    "ContainsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContainsResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_exists();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ContainsResponse {
    fn eq(&self, other: &ContainsResponse) -> bool {
        self.header == other.header &&
        self.exists == other.exists &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ContainsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetRequest {
    pub fn new() -> GetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRequest,
        };
        unsafe {
            instance.get(|| {
                GetRequest {
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

impl ::protobuf::Message for GetRequest {
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
        ::std::any::TypeId::of::<GetRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetRequest {
    fn new() -> GetRequest {
        GetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    GetRequest::has_header,
                    GetRequest::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRequest>(
                    "GetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetRequest {
    fn eq(&self, other: &GetRequest) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetResponse {
    pub fn new() -> GetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetResponse,
        };
        unsafe {
            instance.get(|| {
                GetResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    value: ::protobuf::SingularPtrField::none(),
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

impl ::protobuf::Message for GetResponse {
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
        ::std::any::TypeId::of::<GetResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetResponse {
    fn new() -> GetResponse {
        GetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    GetResponse::has_header,
                    GetResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    GetResponse::has_value,
                    GetResponse::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetResponse>(
                    "GetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetResponse {
    fn eq(&self, other: &GetResponse) -> bool {
        self.header == other.header &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PutRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PutRequest {
    pub fn new() -> PutRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutRequest {
        static mut instance: ::protobuf::lazy::Lazy<PutRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutRequest,
        };
        unsafe {
            instance.get(|| {
                PutRequest {
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

impl ::protobuf::Message for PutRequest {
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
        ::std::any::TypeId::of::<PutRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutRequest {
    fn new() -> PutRequest {
        PutRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    PutRequest::has_header,
                    PutRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    PutRequest::has_value,
                    PutRequest::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutRequest>(
                    "PutRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PutRequest {
    fn eq(&self, other: &PutRequest) -> bool {
        self.header == other.header &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PutRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PutResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PutResponse {
    pub fn new() -> PutResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutResponse {
        static mut instance: ::protobuf::lazy::Lazy<PutResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutResponse,
        };
        unsafe {
            instance.get(|| {
                PutResponse {
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

impl ::protobuf::Message for PutResponse {
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
        ::std::any::TypeId::of::<PutResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutResponse {
    fn new() -> PutResponse {
        PutResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    PutResponse::has_header,
                    PutResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutResponse>(
                    "PutResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PutResponse {
    fn eq(&self, other: &PutResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PutResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ConditionalPutRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    value: ::protobuf::SingularPtrField<Value>,
    exp_value: ::protobuf::SingularPtrField<Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ConditionalPutRequest {
    pub fn new() -> ConditionalPutRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConditionalPutRequest {
        static mut instance: ::protobuf::lazy::Lazy<ConditionalPutRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConditionalPutRequest,
        };
        unsafe {
            instance.get(|| {
                ConditionalPutRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    exp_value: ::protobuf::SingularPtrField::none(),
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

    // optional .cockroach.proto.Value exp_value = 3;

    pub fn clear_exp_value(&mut self) {
        self.exp_value.clear();
    }

    pub fn has_exp_value(&self) -> bool {
        self.exp_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exp_value(&mut self, v: Value) {
        self.exp_value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exp_value<'a>(&'a mut self) -> &'a mut Value {
        if self.exp_value.is_none() {
            self.exp_value.set_default();
        };
        self.exp_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_exp_value(&mut self) -> Value {
        self.exp_value.take().unwrap_or_else(|| Value::new())
    }

    pub fn get_exp_value<'a>(&'a self) -> &'a Value {
        self.exp_value.as_ref().unwrap_or_else(|| Value::default_instance())
    }
}

impl ::protobuf::Message for ConditionalPutRequest {
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
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.exp_value.set_default();
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
        for value in self.exp_value.iter() {
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
        if let Some(v) = self.exp_value.as_ref() {
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
        ::std::any::TypeId::of::<ConditionalPutRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConditionalPutRequest {
    fn new() -> ConditionalPutRequest {
        ConditionalPutRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConditionalPutRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    ConditionalPutRequest::has_header,
                    ConditionalPutRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    ConditionalPutRequest::has_value,
                    ConditionalPutRequest::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "exp_value",
                    ConditionalPutRequest::has_exp_value,
                    ConditionalPutRequest::get_exp_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConditionalPutRequest>(
                    "ConditionalPutRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConditionalPutRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_value();
        self.clear_exp_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ConditionalPutRequest {
    fn eq(&self, other: &ConditionalPutRequest) -> bool {
        self.header == other.header &&
        self.value == other.value &&
        self.exp_value == other.exp_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ConditionalPutRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ConditionalPutResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ConditionalPutResponse {
    pub fn new() -> ConditionalPutResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConditionalPutResponse {
        static mut instance: ::protobuf::lazy::Lazy<ConditionalPutResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConditionalPutResponse,
        };
        unsafe {
            instance.get(|| {
                ConditionalPutResponse {
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

impl ::protobuf::Message for ConditionalPutResponse {
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
        ::std::any::TypeId::of::<ConditionalPutResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConditionalPutResponse {
    fn new() -> ConditionalPutResponse {
        ConditionalPutResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConditionalPutResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    ConditionalPutResponse::has_header,
                    ConditionalPutResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConditionalPutResponse>(
                    "ConditionalPutResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConditionalPutResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ConditionalPutResponse {
    fn eq(&self, other: &ConditionalPutResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ConditionalPutResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IncrementRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    increment: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl IncrementRequest {
    pub fn new() -> IncrementRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IncrementRequest {
        static mut instance: ::protobuf::lazy::Lazy<IncrementRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IncrementRequest,
        };
        unsafe {
            instance.get(|| {
                IncrementRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    increment: ::std::option::Option::None,
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

    // optional int64 increment = 2;

    pub fn clear_increment(&mut self) {
        self.increment = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        self.increment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: i64) {
        self.increment = ::std::option::Option::Some(v);
    }

    pub fn get_increment<'a>(&self) -> i64 {
        self.increment.unwrap_or(0)
    }
}

impl ::protobuf::Message for IncrementRequest {
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
                    let tmp = try!(is.read_int64());
                    self.increment = ::std::option::Option::Some(tmp);
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
        for value in self.increment.iter() {
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
        if let Some(v) = self.increment {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<IncrementRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IncrementRequest {
    fn new() -> IncrementRequest {
        IncrementRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<IncrementRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    IncrementRequest::has_header,
                    IncrementRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "increment",
                    IncrementRequest::has_increment,
                    IncrementRequest::get_increment,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IncrementRequest>(
                    "IncrementRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IncrementRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_increment();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IncrementRequest {
    fn eq(&self, other: &IncrementRequest) -> bool {
        self.header == other.header &&
        self.increment == other.increment &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IncrementRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IncrementResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    new_value: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl IncrementResponse {
    pub fn new() -> IncrementResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IncrementResponse {
        static mut instance: ::protobuf::lazy::Lazy<IncrementResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IncrementResponse,
        };
        unsafe {
            instance.get(|| {
                IncrementResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    new_value: ::std::option::Option::None,
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

    // optional int64 new_value = 2;

    pub fn clear_new_value(&mut self) {
        self.new_value = ::std::option::Option::None;
    }

    pub fn has_new_value(&self) -> bool {
        self.new_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_value(&mut self, v: i64) {
        self.new_value = ::std::option::Option::Some(v);
    }

    pub fn get_new_value<'a>(&self) -> i64 {
        self.new_value.unwrap_or(0)
    }
}

impl ::protobuf::Message for IncrementResponse {
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
                    let tmp = try!(is.read_int64());
                    self.new_value = ::std::option::Option::Some(tmp);
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
        for value in self.new_value.iter() {
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
        if let Some(v) = self.new_value {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<IncrementResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IncrementResponse {
    fn new() -> IncrementResponse {
        IncrementResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<IncrementResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    IncrementResponse::has_header,
                    IncrementResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "new_value",
                    IncrementResponse::has_new_value,
                    IncrementResponse::get_new_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IncrementResponse>(
                    "IncrementResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IncrementResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_new_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IncrementResponse {
    fn eq(&self, other: &IncrementResponse) -> bool {
        self.header == other.header &&
        self.new_value == other.new_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IncrementResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DeleteRequest {
    pub fn new() -> DeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRequest,
        };
        unsafe {
            instance.get(|| {
                DeleteRequest {
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

impl ::protobuf::Message for DeleteRequest {
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
        ::std::any::TypeId::of::<DeleteRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteRequest {
    fn new() -> DeleteRequest {
        DeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    DeleteRequest::has_header,
                    DeleteRequest::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRequest>(
                    "DeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteRequest {
    fn eq(&self, other: &DeleteRequest) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DeleteResponse {
    pub fn new() -> DeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteResponse,
        };
        unsafe {
            instance.get(|| {
                DeleteResponse {
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

impl ::protobuf::Message for DeleteResponse {
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
        ::std::any::TypeId::of::<DeleteResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteResponse {
    fn new() -> DeleteResponse {
        DeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    DeleteResponse::has_header,
                    DeleteResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteResponse>(
                    "DeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteResponse {
    fn eq(&self, other: &DeleteResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteRangeRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    max_entries_to_delete: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DeleteRangeRequest {
    pub fn new() -> DeleteRangeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRangeRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRangeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRangeRequest,
        };
        unsafe {
            instance.get(|| {
                DeleteRangeRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    max_entries_to_delete: ::std::option::Option::None,
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

    // optional int64 max_entries_to_delete = 2;

    pub fn clear_max_entries_to_delete(&mut self) {
        self.max_entries_to_delete = ::std::option::Option::None;
    }

    pub fn has_max_entries_to_delete(&self) -> bool {
        self.max_entries_to_delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_entries_to_delete(&mut self, v: i64) {
        self.max_entries_to_delete = ::std::option::Option::Some(v);
    }

    pub fn get_max_entries_to_delete<'a>(&self) -> i64 {
        self.max_entries_to_delete.unwrap_or(0)
    }
}

impl ::protobuf::Message for DeleteRangeRequest {
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
                    let tmp = try!(is.read_int64());
                    self.max_entries_to_delete = ::std::option::Option::Some(tmp);
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
        for value in self.max_entries_to_delete.iter() {
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
        if let Some(v) = self.max_entries_to_delete {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<DeleteRangeRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteRangeRequest {
    fn new() -> DeleteRangeRequest {
        DeleteRangeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRangeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    DeleteRangeRequest::has_header,
                    DeleteRangeRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "max_entries_to_delete",
                    DeleteRangeRequest::has_max_entries_to_delete,
                    DeleteRangeRequest::get_max_entries_to_delete,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRangeRequest>(
                    "DeleteRangeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRangeRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_max_entries_to_delete();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteRangeRequest {
    fn eq(&self, other: &DeleteRangeRequest) -> bool {
        self.header == other.header &&
        self.max_entries_to_delete == other.max_entries_to_delete &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteRangeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteRangeResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    num_deleted: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DeleteRangeResponse {
    pub fn new() -> DeleteRangeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRangeResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRangeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRangeResponse,
        };
        unsafe {
            instance.get(|| {
                DeleteRangeResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    num_deleted: ::std::option::Option::None,
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

    // optional int64 num_deleted = 2;

    pub fn clear_num_deleted(&mut self) {
        self.num_deleted = ::std::option::Option::None;
    }

    pub fn has_num_deleted(&self) -> bool {
        self.num_deleted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_deleted(&mut self, v: i64) {
        self.num_deleted = ::std::option::Option::Some(v);
    }

    pub fn get_num_deleted<'a>(&self) -> i64 {
        self.num_deleted.unwrap_or(0)
    }
}

impl ::protobuf::Message for DeleteRangeResponse {
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
                    let tmp = try!(is.read_int64());
                    self.num_deleted = ::std::option::Option::Some(tmp);
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
        for value in self.num_deleted.iter() {
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
        if let Some(v) = self.num_deleted {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<DeleteRangeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteRangeResponse {
    fn new() -> DeleteRangeResponse {
        DeleteRangeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRangeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    DeleteRangeResponse::has_header,
                    DeleteRangeResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "num_deleted",
                    DeleteRangeResponse::has_num_deleted,
                    DeleteRangeResponse::get_num_deleted,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRangeResponse>(
                    "DeleteRangeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRangeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_num_deleted();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteRangeResponse {
    fn eq(&self, other: &DeleteRangeResponse) -> bool {
        self.header == other.header &&
        self.num_deleted == other.num_deleted &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteRangeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScanRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    max_results: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ScanRequest {
    pub fn new() -> ScanRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScanRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanRequest,
        };
        unsafe {
            instance.get(|| {
                ScanRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    max_results: ::std::option::Option::None,
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

    // optional int64 max_results = 2;

    pub fn clear_max_results(&mut self) {
        self.max_results = ::std::option::Option::None;
    }

    pub fn has_max_results(&self) -> bool {
        self.max_results.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_results(&mut self, v: i64) {
        self.max_results = ::std::option::Option::Some(v);
    }

    pub fn get_max_results<'a>(&self) -> i64 {
        self.max_results.unwrap_or(0)
    }
}

impl ::protobuf::Message for ScanRequest {
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
                    let tmp = try!(is.read_int64());
                    self.max_results = ::std::option::Option::Some(tmp);
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
        for value in self.max_results.iter() {
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
        if let Some(v) = self.max_results {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<ScanRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScanRequest {
    fn new() -> ScanRequest {
        ScanRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    ScanRequest::has_header,
                    ScanRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "max_results",
                    ScanRequest::has_max_results,
                    ScanRequest::get_max_results,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanRequest>(
                    "ScanRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_max_results();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScanRequest {
    fn eq(&self, other: &ScanRequest) -> bool {
        self.header == other.header &&
        self.max_results == other.max_results &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScanRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScanResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    rows: ::protobuf::RepeatedField<KeyValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ScanResponse {
    pub fn new() -> ScanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanResponse {
        static mut instance: ::protobuf::lazy::Lazy<ScanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanResponse,
        };
        unsafe {
            instance.get(|| {
                ScanResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    rows: ::protobuf::RepeatedField::new(),
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

    // repeated .cockroach.proto.KeyValue rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<KeyValue>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<KeyValue> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<KeyValue> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows<'a>(&'a self) -> &'a [KeyValue] {
        &self.rows
    }
}

impl ::protobuf::Message for ScanResponse {
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
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
        for value in self.rows.iter() {
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
        for v in self.rows.iter() {
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
        ::std::any::TypeId::of::<ScanResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScanResponse {
    fn new() -> ScanResponse {
        ScanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    ScanResponse::has_header,
                    ScanResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    ScanResponse::get_rows,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanResponse>(
                    "ScanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScanResponse {
    fn eq(&self, other: &ScanResponse) -> bool {
        self.header == other.header &&
        self.rows == other.rows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EndTransactionRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    commit: ::std::option::Option<bool>,
    internal_commit_trigger: ::protobuf::SingularPtrField<InternalCommitTrigger>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl EndTransactionRequest {
    pub fn new() -> EndTransactionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndTransactionRequest {
        static mut instance: ::protobuf::lazy::Lazy<EndTransactionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndTransactionRequest,
        };
        unsafe {
            instance.get(|| {
                EndTransactionRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    commit: ::std::option::Option::None,
                    internal_commit_trigger: ::protobuf::SingularPtrField::none(),
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

    // optional bool commit = 2;

    pub fn clear_commit(&mut self) {
        self.commit = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        self.commit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: bool) {
        self.commit = ::std::option::Option::Some(v);
    }

    pub fn get_commit<'a>(&self) -> bool {
        self.commit.unwrap_or(false)
    }

    // optional .cockroach.proto.InternalCommitTrigger internal_commit_trigger = 3;

    pub fn clear_internal_commit_trigger(&mut self) {
        self.internal_commit_trigger.clear();
    }

    pub fn has_internal_commit_trigger(&self) -> bool {
        self.internal_commit_trigger.is_some()
    }

    // Param is passed by value, moved
    pub fn set_internal_commit_trigger(&mut self, v: InternalCommitTrigger) {
        self.internal_commit_trigger = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_internal_commit_trigger<'a>(&'a mut self) -> &'a mut InternalCommitTrigger {
        if self.internal_commit_trigger.is_none() {
            self.internal_commit_trigger.set_default();
        };
        self.internal_commit_trigger.as_mut().unwrap()
    }

    // Take field
    pub fn take_internal_commit_trigger(&mut self) -> InternalCommitTrigger {
        self.internal_commit_trigger.take().unwrap_or_else(|| InternalCommitTrigger::new())
    }

    pub fn get_internal_commit_trigger<'a>(&'a self) -> &'a InternalCommitTrigger {
        self.internal_commit_trigger.as_ref().unwrap_or_else(|| InternalCommitTrigger::default_instance())
    }
}

impl ::protobuf::Message for EndTransactionRequest {
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
                    let tmp = try!(is.read_bool());
                    self.commit = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.internal_commit_trigger.set_default();
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
        if self.commit.is_some() {
            my_size += 2;
        };
        for value in self.internal_commit_trigger.iter() {
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
        if let Some(v) = self.commit {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.internal_commit_trigger.as_ref() {
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
        ::std::any::TypeId::of::<EndTransactionRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EndTransactionRequest {
    fn new() -> EndTransactionRequest {
        EndTransactionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndTransactionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    EndTransactionRequest::has_header,
                    EndTransactionRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "commit",
                    EndTransactionRequest::has_commit,
                    EndTransactionRequest::get_commit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "internal_commit_trigger",
                    EndTransactionRequest::has_internal_commit_trigger,
                    EndTransactionRequest::get_internal_commit_trigger,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EndTransactionRequest>(
                    "EndTransactionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndTransactionRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_commit();
        self.clear_internal_commit_trigger();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EndTransactionRequest {
    fn eq(&self, other: &EndTransactionRequest) -> bool {
        self.header == other.header &&
        self.commit == other.commit &&
        self.internal_commit_trigger == other.internal_commit_trigger &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EndTransactionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EndTransactionResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    commit_wait: ::std::option::Option<i64>,
    resolved: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl EndTransactionResponse {
    pub fn new() -> EndTransactionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndTransactionResponse {
        static mut instance: ::protobuf::lazy::Lazy<EndTransactionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndTransactionResponse,
        };
        unsafe {
            instance.get(|| {
                EndTransactionResponse {
                    header: ::protobuf::SingularPtrField::none(),
                    commit_wait: ::std::option::Option::None,
                    resolved: ::protobuf::RepeatedField::new(),
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

    // optional int64 commit_wait = 2;

    pub fn clear_commit_wait(&mut self) {
        self.commit_wait = ::std::option::Option::None;
    }

    pub fn has_commit_wait(&self) -> bool {
        self.commit_wait.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_wait(&mut self, v: i64) {
        self.commit_wait = ::std::option::Option::Some(v);
    }

    pub fn get_commit_wait<'a>(&self) -> i64 {
        self.commit_wait.unwrap_or(0)
    }

    // repeated bytes resolved = 3;

    pub fn clear_resolved(&mut self) {
        self.resolved.clear();
    }

    // Param is passed by value, moved
    pub fn set_resolved(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.resolved = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resolved<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.resolved
    }

    // Take field
    pub fn take_resolved(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.resolved, ::protobuf::RepeatedField::new())
    }

    pub fn get_resolved<'a>(&'a self) -> &'a [::std::vec::Vec<u8>] {
        &self.resolved
    }
}

impl ::protobuf::Message for EndTransactionResponse {
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
                    let tmp = try!(is.read_int64());
                    self.commit_wait = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.resolved));
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
        for value in self.commit_wait.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.resolved.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
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
        if let Some(v) = self.commit_wait {
            try!(os.write_int64(2, v));
        };
        for v in self.resolved.iter() {
            try!(os.write_bytes(3, &v));
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
        ::std::any::TypeId::of::<EndTransactionResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EndTransactionResponse {
    fn new() -> EndTransactionResponse {
        EndTransactionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndTransactionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    EndTransactionResponse::has_header,
                    EndTransactionResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "commit_wait",
                    EndTransactionResponse::has_commit_wait,
                    EndTransactionResponse::get_commit_wait,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "resolved",
                    EndTransactionResponse::get_resolved,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EndTransactionResponse>(
                    "EndTransactionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndTransactionResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_commit_wait();
        self.clear_resolved();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EndTransactionResponse {
    fn eq(&self, other: &EndTransactionResponse) -> bool {
        self.header == other.header &&
        self.commit_wait == other.commit_wait &&
        self.resolved == other.resolved &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EndTransactionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestUnion {
    // message fields
    // message oneof groups
    value: ::std::option::Option<RequestUnion_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum RequestUnion_oneof_value {
    contains(ContainsRequest),
    get(GetRequest),
    put(PutRequest),
    conditional_put(ConditionalPutRequest),
    increment(IncrementRequest),
    delete(DeleteRequest),
    delete_range(DeleteRangeRequest),
    scan(ScanRequest),
    end_transaction(EndTransactionRequest),
}

impl RequestUnion {
    pub fn new() -> RequestUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestUnion {
        static mut instance: ::protobuf::lazy::Lazy<RequestUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestUnion,
        };
        unsafe {
            instance.get(|| {
                RequestUnion {
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
            ::std::option::Option::Some(RequestUnion_oneof_value::contains(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_contains(&mut self, v: ContainsRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::contains(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contains<'a>(&'a mut self) -> &'a mut ContainsRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::contains(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::contains(ContainsRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::contains(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_contains(&mut self) -> ContainsRequest {
        if self.has_contains() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::contains(v)) => v,
                _ => panic!(),
            }
        } else {
            ContainsRequest::new()
        }
    }

    pub fn get_contains<'a>(&'a self) -> &'a ContainsRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::contains(ref v)) => v,
            _ => ContainsRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.GetRequest get = 2;

    pub fn clear_get(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_get(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::get(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::get(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get<'a>(&'a mut self) -> &'a mut GetRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::get(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::get(GetRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::get(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get(&mut self) -> GetRequest {
        if self.has_get() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::get(v)) => v,
                _ => panic!(),
            }
        } else {
            GetRequest::new()
        }
    }

    pub fn get_get<'a>(&'a self) -> &'a GetRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::get(ref v)) => v,
            _ => GetRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.PutRequest put = 3;

    pub fn clear_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put<'a>(&'a mut self) -> &'a mut PutRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::put(PutRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_put(&mut self) -> PutRequest {
        if self.has_put() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutRequest::new()
        }
    }

    pub fn get_put<'a>(&'a self) -> &'a PutRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::put(ref v)) => v,
            _ => PutRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.ConditionalPutRequest conditional_put = 4;

    pub fn clear_conditional_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_conditional_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_conditional_put(&mut self, v: ConditionalPutRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conditional_put<'a>(&'a mut self) -> &'a mut ConditionalPutRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(ConditionalPutRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_conditional_put(&mut self) -> ConditionalPutRequest {
        if self.has_conditional_put() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(v)) => v,
                _ => panic!(),
            }
        } else {
            ConditionalPutRequest::new()
        }
    }

    pub fn get_conditional_put<'a>(&'a self) -> &'a ConditionalPutRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(ref v)) => v,
            _ => ConditionalPutRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.IncrementRequest increment = 5;

    pub fn clear_increment(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::increment(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: IncrementRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::increment(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_increment<'a>(&'a mut self) -> &'a mut IncrementRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::increment(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::increment(IncrementRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::increment(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_increment(&mut self) -> IncrementRequest {
        if self.has_increment() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::increment(v)) => v,
                _ => panic!(),
            }
        } else {
            IncrementRequest::new()
        }
    }

    pub fn get_increment<'a>(&'a self) -> &'a IncrementRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::increment(ref v)) => v,
            _ => IncrementRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRequest delete = 6;

    pub fn clear_delete(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::delete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::delete(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete<'a>(&'a mut self) -> &'a mut DeleteRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::delete(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::delete(DeleteRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::delete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteRequest {
        if self.has_delete() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::delete(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRequest::new()
        }
    }

    pub fn get_delete<'a>(&'a self) -> &'a DeleteRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::delete(ref v)) => v,
            _ => DeleteRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRangeRequest delete_range = 7;

    pub fn clear_delete_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_range(&mut self, v: DeleteRangeRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_range<'a>(&'a mut self) -> &'a mut DeleteRangeRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(DeleteRangeRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_range(&mut self) -> DeleteRangeRequest {
        if self.has_delete_range() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeRequest::new()
        }
    }

    pub fn get_delete_range<'a>(&'a self) -> &'a DeleteRangeRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(ref v)) => v,
            _ => DeleteRangeRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.ScanRequest scan = 8;

    pub fn clear_scan(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_scan(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::scan(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_scan(&mut self, v: ScanRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::scan(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scan<'a>(&'a mut self) -> &'a mut ScanRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::scan(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::scan(ScanRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::scan(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_scan(&mut self) -> ScanRequest {
        if self.has_scan() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::scan(v)) => v,
                _ => panic!(),
            }
        } else {
            ScanRequest::new()
        }
    }

    pub fn get_scan<'a>(&'a self) -> &'a ScanRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::scan(ref v)) => v,
            _ => ScanRequest::default_instance(),
        }
    }

    // optional .cockroach.proto.EndTransactionRequest end_transaction = 9;

    pub fn clear_end_transaction(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_transaction(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_transaction(&mut self, v: EndTransactionRequest) {
        self.value = ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_transaction<'a>(&'a mut self) -> &'a mut EndTransactionRequest {
        if let ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(EndTransactionRequest::new()));
        }
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_transaction(&mut self) -> EndTransactionRequest {
        if self.has_end_transaction() {
            match self.value.take() {
                ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(v)) => v,
                _ => panic!(),
            }
        } else {
            EndTransactionRequest::new()
        }
    }

    pub fn get_end_transaction<'a>(&'a self) -> &'a EndTransactionRequest {
        match self.value {
            ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(ref v)) => v,
            _ => EndTransactionRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for RequestUnion {
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
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::contains(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::get(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::put(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::conditional_put(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::increment(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::delete(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::delete_range(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::scan(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(RequestUnion_oneof_value::end_transaction(try!(is.read_message())));
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
                &RequestUnion_oneof_value::contains(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::get(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::conditional_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::increment(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::delete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::scan(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestUnion_oneof_value::end_transaction(ref v) => {
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
                &RequestUnion_oneof_value::contains(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::get(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::put(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::conditional_put(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::increment(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::delete(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::delete_range(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::scan(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestUnion_oneof_value::end_transaction(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<RequestUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestUnion {
    fn new() -> RequestUnion {
        RequestUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contains",
                    RequestUnion::has_contains,
                    RequestUnion::get_contains,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get",
                    RequestUnion::has_get,
                    RequestUnion::get_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put",
                    RequestUnion::has_put,
                    RequestUnion::get_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "conditional_put",
                    RequestUnion::has_conditional_put,
                    RequestUnion::get_conditional_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "increment",
                    RequestUnion::has_increment,
                    RequestUnion::get_increment,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete",
                    RequestUnion::has_delete,
                    RequestUnion::get_delete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_range",
                    RequestUnion::has_delete_range,
                    RequestUnion::get_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scan",
                    RequestUnion::has_scan,
                    RequestUnion::get_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end_transaction",
                    RequestUnion::has_end_transaction,
                    RequestUnion::get_end_transaction,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestUnion>(
                    "RequestUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestUnion {
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestUnion {
    fn eq(&self, other: &RequestUnion) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResponseUnion {
    // message fields
    // message oneof groups
    value: ::std::option::Option<ResponseUnion_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

#[derive(Clone,PartialEq)]
pub enum ResponseUnion_oneof_value {
    contains(ContainsResponse),
    get(GetResponse),
    put(PutResponse),
    conditional_put(ConditionalPutResponse),
    increment(IncrementResponse),
    delete(DeleteResponse),
    delete_range(DeleteRangeResponse),
    scan(ScanResponse),
    end_transaction(EndTransactionResponse),
}

impl ResponseUnion {
    pub fn new() -> ResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<ResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseUnion,
        };
        unsafe {
            instance.get(|| {
                ResponseUnion {
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
            ::std::option::Option::Some(ResponseUnion_oneof_value::contains(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_contains(&mut self, v: ContainsResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::contains(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contains<'a>(&'a mut self) -> &'a mut ContainsResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::contains(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::contains(ContainsResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::contains(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_contains(&mut self) -> ContainsResponse {
        if self.has_contains() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::contains(v)) => v,
                _ => panic!(),
            }
        } else {
            ContainsResponse::new()
        }
    }

    pub fn get_contains<'a>(&'a self) -> &'a ContainsResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::contains(ref v)) => v,
            _ => ContainsResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.GetResponse get = 2;

    pub fn clear_get(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_get(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::get(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::get(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get<'a>(&'a mut self) -> &'a mut GetResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::get(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::get(GetResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::get(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get(&mut self) -> GetResponse {
        if self.has_get() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::get(v)) => v,
                _ => panic!(),
            }
        } else {
            GetResponse::new()
        }
    }

    pub fn get_get<'a>(&'a self) -> &'a GetResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::get(ref v)) => v,
            _ => GetResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.PutResponse put = 3;

    pub fn clear_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put<'a>(&'a mut self) -> &'a mut PutResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::put(PutResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_put(&mut self) -> PutResponse {
        if self.has_put() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::put(v)) => v,
                _ => panic!(),
            }
        } else {
            PutResponse::new()
        }
    }

    pub fn get_put<'a>(&'a self) -> &'a PutResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::put(ref v)) => v,
            _ => PutResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.ConditionalPutResponse conditional_put = 4;

    pub fn clear_conditional_put(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_conditional_put(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_conditional_put(&mut self, v: ConditionalPutResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conditional_put<'a>(&'a mut self) -> &'a mut ConditionalPutResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(ConditionalPutResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_conditional_put(&mut self) -> ConditionalPutResponse {
        if self.has_conditional_put() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(v)) => v,
                _ => panic!(),
            }
        } else {
            ConditionalPutResponse::new()
        }
    }

    pub fn get_conditional_put<'a>(&'a self) -> &'a ConditionalPutResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(ref v)) => v,
            _ => ConditionalPutResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.IncrementResponse increment = 5;

    pub fn clear_increment(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_increment(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::increment(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_increment(&mut self, v: IncrementResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::increment(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_increment<'a>(&'a mut self) -> &'a mut IncrementResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::increment(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::increment(IncrementResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::increment(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_increment(&mut self) -> IncrementResponse {
        if self.has_increment() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::increment(v)) => v,
                _ => panic!(),
            }
        } else {
            IncrementResponse::new()
        }
    }

    pub fn get_increment<'a>(&'a self) -> &'a IncrementResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::increment(ref v)) => v,
            _ => IncrementResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteResponse delete = 6;

    pub fn clear_delete(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::delete(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::delete(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete<'a>(&'a mut self) -> &'a mut DeleteResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::delete(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::delete(DeleteResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::delete(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteResponse {
        if self.has_delete() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::delete(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteResponse::new()
        }
    }

    pub fn get_delete<'a>(&'a self) -> &'a DeleteResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::delete(ref v)) => v,
            _ => DeleteResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.DeleteRangeResponse delete_range = 7;

    pub fn clear_delete_range(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_delete_range(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_delete_range(&mut self, v: DeleteRangeResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_range<'a>(&'a mut self) -> &'a mut DeleteRangeResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(DeleteRangeResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_delete_range(&mut self) -> DeleteRangeResponse {
        if self.has_delete_range() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(v)) => v,
                _ => panic!(),
            }
        } else {
            DeleteRangeResponse::new()
        }
    }

    pub fn get_delete_range<'a>(&'a self) -> &'a DeleteRangeResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(ref v)) => v,
            _ => DeleteRangeResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.ScanResponse scan = 8;

    pub fn clear_scan(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_scan(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::scan(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_scan(&mut self, v: ScanResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::scan(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scan<'a>(&'a mut self) -> &'a mut ScanResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::scan(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::scan(ScanResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::scan(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_scan(&mut self) -> ScanResponse {
        if self.has_scan() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::scan(v)) => v,
                _ => panic!(),
            }
        } else {
            ScanResponse::new()
        }
    }

    pub fn get_scan<'a>(&'a self) -> &'a ScanResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::scan(ref v)) => v,
            _ => ScanResponse::default_instance(),
        }
    }

    // optional .cockroach.proto.EndTransactionResponse end_transaction = 9;

    pub fn clear_end_transaction(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_transaction(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_transaction(&mut self, v: EndTransactionResponse) {
        self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_transaction<'a>(&'a mut self) -> &'a mut EndTransactionResponse {
        if let ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(EndTransactionResponse::new()));
        }
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_transaction(&mut self) -> EndTransactionResponse {
        if self.has_end_transaction() {
            match self.value.take() {
                ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(v)) => v,
                _ => panic!(),
            }
        } else {
            EndTransactionResponse::new()
        }
    }

    pub fn get_end_transaction<'a>(&'a self) -> &'a EndTransactionResponse {
        match self.value {
            ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(ref v)) => v,
            _ => EndTransactionResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for ResponseUnion {
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
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::contains(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::get(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::put(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::conditional_put(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::increment(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::delete(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::delete_range(try!(is.read_message())));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::scan(try!(is.read_message())));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    self.value = ::std::option::Option::Some(ResponseUnion_oneof_value::end_transaction(try!(is.read_message())));
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
                &ResponseUnion_oneof_value::contains(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::get(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::conditional_put(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::increment(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::delete(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::delete_range(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::scan(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseUnion_oneof_value::end_transaction(ref v) => {
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
                &ResponseUnion_oneof_value::contains(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::get(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::put(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::conditional_put(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::increment(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::delete(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::delete_range(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::scan(ref v) => {
                    try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseUnion_oneof_value::end_transaction(ref v) => {
                    try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ResponseUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseUnion {
    fn new() -> ResponseUnion {
        ResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "contains",
                    ResponseUnion::has_contains,
                    ResponseUnion::get_contains,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get",
                    ResponseUnion::has_get,
                    ResponseUnion::get_get,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put",
                    ResponseUnion::has_put,
                    ResponseUnion::get_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "conditional_put",
                    ResponseUnion::has_conditional_put,
                    ResponseUnion::get_conditional_put,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "increment",
                    ResponseUnion::has_increment,
                    ResponseUnion::get_increment,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete",
                    ResponseUnion::has_delete,
                    ResponseUnion::get_delete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_range",
                    ResponseUnion::has_delete_range,
                    ResponseUnion::get_delete_range,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scan",
                    ResponseUnion::has_scan,
                    ResponseUnion::get_scan,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "end_transaction",
                    ResponseUnion::has_end_transaction,
                    ResponseUnion::get_end_transaction,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseUnion>(
                    "ResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseUnion {
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseUnion {
    fn eq(&self, other: &ResponseUnion) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BatchRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    requests: ::protobuf::RepeatedField<RequestUnion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BatchRequest {
    pub fn new() -> BatchRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchRequest {
        static mut instance: ::protobuf::lazy::Lazy<BatchRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchRequest,
        };
        unsafe {
            instance.get(|| {
                BatchRequest {
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

    // repeated .cockroach.proto.RequestUnion requests = 2;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<RequestUnion>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<RequestUnion> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<RequestUnion> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests<'a>(&'a self) -> &'a [RequestUnion] {
        &self.requests
    }
}

impl ::protobuf::Message for BatchRequest {
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
        ::std::any::TypeId::of::<BatchRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BatchRequest {
    fn new() -> BatchRequest {
        BatchRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    BatchRequest::has_header,
                    BatchRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "requests",
                    BatchRequest::get_requests,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchRequest>(
                    "BatchRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_requests();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BatchRequest {
    fn eq(&self, other: &BatchRequest) -> bool {
        self.header == other.header &&
        self.requests == other.requests &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BatchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BatchResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    responses: ::protobuf::RepeatedField<ResponseUnion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BatchResponse {
    pub fn new() -> BatchResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchResponse {
        static mut instance: ::protobuf::lazy::Lazy<BatchResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchResponse,
        };
        unsafe {
            instance.get(|| {
                BatchResponse {
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

    // repeated .cockroach.proto.ResponseUnion responses = 2;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<ResponseUnion>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ResponseUnion> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<ResponseUnion> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses<'a>(&'a self) -> &'a [ResponseUnion] {
        &self.responses
    }
}

impl ::protobuf::Message for BatchResponse {
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
        ::std::any::TypeId::of::<BatchResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BatchResponse {
    fn new() -> BatchResponse {
        BatchResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    BatchResponse::has_header,
                    BatchResponse::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "responses",
                    BatchResponse::get_responses,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchResponse>(
                    "BatchResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BatchResponse {
    fn eq(&self, other: &BatchResponse) -> bool {
        self.header == other.header &&
        self.responses == other.responses &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BatchResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AdminSplitRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    split_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AdminSplitRequest {
    pub fn new() -> AdminSplitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminSplitRequest {
        static mut instance: ::protobuf::lazy::Lazy<AdminSplitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminSplitRequest,
        };
        unsafe {
            instance.get(|| {
                AdminSplitRequest {
                    header: ::protobuf::SingularPtrField::none(),
                    split_key: ::protobuf::SingularField::none(),
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

    // optional bytes split_key = 2;

    pub fn clear_split_key(&mut self) {
        self.split_key.clear();
    }

    pub fn has_split_key(&self) -> bool {
        self.split_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_split_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.split_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_split_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.split_key.is_none() {
            self.split_key.set_default();
        };
        self.split_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_split_key(&mut self) -> ::std::vec::Vec<u8> {
        self.split_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_split_key<'a>(&'a self) -> &'a [u8] {
        match self.split_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for AdminSplitRequest {
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
                    let tmp = self.split_key.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.split_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
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
        if let Some(v) = self.split_key.as_ref() {
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
        ::std::any::TypeId::of::<AdminSplitRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminSplitRequest {
    fn new() -> AdminSplitRequest {
        AdminSplitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminSplitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    AdminSplitRequest::has_header,
                    AdminSplitRequest::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "split_key",
                    AdminSplitRequest::has_split_key,
                    AdminSplitRequest::get_split_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminSplitRequest>(
                    "AdminSplitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminSplitRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_split_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AdminSplitRequest {
    fn eq(&self, other: &AdminSplitRequest) -> bool {
        self.header == other.header &&
        self.split_key == other.split_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AdminSplitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AdminSplitResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AdminSplitResponse {
    pub fn new() -> AdminSplitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminSplitResponse {
        static mut instance: ::protobuf::lazy::Lazy<AdminSplitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminSplitResponse,
        };
        unsafe {
            instance.get(|| {
                AdminSplitResponse {
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

impl ::protobuf::Message for AdminSplitResponse {
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
        ::std::any::TypeId::of::<AdminSplitResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminSplitResponse {
    fn new() -> AdminSplitResponse {
        AdminSplitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminSplitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    AdminSplitResponse::has_header,
                    AdminSplitResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminSplitResponse>(
                    "AdminSplitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminSplitResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AdminSplitResponse {
    fn eq(&self, other: &AdminSplitResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AdminSplitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AdminMergeRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AdminMergeRequest {
    pub fn new() -> AdminMergeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminMergeRequest {
        static mut instance: ::protobuf::lazy::Lazy<AdminMergeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminMergeRequest,
        };
        unsafe {
            instance.get(|| {
                AdminMergeRequest {
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

impl ::protobuf::Message for AdminMergeRequest {
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
        ::std::any::TypeId::of::<AdminMergeRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminMergeRequest {
    fn new() -> AdminMergeRequest {
        AdminMergeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminMergeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    AdminMergeRequest::has_header,
                    AdminMergeRequest::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminMergeRequest>(
                    "AdminMergeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminMergeRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AdminMergeRequest {
    fn eq(&self, other: &AdminMergeRequest) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AdminMergeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AdminMergeResponse {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AdminMergeResponse {
    pub fn new() -> AdminMergeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminMergeResponse {
        static mut instance: ::protobuf::lazy::Lazy<AdminMergeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminMergeResponse,
        };
        unsafe {
            instance.get(|| {
                AdminMergeResponse {
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

impl ::protobuf::Message for AdminMergeResponse {
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
        ::std::any::TypeId::of::<AdminMergeResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AdminMergeResponse {
    fn new() -> AdminMergeResponse {
        AdminMergeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminMergeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    AdminMergeResponse::has_header,
                    AdminMergeResponse::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminMergeResponse>(
                    "AdminMergeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminMergeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AdminMergeResponse {
    fn eq(&self, other: &AdminMergeResponse) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AdminMergeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum ReadConsistencyType {
    CONSISTENT = 0,
    CONSENSUS = 1,
    INCONSISTENT = 2,
}

impl ::protobuf::ProtobufEnum for ReadConsistencyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReadConsistencyType> {
        match value {
            0 => ::std::option::Option::Some(ReadConsistencyType::CONSISTENT),
            1 => ::std::option::Option::Some(ReadConsistencyType::CONSENSUS),
            2 => ::std::option::Option::Some(ReadConsistencyType::INCONSISTENT),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ReadConsistencyType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReadConsistencyType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReadConsistencyType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x61, 0x70, 0x69, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1a, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x64, 0x61, 0x74, 0x61,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x30, 0x0a, 0x0b, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6d,
    0x64, 0x49, 0x44, 0x12, 0x11, 0x0a, 0x09, 0x77, 0x61, 0x6c, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x61, 0x6e, 0x64, 0x6f, 0x6d,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x22, 0xd9, 0x02, 0x0a, 0x0d, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2d, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x2c, 0x0a, 0x06, 0x63, 0x6d, 0x64, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x43, 0x6d, 0x64, 0x49, 0x44, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0c, 0x12, 0x0c, 0x0a, 0x04, 0x75, 0x73, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x29, 0x0a, 0x07, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x18, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x12, 0x0f, 0x0a, 0x07,
    0x72, 0x61, 0x66, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x03, 0x12, 0x18, 0x0a,
    0x0d, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x31, 0x12, 0x29, 0x0a, 0x03, 0x74, 0x78, 0x6e, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x3e, 0x0a, 0x10, 0x72, 0x65, 0x61, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x73, 0x69,
    0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x24, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52,
    0x65, 0x61, 0x64, 0x43, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x54, 0x79,
    0x70, 0x65, 0x22, 0x91, 0x01, 0x0a, 0x0e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x2d, 0x0a, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x29, 0x0a, 0x03, 0x74,
    0x78, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x41, 0x0a, 0x0f, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x53, 0x0a, 0x10, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a,
    0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x0e,
    0x0a, 0x06, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x22, 0x3c,
    0x0a, 0x0a, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06,
    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x65, 0x0a, 0x0b,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x22, 0x63, 0x0a, 0x0a, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x12, 0x25, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x16, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x3e, 0x0a, 0x0b, 0x50, 0x75, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x99, 0x01, 0x0a, 0x15, 0x43, 0x6f, 0x6e,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x25, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x16, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x29, 0x0a, 0x09, 0x65, 0x78, 0x70,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x22, 0x49, 0x0a, 0x16, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f,
    0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22,
    0x55, 0x0a, 0x10, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x11, 0x0a, 0x09, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x22, 0x57, 0x0a, 0x11, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x11, 0x0a, 0x09,
    0x6e, 0x65, 0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x22,
    0x3f, 0x0a, 0x0d, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x22, 0x41, 0x0a, 0x0e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x22, 0x63, 0x0a, 0x12, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x15, 0x6d, 0x61, 0x78,
    0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x22, 0x5b, 0x0a, 0x13, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x03, 0x22, 0x52, 0x0a, 0x0b, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x22, 0x68, 0x0a, 0x0c, 0x53, 0x63, 0x61,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x27, 0x0a, 0x04, 0x72, 0x6f,
    0x77, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4b, 0x65, 0x79, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x22, 0xa0, 0x01, 0x0a, 0x15, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a,
    0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x0e, 0x0a,
    0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x12, 0x47, 0x0a,
    0x17, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x5f, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54,
    0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x22, 0x70, 0x0a, 0x16, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x12, 0x13, 0x0a, 0x0b, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x77, 0x61, 0x69, 0x74,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x80, 0x04, 0x0a, 0x0c, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x34, 0x0a, 0x08, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12,
    0x2a, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x2a, 0x0a, 0x03, 0x70,
    0x75, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x50, 0x75, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x41, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x64, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x70, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x26, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12, 0x36, 0x0a, 0x09, 0x69, 0x6e,
    0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x48, 0x00, 0x12, 0x30, 0x0a, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x00, 0x12, 0x3b, 0x0a, 0x0c, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c,
    0x65, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48,
    0x00, 0x12, 0x2c, 0x0a, 0x04, 0x73, 0x63, 0x61, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x12,
    0x41, 0x0a, 0x0f, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x45, 0x6e, 0x64, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x48, 0x00, 0x42, 0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x8a, 0x04, 0x0a, 0x0d,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x35, 0x0a,
    0x08, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x21, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x48, 0x00, 0x12, 0x2b, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48,
    0x00, 0x12, 0x2b, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x42,
    0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x70, 0x75,
    0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x48, 0x00, 0x12, 0x37, 0x0a, 0x09, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63,
    0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x31, 0x0a, 0x06, 0x64,
    0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f,
    0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x3c,
    0x0a, 0x0c, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x2d, 0x0a, 0x04,
    0x73, 0x63, 0x61, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x63, 0x61,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x12, 0x42, 0x0a, 0x0f, 0x65,
    0x6e, 0x64, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x42,
    0x07, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x6f, 0x0a, 0x0c, 0x42, 0x61, 0x74, 0x63,
    0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2f, 0x0a, 0x08, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x22, 0x73, 0x0a, 0x0d, 0x42, 0x61, 0x74,
    0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x31, 0x0a, 0x09, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x22, 0x56,
    0x0a, 0x11, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b, 0x65, 0x79,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x45, 0x0a, 0x12, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53,
    0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06,
    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63,
    0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x43, 0x0a,
    0x11, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x2e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x22, 0x45, 0x0a, 0x12, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65, 0x72, 0x67, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72,
    0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2a, 0x46, 0x0a, 0x13, 0x52, 0x65, 0x61,
    0x64, 0x43, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x4f, 0x4e, 0x53, 0x49, 0x53, 0x54, 0x45, 0x4e, 0x54, 0x10, 0x00,
    0x12, 0x0d, 0x0a, 0x09, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x53, 0x55, 0x53, 0x10, 0x01, 0x12,
    0x10, 0x0a, 0x0c, 0x49, 0x4e, 0x43, 0x4f, 0x4e, 0x53, 0x49, 0x53, 0x54, 0x45, 0x4e, 0x54, 0x10,
    0x02, 0x42, 0x07, 0x5a, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x4a, 0xc8, 0x80, 0x01, 0x0a, 0x07,
    0x12, 0x05, 0x00, 0x00, 0xe2, 0x02, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02,
    0x07, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x03, 0x07, 0x23, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x04, 0x07, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x05,
    0x08, 0x17, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x07, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x07, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x07, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12,
    0x03, 0x07, 0x14, 0x1b, 0x0a, 0x6a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x17, 0x01,
    0x1a, 0x5e, 0x20, 0x52, 0x65, 0x61, 0x64, 0x43, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e,
    0x63, 0x79, 0x54, 0x79, 0x70, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73,
    0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f,
    0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x62, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x64, 0x75, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x72,
    0x65, 0x61, 0x64, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x05, 0x18, 0x0a, 0x87, 0x01, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x11, 0x1a, 0x7a, 0x20, 0x43, 0x4f, 0x4e,
    0x53, 0x49, 0x53, 0x54, 0x45, 0x4e, 0x54, 0x20, 0x72, 0x65, 0x61, 0x64, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x72, 0x65, 0x61, 0x64, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x3b, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x63, 0x68, 0x61,
    0x6e, 0x69, 0x73, 0x6d, 0x20, 0x72, 0x65, 0x6c, 0x69, 0x65, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x63,
    0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69,
    0x6e, 0x65, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x20, 0x65, 0x78, 0x70, 0x69, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0e, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e,
    0x0f, 0x10, 0x0a, 0xac, 0x01, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x10,
    0x1a, 0x9e, 0x01, 0x20, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x53, 0x55, 0x53, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x72, 0x65, 0x61, 0x64,
    0x73, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x61, 0x63, 0x68, 0x69, 0x65, 0x76, 0x65, 0x20, 0x63,
    0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x73, 0x74, 0x72, 0x6f, 0x6e, 0x67, 0x65, 0x72, 0x20, 0x67,
    0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x73,
    0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x43, 0x4f, 0x4e,
    0x53, 0x49, 0x53, 0x54, 0x45, 0x4e, 0x54, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28,
    0x73, 0x70, 0x65, 0x6e, 0x63, 0x65, 0x72, 0x29, 0x3a, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x75, 0x6e, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x02, 0x0b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x12, 0x0e, 0x0f, 0x0a, 0xa8, 0x01,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x16, 0x02, 0x13, 0x1a, 0x9a, 0x01, 0x20, 0x49,
    0x4e, 0x43, 0x4f, 0x4e, 0x53, 0x49, 0x53, 0x54, 0x45, 0x4e, 0x54, 0x20, 0x72, 0x65, 0x61, 0x64,
    0x73, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74,
    0x65, 0x73, 0x74, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x2c, 0x20, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e,
    0x0a, 0x20, 0x20, 0x54, 0x68, 0x65, 0x79, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65,
    0x20, 0x65, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20,
    0x6d, 0x61, 0x79, 0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x73, 0x74, 0x61, 0x6c, 0x65, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x73, 0x20, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x0a, 0x20, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x69,
    0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x16, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x16, 0x11, 0x12, 0x0a, 0xf0, 0x06, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x27, 0x00, 0x2b,
    0x01, 0x1a, 0xe3, 0x06, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6d, 0x64, 0x49, 0x44,
    0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x73, 0x20, 0x61, 0x20, 0x75, 0x6e, 0x69, 0x71,
    0x75, 0x65, 0x20, 0x49, 0x44, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x2e, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6d, 0x64, 0x49, 0x44, 0x20, 0x67,
    0x61, 0x69, 0x6e, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x64,
    0x65, 0x6d, 0x70, 0x6f, 0x74, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x6f, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x77, 0x6f, 0x72, 0x64, 0x73, 0x2c, 0x0a, 0x20, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x73, 0x75, 0x62, 0x6d, 0x69, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x63,
    0x65, 0x69, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x72, 0x69, 0x65,
    0x73, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x66, 0x6c, 0x61, 0x6b, 0x79, 0x0a, 0x20, 0x20, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x2e, 0x20, 0x48, 0x6f, 0x77, 0x65, 0x76, 0x65, 0x72,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x20, 0x69, 0x6d, 0x70,
    0x6f, 0x73, 0x65, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20, 0x6f, 0x6e, 0x20,
    0x68, 0x6f, 0x77, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x0a, 0x20, 0x20, 0x69, 0x64, 0x65, 0x6d, 0x70,
    0x6f, 0x74, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64,
    0x65, 0x64, 0x2e, 0x20, 0x52, 0x65, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x6f, 0x76, 0x65, 0x72,
    0x20, 0x61, 0x6e, 0x20, 0x68, 0x6f, 0x75, 0x72, 0x20, 0x6f, 0x6c, 0x64, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x6e, 0x6f, 0x74, 0x0a, 0x20, 0x20, 0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65,
    0x64, 0x20, 0x69, 0x64, 0x65, 0x6d, 0x70, 0x6f, 0x74, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65,
    0x64, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x63, 0x65,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x20, 0x70, 0x6f, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x61,
    0x6c, 0x6c, 0x79, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43,
    0x6d, 0x64, 0x49, 0x44, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x27, 0x73, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x2d, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x72, 0x61,
    0x6e, 0x64, 0x6f, 0x6d, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x20, 0x69, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x20, 0x75, 0x6e, 0x69, 0x78, 0x0a, 0x20, 0x20, 0x6e, 0x61, 0x6e, 0x6f, 0x73, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65,
    0x6e, 0x65, 0x73, 0x73, 0x20, 0x62, 0x75, 0x74, 0x20, 0x61, 0x6c, 0x73, 0x6f, 0x20, 0x74, 0x6f,
    0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x72, 0x6f, 0x75,
    0x67, 0x68, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x66, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2c, 0x20, 0x75, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x69, 0x74,
    0x79, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x52, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x20, 0x69, 0x73,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61,
    0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65,
    0x6e, 0x65, 0x73, 0x73, 0x2e, 0x0a, 0x20, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x41, 0x6e,
    0x20, 0x61, 0x63, 0x63, 0x75, 0x72, 0x61, 0x74, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x73,
    0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x49, 0x53, 0x20, 0x4e, 0x4f, 0x54, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x63,
    0x74, 0x6e, 0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x27, 0x08, 0x13, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x1f,
    0x1a, 0x1f, 0x20, 0x4e, 0x61, 0x6e, 0x6f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x73,
    0x69, 0x6e, 0x63, 0x65, 0x20, 0x55, 0x6e, 0x69, 0x78, 0x20, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x2a, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x1a, 0x1b, 0x0a, 0x48,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2e, 0x00, 0x59, 0x01, 0x1a, 0x3c, 0x20, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x73, 0x75,
    0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x65, 0x76, 0x65, 0x72,
    0x79, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x2e, 0x08, 0x15, 0x0a, 0xc1, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x32,
    0x02, 0x34, 0x1a, 0xb3, 0x01, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x61,
    0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x6f, 0x72, 0x20,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65,
    0x0a, 0x20, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x69,
    0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x7a, 0x65, 0x72, 0x6f, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x2c, 0x20, 0x69, 0x74, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x0a, 0x20,
    0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x69, 0x6d, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x69, 0x6e,
    0x67, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x32, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32,
    0x26, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x32, 0x33,
    0x0a, 0x60, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x35, 0x02, 0x33, 0x1a, 0x53, 0x20,
    0x43, 0x6d, 0x64, 0x49, 0x44, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61,
    0x6c, 0x6c, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x64, 0x65, 0x6d, 0x70, 0x6f,
    0x74, 0x65, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x20, 0x28, 0x69, 0x2e, 0x65, 0x2e, 0x20, 0x72, 0x65,
    0x70, 0x6c, 0x61, 0x79, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x29,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x35, 0x0b, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x28, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x35, 0x31, 0x32, 0x0a, 0x79, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x38, 0x02, 0x19, 0x1a, 0x6c, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6b, 0x65,
    0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x20, 0x49,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6f, 0x70,
    0x65, 0x72, 0x61, 0x74, 0x65, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e,
    0x67, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x38, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x38, 0x11, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x38, 0x17, 0x18, 0x0a, 0x43, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x02, 0x1d, 0x1a, 0x36, 0x20, 0x45, 0x6e, 0x64,
    0x20, 0x6b, 0x65, 0x79, 0x20, 0x69, 0x73, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x69, 0x66,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x70, 0x61, 0x6e, 0x73, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x6b, 0x65, 0x79,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3a, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3a, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3a, 0x1b, 0x1c, 0x0a, 0x78, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x1b, 0x1a, 0x6b, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6e,
    0x67, 0x20, 0x75, 0x73, 0x65, 0x72, 0x2e, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20,
    0x77, 0x68, 0x65, 0x6e, 0x0a, 0x20, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x69, 0x6e,
    0x67, 0x20, 0x71, 0x75, 0x65, 0x75, 0x65, 0x64, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x61, 0x74, 0x20, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x20, 0x6e, 0x6f,
    0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x3d,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x3d, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3d, 0x19, 0x1a, 0x0a, 0x90, 0x01, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x40, 0x02, 0x30, 0x1a, 0x82, 0x01, 0x20, 0x52, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e,
    0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x63, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20,
    0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x6e, 0x67, 0x69,
    0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x49, 0x44, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x40, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x40, 0x24, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x40, 0x2e, 0x2f, 0x0a, 0xb5, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06,
    0x12, 0x03, 0x44, 0x02, 0x1d, 0x1a, 0xa7, 0x01, 0x20, 0x52, 0x61, 0x66, 0x74, 0x49, 0x44, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x44,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x61, 0x66, 0x74, 0x20, 0x63, 0x6f, 0x6e,
    0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x20, 0x77, 0x68, 0x69,
    0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x20, 0x20, 0x72, 0x61, 0x6e,
    0x67, 0x65, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x6e, 0x67, 0x73, 0x20, 0x74, 0x6f, 0x2e, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x6f, 0x64,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x6f, 0x72, 0x72, 0x65, 0x63, 0x74, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x44, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x44, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x44, 0x1b, 0x1c, 0x0a, 0x94, 0x04, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07,
    0x12, 0x03, 0x4e, 0x02, 0x31, 0x1a, 0x86, 0x04, 0x20, 0x55, 0x73, 0x65, 0x72, 0x50, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20,
    0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c,
    0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
    0x64, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x20, 0x5b, 0x31, 0x2c, 0x20,
    0x32, 0x5e, 0x33, 0x31, 0x2d, 0x31, 0x29, 0x2e, 0x0a, 0x20, 0x20, 0x49, 0x74, 0x27, 0x73, 0x20,
    0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x6c, 0x79, 0x20, 0x76, 0x69, 0x65, 0x77, 0x65, 0x64, 0x20,
    0x61, 0x73, 0x20, 0x61, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x0a, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x65, 0x76, 0x61,
    0x69, 0x6c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x6c, 0x69, 0x63, 0x74, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x73, 0x2e, 0x0a, 0x20,
    0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x55,
    0x73, 0x65, 0x72, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x3d, 0x31, 0x30, 0x30, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x31, 0x30, 0x30, 0x78, 0x20, 0x6c, 0x65, 0x73,
    0x73, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x6c, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x0a, 0x20,
    0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x6c, 0x69, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
    0x64, 0x73, 0x0a, 0x20, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x55, 0x73, 0x65, 0x72, 0x50, 0x72,
    0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x3d, 0x31, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x20,
    0x69, 0x66, 0x20, 0x54, 0x78, 0x6e, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x65, 0x69, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6e, 0x6f, 0x72,
    0x20, 0x54, 0x78, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65,
    0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x0a, 0x20, 0x20, 0x64,
    0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x31, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x4e, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x4e, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x08, 0x12,
    0x03, 0x4e, 0x23, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x07, 0x12, 0x03, 0x4e,
    0x2e, 0x2f, 0x0a, 0xb6, 0x02, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x54, 0x02, 0x30,
    0x1a, 0xa8, 0x02, 0x20, 0x54, 0x78, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6e,
    0x6f, 0x6e, 0x2d, 0x6e, 0x69, 0x6c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72,
    0x77, 0x61, 0x79, 0x2e, 0x20, 0x54, 0x6f, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x61, 0x20,
    0x74, 0x78, 0x6e, 0x2c, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x6e, 0x69, 0x6c, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x20, 0x69, 0x73, 0x6f, 0x6c, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x73, 0x65, 0x74, 0x20, 0x61, 0x73,
    0x20, 0x64, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x79, 0x2d,
    0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x78, 0x6e,
    0x20, 0x49, 0x44, 0x2c, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x2c, 0x20, 0x69,
    0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x0a, 0x20, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x08, 0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x08, 0x06, 0x12, 0x03, 0x54, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x54, 0x28, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x54, 0x2e, 0x2f, 0x0a, 0x99, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x58, 0x02,
    0x46, 0x1a, 0x8b, 0x01, 0x20, 0x52, 0x65, 0x61, 0x64, 0x43, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74,
    0x65, 0x6e, 0x63, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x72, 0x65, 0x61, 0x64, 0x0a, 0x20, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x43, 0x4f, 0x4e, 0x53, 0x49, 0x53, 0x54, 0x45, 0x4e, 0x54, 0x2e,
    0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x69,
    0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x77, 0x72, 0x69,
    0x74, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12, 0x03, 0x58, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x09, 0x06, 0x12, 0x03, 0x58, 0x0b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x58, 0x30, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x09, 0x03, 0x12, 0x03, 0x58, 0x43, 0x45, 0x0a, 0x4a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x5c,
    0x00, 0x6b, 0x01, 0x1a, 0x3e, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x73, 0x74, 0x6f, 0x72,
    0x61, 0x67, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x16, 0x0a,
    0x35, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x02, 0x2c, 0x1a, 0x28, 0x20, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x6e, 0x69, 0x6c, 0x20,
    0x69, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6f, 0x63, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5e,
    0x0b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5e, 0x22, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e, 0x2a, 0x2b, 0x0a, 0xb0,
    0x03, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x66, 0x02, 0x34, 0x1a, 0xa2, 0x03, 0x20,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x65, 0x73, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x61, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20,
    0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x77, 0x61, 0x73, 0x0a, 0x20, 0x20, 0x70,
    0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x72, 0x65,
    0x61, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x73, 0x2c, 0x20,
    0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x0a, 0x20, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x30,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x20, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20,
    0x68, 0x65, 0x72, 0x65, 0x2e, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x6c, 0x79, 0x2c, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x73, 0x65, 0x0a,
    0x20, 0x20, 0x6f, 0x66, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x73, 0x2c, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x69,
    0x6e, 0x63, 0x72, 0x65, 0x61, 0x73, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x70, 0x61, 0x73, 0x73,
    0x65, 0x64, 0x0a, 0x20, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x72, 0x69,
    0x74, 0x74, 0x65, 0x6e, 0x20, 0x77, 0x61, 0x73, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20,
    0x72, 0x65, 0x61, 0x64, 0x0a, 0x20, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65,
    0x6e, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x72, 0x65, 0x63, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x66, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x66, 0x0b, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x66, 0x26, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x66, 0x32, 0x33, 0x0a, 0xc4, 0x01, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x6a, 0x02, 0x30, 0x1a, 0xb6, 0x01, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x6e, 0x69,
    0x6c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x61, 0x20, 0x6e, 0x6f, 0x6e,
    0x2d, 0x6e, 0x69, 0x6c, 0x0a, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x61, 0x6e,
    0x64, 0x2f, 0x6f, 0x72, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x6d, 0x61,
    0x79, 0x20, 0x68, 0x61, 0x76, 0x65, 0x0a, 0x20, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x6a, 0x0b, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x28, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x6a, 0x2e, 0x2f, 0x0a, 0x46, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x6e, 0x00, 0x70, 0x01, 0x1a, 0x3a, 0x20, 0x41, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x02, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x6f, 0x0b, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x6f, 0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6f, 0x33,
    0x34, 0x0a, 0x4e, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x73, 0x00, 0x76, 0x01, 0x1a, 0x42, 0x20,
    0x41, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x73, 0x08, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x74, 0x02, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x74, 0x0b, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x74, 0x2b, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x74,
    0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x75, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x75, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x75, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x75, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x75, 0x19, 0x1a, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x79,
    0x00, 0x7b, 0x01, 0x1a, 0x30, 0x20, 0x41, 0x20, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x65, 0x74, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x79, 0x08,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x7a, 0x02, 0x35, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7a, 0x0b, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x7a, 0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x7a, 0x33, 0x34, 0x0a, 0x80, 0x01, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x05, 0x7f,
    0x00, 0x82, 0x01, 0x01, 0x1a, 0x73, 0x20, 0x41, 0x20, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x47, 0x65, 0x74, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a,
    0x20, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x64, 0x6f, 0x65,
    0x73, 0x6e, 0x27, 0x74, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x2c, 0x20, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x73, 0x20, 0x6e, 0x69, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x2e, 0x42, 0x79, 0x74, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x7f, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x04, 0x80,
    0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x80, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x04, 0x80, 0x01, 0x0b,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x2b, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x01, 0x34, 0x35, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01, 0x02, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0x81, 0x01, 0x22, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x81, 0x01, 0x2a, 0x2b, 0x0a, 0xb9, 0x01, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x06, 0x87, 0x01, 0x00, 0x8a, 0x01, 0x01, 0x1a, 0xaa, 0x01, 0x20, 0x41, 0x20, 0x50, 0x75,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50, 0x75, 0x74,
    0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x0a, 0x20, 0x20,
    0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65,
    0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x69, 0x6c, 0x6c, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x62, 0x6f, 0x74, 0x68,
    0x0a, 0x20, 0x20, 0x42, 0x79, 0x74, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x49, 0x6e, 0x74,
    0x65, 0x67, 0x65, 0x72, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x6e, 0x69, 0x6c, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0x87, 0x01,
    0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0x88, 0x01, 0x02, 0x35,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x88, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x04, 0x88, 0x01, 0x0b, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x01, 0x2a, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x01, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0x89, 0x01, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x89, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x06, 0x12, 0x04, 0x89, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x89, 0x01, 0x22, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x89, 0x01, 0x2a, 0x2b, 0x0a, 0x48, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x8d, 0x01,
    0x00, 0x8f, 0x01, 0x01, 0x1a, 0x3a, 0x20, 0x41, 0x20, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x50, 0x75, 0x74, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x06, 0x12, 0x04, 0x8e, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x8e, 0x01, 0x34, 0x35, 0x0a, 0xf5, 0x02, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06,
    0x96, 0x01, 0x00, 0x9e, 0x01, 0x01, 0x1a, 0xe6, 0x02, 0x20, 0x41, 0x20, 0x43, 0x6f, 0x6e, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61,
    0x6c, 0x50, 0x75, 0x74, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x20,
    0x20, 0x2d, 0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x74, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69,
    0x66, 0x20, 0x45, 0x78, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x65, 0x71, 0x75, 0x61, 0x6c,
    0x73, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x2e, 0x0a, 0x20, 0x20, 0x2d, 0x20, 0x49, 0x66, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x64, 0x6f, 0x65,
    0x73, 0x6e, 0x27, 0x74, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x45,
    0x78, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x69, 0x6c, 0x2c, 0x20,
    0x73, 0x65, 0x74, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x0a, 0x20, 0x20, 0x2d, 0x20,
    0x49, 0x66, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x2c, 0x20, 0x62,
    0x75, 0x74, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x65, 0x6d, 0x70, 0x74,
    0x79, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x45, 0x78, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6e, 0x69, 0x6c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x65, 0x6d,
    0x70, 0x74, 0x79, 0x2c, 0x20, 0x73, 0x65, 0x74, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e,
    0x0a, 0x20, 0x20, 0x2d, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x2c, 0x20,
    0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0x97, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x97, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x97, 0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x97, 0x01, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x97, 0x01, 0x33, 0x34, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04,
    0x99, 0x01, 0x02, 0x2c, 0x1a, 0x13, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x70, 0x75, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x99, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x06, 0x12, 0x04, 0x99, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x99, 0x01, 0x22, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x99, 0x01, 0x2a, 0x2b, 0x0a, 0xcd, 0x01, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x04,
    0x9d, 0x01, 0x02, 0x30, 0x1a, 0xbe, 0x01, 0x20, 0x45, 0x78, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x2e, 0x42, 0x79, 0x74, 0x65, 0x73, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x65, 0x73, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x65, 0x78, 0x69,
    0x73, 0x74, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20,
    0x61, 0x73, 0x20, 0x6e, 0x69, 0x6c, 0x0a, 0x20, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x20, 0x6e, 0x6f, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20,
    0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x20, 0x62, 0x75, 0x74, 0x20, 0x69, 0x73, 0x20, 0x65, 0x6d,
    0x70, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x9d, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x06, 0x12, 0x04, 0x9d,
    0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x01,
    0x22, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x2e,
    0x2f, 0x0a, 0x60, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xa2, 0x01, 0x00, 0xa4, 0x01, 0x01, 0x1a,
    0x52, 0x20, 0x41, 0x20, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50,
    0x75, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f,
    0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0xa3, 0x01, 0x02, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa3, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x34, 0x35, 0x0a, 0x8b, 0x03, 0x0a, 0x02, 0x04,
    0x0b, 0x12, 0x06, 0xac, 0x01, 0x00, 0xaf, 0x01, 0x01, 0x1a, 0xfc, 0x02, 0x20, 0x41, 0x6e, 0x20,
    0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x28, 0x29,
    0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x0a, 0x20, 0x20, 0x69, 0x6e,
    0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x0a, 0x20, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x61, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x62, 0x79, 0x20, 0x30, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x61, 0x20, 0x6e, 0x6f, 0x6f, 0x70, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x0a, 0x20, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x7a, 0x65, 0x72, 0x6f,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x20, 0x49, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x20,
    0x62, 0x65, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x6b,
    0x65, 0x79, 0x20, 0x73, 0x65, 0x74, 0x0a, 0x20, 0x20, 0x62, 0x79, 0x20, 0x50, 0x75, 0x74, 0x28,
    0x29, 0x20, 0x6f, 0x72, 0x20, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x50, 0x75, 0x74, 0x28, 0x29, 0x2e, 0x20, 0x53, 0x69, 0x6d, 0x69, 0x6c, 0x61, 0x72, 0x6c, 0x79,
    0x2c, 0x20, 0x50, 0x75, 0x74, 0x28, 0x29, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x43, 0x6f, 0x6e, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x75, 0x74, 0x28, 0x29, 0x0a, 0x20, 0x20, 0x63,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65, 0x64,
    0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x65, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12,
    0x04, 0xac, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xad,
    0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xad, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xad, 0x01, 0x0b,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x01, 0x2a, 0x30,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xad, 0x01, 0x33, 0x34, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x04, 0xae, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xae, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x04, 0xae, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xae, 0x01, 0x1d, 0x1e, 0x0a, 0xd3, 0x01, 0x0a, 0x02, 0x04, 0x0c,
    0x12, 0x06, 0xb4, 0x01, 0x00, 0xb7, 0x01, 0x01, 0x1a, 0xc4, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x49,
    0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x6e,
    0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20,
    0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20,
    0x69, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x4e, 0x65, 0x77, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x0a, 0x20, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x20, 0x61, 0x73,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xb5, 0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xb5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xb5, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb5, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xb5, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04,
    0xb6, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb6,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb6, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x11,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x1d, 0x1e,
    0x0a, 0x44, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0xba, 0x01, 0x00, 0xbc, 0x01, 0x01, 0x1a, 0x36,
    0x20, 0x41, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x28, 0x29, 0x20, 0x6d, 0x65,
    0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0xba,
    0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0xbb, 0x01, 0x02,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x0b, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x2a, 0x30, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x33, 0x34, 0x0a, 0x4e, 0x0a,
    0x02, 0x04, 0x0e, 0x12, 0x06, 0xbf, 0x01, 0x00, 0xc1, 0x01, 0x01, 0x1a, 0x40, 0x20, 0x41, 0x20,
    0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x00, 0x12, 0x04, 0xc0, 0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xc0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xc0, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xc0, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xc0, 0x01, 0x34, 0x35, 0x0a, 0x78, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xc5, 0x01, 0x00, 0xca,
    0x01, 0x01, 0x1a, 0x6a, 0x20, 0x41, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67,
    0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x65,
    0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    0x2e, 0x20, 0x49, 0x74, 0x0a, 0x20, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x6b, 0x65,
    0x79, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x01, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xc6, 0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc6, 0x01, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xc6, 0x01, 0x33, 0x34, 0x0a, 0x6e, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xc9,
    0x01, 0x02, 0x2b, 0x1a, 0x60, 0x20, 0x49, 0x66, 0x20, 0x30, 0x2c, 0x20, 0x2a, 0x61, 0x6c, 0x6c,
    0x2a, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65,
    0x6e, 0x20, 0x4b, 0x65, 0x79, 0x20, 0x28, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65,
    0x29, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x45, 0x6e, 0x64, 0x4b, 0x65, 0x79, 0x0a, 0x20, 0x20, 0x28,
    0x65, 0x78, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x29, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64,
    0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x2e, 0x20, 0x4d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20,
    0x3e, 0x3d, 0x20, 0x30, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xc9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc9,
    0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc9, 0x01,
    0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x29,
    0x2a, 0x0a, 0x5a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xce, 0x01, 0x00, 0xd2, 0x01, 0x01, 0x1a,
    0x4c, 0x20, 0x41, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x28, 0x29, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xce, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x00, 0x12, 0x04, 0xcf, 0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xcf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xcf, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xcf, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xcf, 0x01, 0x34, 0x35, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xd1, 0x01,
    0x02, 0x21, 0x1a, 0x1c, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x65,
    0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x11, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x1f, 0x20, 0x0a, 0x97, 0x01, 0x0a,
    0x02, 0x04, 0x11, 0x12, 0x06, 0xd6, 0x01, 0x00, 0xda, 0x01, 0x01, 0x1a, 0x88, 0x01, 0x20, 0x41,
    0x20, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x53, 0x63, 0x61, 0x6e, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20,
    0x49, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x65, 0x6e, 0x64,
    0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63,
    0x61, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d,
    0x75, 0x6d, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xd6,
    0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x01, 0x02,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd7, 0x01, 0x0b, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x2a, 0x30, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x33, 0x34, 0x0a, 0x1c, 0x0a,
    0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x02, 0x21, 0x1a, 0x0e, 0x20, 0x4d, 0x75,
    0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x3e, 0x20, 0x30, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x11, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xd9, 0x01, 0x1f, 0x20, 0x0a, 0x4a, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xdd,
    0x01, 0x00, 0xe1, 0x01, 0x01, 0x1a, 0x3c, 0x20, 0x41, 0x20, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x53, 0x63, 0x61, 0x6e, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f,
    0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xde, 0x01, 0x02, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xde, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04, 0xde, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xde, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xde, 0x01, 0x34, 0x35, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x12,
    0x02, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x02, 0x2e, 0x1a, 0x20, 0x20, 0x45, 0x6d, 0x70, 0x74, 0x79,
    0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x20, 0x72, 0x6f, 0x77, 0x73, 0x20, 0x77, 0x65, 0x72, 0x65,
    0x20, 0x73, 0x63, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xe0, 0x01, 0x0b, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xe0, 0x01, 0x25, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xe0, 0x01, 0x2c, 0x2d, 0x0a, 0x9b, 0x01, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xe5,
    0x01, 0x00, 0xed, 0x01, 0x01, 0x1a, 0x8c, 0x01, 0x20, 0x41, 0x6e, 0x20, 0x45, 0x6e, 0x64, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x20,
    0x20, 0x49, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x77, 0x68,
    0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x20,
    0x6f, 0x72, 0x20, 0x72, 0x6f, 0x6c, 0x6c, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x20, 0x61, 0x6e, 0x20,
    0x65, 0x78, 0x74, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x35, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe6, 0x01, 0x0b, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x33, 0x34, 0x0a, 0x2c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x02, 0x1b, 0x1a, 0x1e, 0x20, 0x46, 0x61, 0x6c, 0x73,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72,
    0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xe8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xe8, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xe8, 0x01, 0x10, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xe8, 0x01, 0x19, 0x1a, 0x0a, 0xa5, 0x01, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04,
    0xec, 0x01, 0x02, 0x4e, 0x1a, 0x96, 0x01, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x73,
    0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20,
    0x66, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x2d, 0x66, 0x61,
    0x63, 0x69, 0x6e, 0x67, 0x20, 0x4b, 0x56, 0x20, 0x41, 0x50, 0x49, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0xec, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x02, 0x06, 0x12, 0x04, 0xec, 0x01, 0x0b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xec, 0x01, 0x32, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xec, 0x01, 0x4c, 0x4d, 0x0a, 0xf2, 0x04, 0x0a, 0x02, 0x04, 0x14,
    0x12, 0x06, 0xf9, 0x01, 0x00, 0xff, 0x01, 0x01, 0x1a, 0xe3, 0x04, 0x20, 0x41, 0x6e, 0x20, 0x45,
    0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x74,
    0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74,
    0x68, 0x65, 0x0a, 0x20, 0x20, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x66, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x61, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x63, 0x75, 0x6c, 0x61, 0x72, 0x2c, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x0a, 0x20, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x62, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65,
    0x66, 0x6c, 0x65, 0x63, 0x74, 0x20, 0x66, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x20,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x70, 0x72, 0x6f, 0x70,
    0x61, 0x67, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20,
    0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x66, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x74,
    0x78, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x20, 0x69, 0x6e, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x70, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x20, 0x63, 0x61, 0x75, 0x73, 0x61, 0x6c, 0x20,
    0x6f, 0x72, 0x64, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65,
    0x65, 0x6e, 0x20, 0x73, 0x75, 0x62, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x20, 0x43, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x57, 0x61, 0x69, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x0a, 0x20, 0x20, 0x77, 0x61,
    0x69, 0x74, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x4d, 0x55, 0x53, 0x54, 0x20,
    0x77, 0x61, 0x69, 0x74, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x0a, 0x20, 0x20, 0x73, 0x69,
    0x67, 0x6e, 0x61, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x20,
    0x6e, 0x6f, 0x64, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x01, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xfa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xfa, 0x01, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xfa, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xfa, 0x01, 0x34, 0x35, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xfc,
    0x01, 0x02, 0x21, 0x1a, 0x16, 0x20, 0x52, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x20, 0x28, 0x6e, 0x73, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x11, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xfc, 0x01, 0x1f, 0x20, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12,
    0x04, 0xfe, 0x01, 0x02, 0x1e, 0x1a, 0x32, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x45, 0x6e, 0x64, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xfe, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xfe, 0x01, 0x11, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xfe, 0x01, 0x1c, 0x1d, 0x0a, 0x91, 0x01, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0x83, 0x02,
    0x00, 0x8f, 0x02, 0x01, 0x1a, 0x82, 0x01, 0x20, 0x41, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20,
    0x65, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61,
    0x64, 0x64, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x73, 0x20, 0x77, 0x65, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01,
    0x12, 0x04, 0x83, 0x02, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x15, 0x08, 0x00, 0x12, 0x06,
    0x84, 0x02, 0x03, 0x8e, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x08, 0x00, 0x01, 0x12,
    0x04, 0x84, 0x02, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x85,
    0x02, 0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0x85, 0x02,
    0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x85, 0x02, 0x25,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x85, 0x02, 0x30, 0x31,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x86, 0x02, 0x04, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0x86, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0x86, 0x02, 0x20, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0x86, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x02, 0x12, 0x04, 0x87, 0x02, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x87, 0x02, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x87, 0x02, 0x20, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x87, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04,
    0x88, 0x02, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06, 0x12, 0x04, 0x88,
    0x02, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0x88, 0x02,
    0x2b, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0x88, 0x02, 0x3d,
    0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0x89, 0x02, 0x04, 0x34, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x06, 0x12, 0x04, 0x89, 0x02, 0x04, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01, 0x12, 0x04, 0x89, 0x02, 0x26, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12, 0x04, 0x89, 0x02, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x05, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x05, 0x06, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x23, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05,
    0x03, 0x12, 0x04, 0x8a, 0x02, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x06, 0x12,
    0x04, 0x8b, 0x02, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x06, 0x12, 0x04,
    0x8b, 0x02, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x01, 0x12, 0x04, 0x8b,
    0x02, 0x28, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x03, 0x12, 0x04, 0x8b, 0x02,
    0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x07, 0x12, 0x04, 0x8c, 0x02, 0x04, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x06, 0x12, 0x04, 0x8c, 0x02, 0x04, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x21, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x08, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x08, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x08, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x2b, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x08, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x3d, 0x3e, 0x0a, 0x94, 0x01, 0x0a, 0x02, 0x04, 0x16, 0x12,
    0x06, 0x93, 0x02, 0x00, 0x9f, 0x02, 0x01, 0x1a, 0x85, 0x01, 0x20, 0x41, 0x20, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x73, 0x20, 0x65, 0x78, 0x61, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2e, 0x0a, 0x20, 0x20, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x73, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x6d,
    0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x73, 0x20, 0x77, 0x65, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x93, 0x02, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x16, 0x08, 0x00, 0x12, 0x06, 0x94, 0x02, 0x03, 0x9e, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x08, 0x00, 0x01, 0x12, 0x04, 0x94, 0x02, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x00, 0x12, 0x04, 0x95, 0x02, 0x04, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x95, 0x02, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x95, 0x02, 0x26, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x95, 0x02, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04,
    0x96, 0x02, 0x04, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x06, 0x12, 0x04, 0x96,
    0x02, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x02,
    0x21, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x96, 0x02, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x02, 0x12, 0x04, 0x97, 0x02, 0x04, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x06, 0x12, 0x04, 0x97, 0x02, 0x04, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01, 0x12, 0x04, 0x97, 0x02, 0x21, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12, 0x04, 0x97, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x03, 0x12, 0x04, 0x98, 0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x03, 0x06, 0x12, 0x04, 0x98, 0x02, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x98, 0x02, 0x2c, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x98, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x04, 0x12,
    0x04, 0x99, 0x02, 0x04, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x99, 0x02, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x01, 0x12, 0x04, 0x99,
    0x02, 0x27, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x03, 0x12, 0x04, 0x99, 0x02,
    0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x05, 0x12, 0x04, 0x9a, 0x02, 0x04, 0x2f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x06, 0x12, 0x04, 0x9a, 0x02, 0x04, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x24, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x03, 0x12, 0x04, 0x9a, 0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x16, 0x02, 0x06, 0x12, 0x04, 0x9b, 0x02, 0x04, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x06, 0x06, 0x12, 0x04, 0x9b, 0x02, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x06, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x29, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x06, 0x03, 0x12, 0x04, 0x9b, 0x02, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x07,
    0x12, 0x04, 0x9c, 0x02, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x06, 0x12,
    0x04, 0x9c, 0x02, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x01, 0x12, 0x04,
    0x9c, 0x02, 0x22, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x03, 0x12, 0x04, 0x9c,
    0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x08, 0x12, 0x04, 0x9d, 0x02, 0x04,
    0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x06, 0x12, 0x04, 0x9d, 0x02, 0x04, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x2c, 0x3b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x3e, 0x3f, 0x0a, 0x85,
    0x02, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xa6, 0x02, 0x00, 0xa9, 0x02, 0x01, 0x1a, 0xf6, 0x01,
    0x20, 0x41, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x72, 0x20,
    0x6d, 0x6f, 0x72, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x0a,
    0x20, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6c, 0x6c, 0x65, 0x6c, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x69,
    0x66, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x28, 0x62, 0x61,
    0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2d, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20,
    0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2d, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x29,
    0x2c, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4b, 0x65, 0x79,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62,
    0x61, 0x74, 0x63, 0x68, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xa6,
    0x02, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xa7, 0x02, 0x02,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa7, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa7, 0x02, 0x0b, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x2a, 0x30, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa7, 0x02, 0x33, 0x34, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xa8, 0x02, 0x0b, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x29, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xa8, 0x02, 0x34, 0x35, 0x0a, 0xf9, 0x01, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06,
    0xaf, 0x02, 0x00, 0xb2, 0x02, 0x01, 0x1a, 0xea, 0x01, 0x20, 0x41, 0x20, 0x42, 0x61, 0x74, 0x63,
    0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x2c, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x70, 0x65,
    0x72, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x72, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20,
    0x73, 0x6c, 0x69, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x73, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c,
    0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x02, 0x02, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb0, 0x02, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x02, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb0, 0x02, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x01, 0x12, 0x04, 0xb1, 0x02, 0x02, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xb1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06,
    0x12, 0x04, 0xb1, 0x02, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xb1, 0x02, 0x2a, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xb1, 0x02, 0x36, 0x37, 0x0a, 0xf9, 0x08, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xc6, 0x02, 0x00,
    0xc9, 0x02, 0x01, 0x1a, 0xea, 0x08, 0x20, 0x41, 0x6e, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53,
    0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61,
    0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x4b, 0x65, 0x79, 0x20, 0x69, 0x73, 0x20, 0x73, 0x70,
    0x6c, 0x69, 0x74, 0x20, 0x62, 0x79, 0x0a, 0x20, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b,
    0x65, 0x79, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b, 0x65, 0x79,
    0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65,
    0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x0a, 0x20, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72,
    0x6d, 0x69, 0x6e, 0x65, 0x20, 0x61, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x6b, 0x65, 0x79,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x6c, 0x79,
    0x20, 0x68, 0x61, 0x6c, 0x66, 0x77, 0x61, 0x79, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x63,
    0x6f, 0x76, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x74, 0x73, 0x20, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x6e, 0x65, 0x77, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x73, 0x20, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x73,
    0x70, 0x6c, 0x69, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x27, 0x73, 0x20, 0x65, 0x6e, 0x64,
    0x20, 0x6b, 0x65, 0x79, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b,
    0x65, 0x79, 0x0a, 0x20, 0x20, 0x69, 0x73, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x2c, 0x20, 0x68,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x6b, 0x65, 0x79, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x61, 0x6c, 0x73, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x20, 0x20, 0x4e, 0x65, 0x77,
    0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x49, 0x44, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65,
    0x61, 0x63, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74,
    0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x27, 0x73, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x52, 0x61, 0x66, 0x74, 0x0a,
    0x20, 0x20, 0x49, 0x44, 0x20, 0x61, 0x72, 0x65, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x65, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68,
    0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x20, 0x72, 0x65, 0x63, 0x6f,
    0x72, 0x64, 0x73, 0x2c, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x66, 0x69, 0x6e, 0x61, 0x6c, 0x6c, 0x79, 0x2c,
    0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x20,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x62, 0x6f, 0x6f, 0x6b, 0x6b, 0x65, 0x65, 0x70, 0x69,
    0x6e, 0x67, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x69, 0x61,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x20, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x73, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x73, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x3b, 0x0a,
    0x20, 0x20, 0x6e, 0x6f, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20,
    0x69, 0x73, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x20, 0x64, 0x75, 0x72, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65,
    0x0a, 0x20, 0x20, 0x74, 0x68, 0x6f, 0x75, 0x67, 0x68, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x73,
    0x20, 0x61, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x6c, 0x79, 0x20, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61,
    0x6c, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x74, 0x68, 0x6f,
    0x75, 0x67, 0x68, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x20,
    0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x6d,
    0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x70, 0x69, 0x65, 0x64, 0x20, 0x6f, 0x72,
    0x0a, 0x20, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x65, 0x64, 0x29, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xc6, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xc7, 0x02, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xc7, 0x02, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc7, 0x02, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12,
    0x04, 0xc8, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xc8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc8,
    0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc8, 0x02,
    0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc8, 0x02, 0x1d,
    0x1e, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xcd, 0x02, 0x00, 0xcf, 0x02, 0x01, 0x1a,
    0x4b, 0x20, 0x41, 0x6e, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x28,
    0x29, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x1a, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02,
    0x00, 0x12, 0x04, 0xce, 0x02, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xce, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xce, 0x02, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xce, 0x02, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xce,
    0x02, 0x34, 0x35, 0x0a, 0xc7, 0x04, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xda, 0x02, 0x00, 0xdc,
    0x02, 0x01, 0x1a, 0xb8, 0x04, 0x20, 0x41, 0x6e, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65,
    0x72, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72,
    0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41,
    0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x28, 0x29, 0x20, 0x6d, 0x65, 0x74, 0x68,
    0x6f, 0x64, 0x2e, 0x20, 0x41, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x63, 0x61,
    0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65, 0x72, 0x67, 0x65,
    0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x66, 0x74, 0x2d, 0x68, 0x61, 0x6e,
    0x64, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x20, 0x74, 0x77, 0x6f,
    0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x75, 0x74, 0x69, 0x76, 0x65, 0x20, 0x72, 0x61, 0x6e,
    0x67, 0x65, 0x73, 0x20, 0x28, 0x69, 0x2e, 0x65, 0x2e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x20,
    0x73, 0x6f, 0x72, 0x74, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x29, 0x2e, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x62, 0x73, 0x75, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x69, 0x67, 0x68,
    0x74, 0x0a, 0x20, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x75, 0x62, 0x73, 0x75, 0x6d, 0x65, 0x64, 0x2e,
    0x20, 0x41, 0x66, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x72, 0x67, 0x65,
    0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x0a,
    0x20, 0x20, 0x73, 0x75, 0x62, 0x73, 0x75, 0x6d, 0x65, 0x64, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x65, 0x72, 0x20,
    0x65, 0x78, 0x69, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x75,
    0x62, 0x73, 0x75, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x0a, 0x20, 0x20, 0x6e, 0x6f, 0x77, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x61,
    0x73, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x69, 0x74, 0x73, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65,
    0x6e, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x20, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x75, 0x62, 0x73, 0x75, 0x6d, 0x65, 0x64, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x20,
    0x49, 0x66, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x69, 0x6e, 0x61, 0x6c, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x73, 0x70, 0x61, 0x63, 0x65, 0x2c, 0x20, 0x69,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x6f, 0x6f, 0x70, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xda, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b,
    0x02, 0x00, 0x12, 0x04, 0xdb, 0x02, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xdb, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xdb, 0x02, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xdb, 0x02, 0x2a, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xdb, 0x02, 0x33, 0x34, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xe0, 0x02, 0x00, 0xe2,
    0x02, 0x01, 0x1a, 0x4b, 0x20, 0x41, 0x6e, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65, 0x72,
    0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x4d, 0x65, 0x72,
    0x67, 0x65, 0x28, 0x29, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xe0, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x02, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xe1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xe1, 0x02, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xe1, 0x02, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe1, 0x02, 0x34, 0x35,
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
