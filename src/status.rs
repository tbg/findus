// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
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
pub struct StoreStatus {
    // message fields
    desc: ::protobuf::SingularPtrField<StoreDescriptor>,
    node_id: ::std::option::Option<i32>,
    range_count: ::std::option::Option<i32>,
    started_at: ::std::option::Option<i64>,
    updated_at: ::std::option::Option<i64>,
    stats: ::protobuf::SingularPtrField<MVCCStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StoreStatus {
    pub fn new() -> StoreStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreStatus {
        static mut instance: ::protobuf::lazy::Lazy<StoreStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreStatus,
        };
        unsafe {
            instance.get(|| {
                StoreStatus {
                    desc: ::protobuf::SingularPtrField::none(),
                    node_id: ::std::option::Option::None,
                    range_count: ::std::option::Option::None,
                    started_at: ::std::option::Option::None,
                    updated_at: ::std::option::Option::None,
                    stats: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.StoreDescriptor desc = 1;

    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: StoreDescriptor) {
        self.desc = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc<'a>(&'a mut self) -> &'a mut StoreDescriptor {
        if self.desc.is_none() {
            self.desc.set_default();
        };
        self.desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc(&mut self) -> StoreDescriptor {
        self.desc.take().unwrap_or_else(|| StoreDescriptor::new())
    }

    pub fn get_desc<'a>(&'a self) -> &'a StoreDescriptor {
        self.desc.as_ref().unwrap_or_else(|| StoreDescriptor::default_instance())
    }

    // optional int32 node_id = 2;

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

    // optional int32 range_count = 3;

    pub fn clear_range_count(&mut self) {
        self.range_count = ::std::option::Option::None;
    }

    pub fn has_range_count(&self) -> bool {
        self.range_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_count(&mut self, v: i32) {
        self.range_count = ::std::option::Option::Some(v);
    }

    pub fn get_range_count<'a>(&self) -> i32 {
        self.range_count.unwrap_or(0)
    }

    // optional int64 started_at = 4;

    pub fn clear_started_at(&mut self) {
        self.started_at = ::std::option::Option::None;
    }

    pub fn has_started_at(&self) -> bool {
        self.started_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_started_at(&mut self, v: i64) {
        self.started_at = ::std::option::Option::Some(v);
    }

    pub fn get_started_at<'a>(&self) -> i64 {
        self.started_at.unwrap_or(0)
    }

    // optional int64 updated_at = 5;

    pub fn clear_updated_at(&mut self) {
        self.updated_at = ::std::option::Option::None;
    }

    pub fn has_updated_at(&self) -> bool {
        self.updated_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_at(&mut self, v: i64) {
        self.updated_at = ::std::option::Option::Some(v);
    }

    pub fn get_updated_at<'a>(&self) -> i64 {
        self.updated_at.unwrap_or(0)
    }

    // optional .cockroach.proto.MVCCStats stats = 6;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: MVCCStats) {
        self.stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats<'a>(&'a mut self) -> &'a mut MVCCStats {
        if self.stats.is_none() {
            self.stats.set_default();
        };
        self.stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats(&mut self) -> MVCCStats {
        self.stats.take().unwrap_or_else(|| MVCCStats::new())
    }

    pub fn get_stats<'a>(&'a self) -> &'a MVCCStats {
        self.stats.as_ref().unwrap_or_else(|| MVCCStats::default_instance())
    }
}

impl ::protobuf::Message for StoreStatus {
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
                    let tmp = self.desc.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.node_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.range_count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.started_at = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.updated_at = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.stats.set_default();
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
        for value in self.desc.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.node_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.range_count.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.started_at.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.updated_at.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.stats.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.desc.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.node_id {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.range_count {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.started_at {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.updated_at {
            try!(os.write_int64(5, v));
        };
        if let Some(v) = self.stats.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<StoreStatus>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StoreStatus {
    fn new() -> StoreStatus {
        StoreStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "desc",
                    StoreStatus::has_desc,
                    StoreStatus::get_desc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "node_id",
                    StoreStatus::has_node_id,
                    StoreStatus::get_node_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "range_count",
                    StoreStatus::has_range_count,
                    StoreStatus::get_range_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "started_at",
                    StoreStatus::has_started_at,
                    StoreStatus::get_started_at,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "updated_at",
                    StoreStatus::has_updated_at,
                    StoreStatus::get_updated_at,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stats",
                    StoreStatus::has_stats,
                    StoreStatus::get_stats,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StoreStatus>(
                    "StoreStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreStatus {
    fn clear(&mut self) {
        self.clear_desc();
        self.clear_node_id();
        self.clear_range_count();
        self.clear_started_at();
        self.clear_updated_at();
        self.clear_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StoreStatus {
    fn eq(&self, other: &StoreStatus) -> bool {
        self.desc == other.desc &&
        self.node_id == other.node_id &&
        self.range_count == other.range_count &&
        self.started_at == other.started_at &&
        self.updated_at == other.updated_at &&
        self.stats == other.stats &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StoreStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NodeStatus {
    // message fields
    desc: ::protobuf::SingularPtrField<NodeDescriptor>,
    store_ids: ::std::vec::Vec<i32>,
    range_count: ::std::option::Option<i32>,
    started_at: ::std::option::Option<i64>,
    updated_at: ::std::option::Option<i64>,
    stats: ::protobuf::SingularPtrField<MVCCStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl NodeStatus {
    pub fn new() -> NodeStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeStatus {
        static mut instance: ::protobuf::lazy::Lazy<NodeStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeStatus,
        };
        unsafe {
            instance.get(|| {
                NodeStatus {
                    desc: ::protobuf::SingularPtrField::none(),
                    store_ids: ::std::vec::Vec::new(),
                    range_count: ::std::option::Option::None,
                    started_at: ::std::option::Option::None,
                    updated_at: ::std::option::Option::None,
                    stats: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .cockroach.proto.NodeDescriptor desc = 1;

    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    pub fn has_desc(&self) -> bool {
        self.desc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: NodeDescriptor) {
        self.desc = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc<'a>(&'a mut self) -> &'a mut NodeDescriptor {
        if self.desc.is_none() {
            self.desc.set_default();
        };
        self.desc.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc(&mut self) -> NodeDescriptor {
        self.desc.take().unwrap_or_else(|| NodeDescriptor::new())
    }

    pub fn get_desc<'a>(&'a self) -> &'a NodeDescriptor {
        self.desc.as_ref().unwrap_or_else(|| NodeDescriptor::default_instance())
    }

    // repeated int32 store_ids = 2;

    pub fn clear_store_ids(&mut self) {
        self.store_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_store_ids(&mut self, v: ::std::vec::Vec<i32>) {
        self.store_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_store_ids<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.store_ids
    }

    // Take field
    pub fn take_store_ids(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.store_ids, ::std::vec::Vec::new())
    }

    pub fn get_store_ids<'a>(&'a self) -> &'a [i32] {
        &self.store_ids
    }

    // optional int32 range_count = 3;

    pub fn clear_range_count(&mut self) {
        self.range_count = ::std::option::Option::None;
    }

    pub fn has_range_count(&self) -> bool {
        self.range_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_count(&mut self, v: i32) {
        self.range_count = ::std::option::Option::Some(v);
    }

    pub fn get_range_count<'a>(&self) -> i32 {
        self.range_count.unwrap_or(0)
    }

    // optional int64 started_at = 4;

    pub fn clear_started_at(&mut self) {
        self.started_at = ::std::option::Option::None;
    }

    pub fn has_started_at(&self) -> bool {
        self.started_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_started_at(&mut self, v: i64) {
        self.started_at = ::std::option::Option::Some(v);
    }

    pub fn get_started_at<'a>(&self) -> i64 {
        self.started_at.unwrap_or(0)
    }

    // optional int64 updated_at = 5;

    pub fn clear_updated_at(&mut self) {
        self.updated_at = ::std::option::Option::None;
    }

    pub fn has_updated_at(&self) -> bool {
        self.updated_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_at(&mut self, v: i64) {
        self.updated_at = ::std::option::Option::Some(v);
    }

    pub fn get_updated_at<'a>(&self) -> i64 {
        self.updated_at.unwrap_or(0)
    }

    // optional .cockroach.proto.MVCCStats stats = 6;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: MVCCStats) {
        self.stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats<'a>(&'a mut self) -> &'a mut MVCCStats {
        if self.stats.is_none() {
            self.stats.set_default();
        };
        self.stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats(&mut self) -> MVCCStats {
        self.stats.take().unwrap_or_else(|| MVCCStats::new())
    }

    pub fn get_stats<'a>(&'a self) -> &'a MVCCStats {
        self.stats.as_ref().unwrap_or_else(|| MVCCStats::default_instance())
    }
}

impl ::protobuf::Message for NodeStatus {
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
                    let tmp = self.desc.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.store_ids));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.range_count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.started_at = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.updated_at = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.stats.set_default();
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
        for value in self.desc.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.store_ids.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.range_count.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.started_at.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.updated_at.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.stats.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.desc.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.store_ids.iter() {
            try!(os.write_int32(2, *v));
        };
        if let Some(v) = self.range_count {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.started_at {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.updated_at {
            try!(os.write_int64(5, v));
        };
        if let Some(v) = self.stats.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<NodeStatus>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodeStatus {
    fn new() -> NodeStatus {
        NodeStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "desc",
                    NodeStatus::has_desc,
                    NodeStatus::get_desc,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "store_ids",
                    NodeStatus::get_store_ids,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "range_count",
                    NodeStatus::has_range_count,
                    NodeStatus::get_range_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "started_at",
                    NodeStatus::has_started_at,
                    NodeStatus::get_started_at,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "updated_at",
                    NodeStatus::has_updated_at,
                    NodeStatus::get_updated_at,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stats",
                    NodeStatus::has_stats,
                    NodeStatus::get_stats,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeStatus>(
                    "NodeStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeStatus {
    fn clear(&mut self) {
        self.clear_desc();
        self.clear_store_ids();
        self.clear_range_count();
        self.clear_started_at();
        self.clear_updated_at();
        self.clear_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NodeStatus {
    fn eq(&self, other: &NodeStatus) -> bool {
        self.desc == other.desc &&
        self.store_ids == other.store_ids &&
        self.range_count == other.range_count &&
        self.started_at == other.started_at &&
        self.updated_at == other.updated_at &&
        self.stats == other.stats &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NodeStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1a, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2f, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb6, 0x01, 0x0a, 0x0b, 0x53, 0x74,
    0x6f, 0x72, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x2e, 0x0a, 0x04, 0x64, 0x65, 0x73,
    0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x44,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x6e, 0x6f, 0x64,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x12, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x03, 0x12, 0x12, 0x0a, 0x0a, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61,
    0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03, 0x12, 0x29, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x73,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4d, 0x56, 0x43, 0x43, 0x53, 0x74, 0x61,
    0x74, 0x73, 0x22, 0xb6, 0x01, 0x0a, 0x0a, 0x4e, 0x6f, 0x64, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x12, 0x2d, 0x0a, 0x04, 0x64, 0x65, 0x73, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72,
    0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x12, 0x12, 0x0a, 0x0a,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x03,
    0x12, 0x29, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x4d, 0x56, 0x43, 0x43, 0x53, 0x74, 0x61, 0x74, 0x73, 0x42, 0x07, 0x5a, 0x05, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x4a, 0x93, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1c, 0x01, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x03, 0x07, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x04, 0x08, 0x17, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x06, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x06, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x06, 0x14,
    0x1b, 0x0a, 0x61, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x11, 0x01, 0x1a, 0x55, 0x20,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x6e,
    0x65, 0x65, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x0a, 0x20, 0x20, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x35, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b, 0x0b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0b, 0x2c, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0b, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c,
    0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x11,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0e, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0e, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x10, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x10,
    0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x26, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x2e, 0x2f, 0x0a, 0x60,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x1c, 0x01, 0x1a, 0x54, 0x20, 0x4e, 0x6f, 0x64,
    0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x61, 0x6c, 0x63, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x0a, 0x20, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x15, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x16, 0x0b, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x16, 0x2b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x32,
    0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x17, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x18, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x18, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x18, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x11, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x03, 0x12, 0x03, 0x19, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x19, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x19,
    0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x19, 0x1e, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x1a, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x1a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x1b,
    0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x06, 0x12, 0x03, 0x1b, 0x0b, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1b, 0x26, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1b, 0x2e, 0x2f,
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
