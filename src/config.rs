// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Attributes {
    // message fields
    attrs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Attributes {
    pub fn new() -> Attributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Attributes {
        static mut instance: ::protobuf::lazy::Lazy<Attributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Attributes,
        };
        unsafe {
            instance.get(|| {
                Attributes {
                    attrs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string attrs = 1;

    pub fn clear_attrs(&mut self) {
        self.attrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_attrs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.attrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attrs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.attrs
    }

    // Take field
    pub fn take_attrs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.attrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_attrs<'a>(&'a self) -> &'a [::std::string::String] {
        &self.attrs
    }
}

impl ::protobuf::Message for Attributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.attrs));
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
        for value in self.attrs.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.attrs.iter() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<Attributes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Attributes {
    fn new() -> Attributes {
        Attributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<Attributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "attrs",
                    Attributes::get_attrs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Attributes>(
                    "Attributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Attributes {
    fn clear(&mut self) {
        self.clear_attrs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Attributes {
    fn eq(&self, other: &Attributes) -> bool {
        self.attrs == other.attrs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Attributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Replica {
    // message fields
    node_id: ::std::option::Option<i32>,
    store_id: ::std::option::Option<i32>,
    attrs: ::protobuf::SingularPtrField<Attributes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Replica {
    pub fn new() -> Replica {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Replica {
        static mut instance: ::protobuf::lazy::Lazy<Replica> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Replica,
        };
        unsafe {
            instance.get(|| {
                Replica {
                    node_id: ::std::option::Option::None,
                    store_id: ::std::option::Option::None,
                    attrs: ::protobuf::SingularPtrField::none(),
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

    // optional int32 store_id = 2;

    pub fn clear_store_id(&mut self) {
        self.store_id = ::std::option::Option::None;
    }

    pub fn has_store_id(&self) -> bool {
        self.store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store_id(&mut self, v: i32) {
        self.store_id = ::std::option::Option::Some(v);
    }

    pub fn get_store_id<'a>(&self) -> i32 {
        self.store_id.unwrap_or(0)
    }

    // optional .cockroach.proto.Attributes attrs = 3;

    pub fn clear_attrs(&mut self) {
        self.attrs.clear();
    }

    pub fn has_attrs(&self) -> bool {
        self.attrs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attrs(&mut self, v: Attributes) {
        self.attrs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attrs<'a>(&'a mut self) -> &'a mut Attributes {
        if self.attrs.is_none() {
            self.attrs.set_default();
        };
        self.attrs.as_mut().unwrap()
    }

    // Take field
    pub fn take_attrs(&mut self) -> Attributes {
        self.attrs.take().unwrap_or_else(|| Attributes::new())
    }

    pub fn get_attrs<'a>(&'a self) -> &'a Attributes {
        self.attrs.as_ref().unwrap_or_else(|| Attributes::default_instance())
    }
}

impl ::protobuf::Message for Replica {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.store_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attrs.set_default();
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
        for value in self.node_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.store_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.attrs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.store_id {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.attrs.as_ref() {
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
        ::std::any::TypeId::of::<Replica>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Replica {
    fn new() -> Replica {
        Replica::new()
    }

    fn descriptor_static(_: ::std::option::Option<Replica>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "node_id",
                    Replica::has_node_id,
                    Replica::get_node_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "store_id",
                    Replica::has_store_id,
                    Replica::get_store_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "attrs",
                    Replica::has_attrs,
                    Replica::get_attrs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Replica>(
                    "Replica",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Replica {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_store_id();
        self.clear_attrs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Replica {
    fn eq(&self, other: &Replica) -> bool {
        self.node_id == other.node_id &&
        self.store_id == other.store_id &&
        self.attrs == other.attrs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Replica {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RangeDescriptor {
    // message fields
    raft_id: ::std::option::Option<i64>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    replicas: ::protobuf::RepeatedField<Replica>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RangeDescriptor {
    pub fn new() -> RangeDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RangeDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<RangeDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RangeDescriptor,
        };
        unsafe {
            instance.get(|| {
                RangeDescriptor {
                    raft_id: ::std::option::Option::None,
                    start_key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
                    replicas: ::protobuf::RepeatedField::new(),
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

    // optional bytes start_key = 2;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    pub fn has_start_key(&self) -> bool {
        self.start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key<'a>(&'a self) -> &'a [u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes end_key = 3;

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

    // repeated .cockroach.proto.Replica replicas = 4;

    pub fn clear_replicas(&mut self) {
        self.replicas.clear();
    }

    // Param is passed by value, moved
    pub fn set_replicas(&mut self, v: ::protobuf::RepeatedField<Replica>) {
        self.replicas = v;
    }

    // Mutable pointer to the field.
    pub fn mut_replicas<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Replica> {
        &mut self.replicas
    }

    // Take field
    pub fn take_replicas(&mut self) -> ::protobuf::RepeatedField<Replica> {
        ::std::mem::replace(&mut self.replicas, ::protobuf::RepeatedField::new())
    }

    pub fn get_replicas<'a>(&'a self) -> &'a [Replica] {
        &self.replicas
    }
}

impl ::protobuf::Message for RangeDescriptor {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.start_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.end_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.replicas));
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
        for value in self.start_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.end_key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.replicas.iter() {
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
        if let Some(v) = self.start_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        for v in self.replicas.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<RangeDescriptor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RangeDescriptor {
    fn new() -> RangeDescriptor {
        RangeDescriptor::new()
    }

    fn descriptor_static(_: ::std::option::Option<RangeDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "raft_id",
                    RangeDescriptor::has_raft_id,
                    RangeDescriptor::get_raft_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    RangeDescriptor::has_start_key,
                    RangeDescriptor::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    RangeDescriptor::has_end_key,
                    RangeDescriptor::get_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "replicas",
                    RangeDescriptor::get_replicas,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RangeDescriptor>(
                    "RangeDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RangeDescriptor {
    fn clear(&mut self) {
        self.clear_raft_id();
        self.clear_start_key();
        self.clear_end_key();
        self.clear_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RangeDescriptor {
    fn eq(&self, other: &RangeDescriptor) -> bool {
        self.raft_id == other.raft_id &&
        self.start_key == other.start_key &&
        self.end_key == other.end_key &&
        self.replicas == other.replicas &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RangeDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GCPolicy {
    // message fields
    ttl_seconds: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GCPolicy {
    pub fn new() -> GCPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GCPolicy {
        static mut instance: ::protobuf::lazy::Lazy<GCPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GCPolicy,
        };
        unsafe {
            instance.get(|| {
                GCPolicy {
                    ttl_seconds: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 ttl_seconds = 1;

    pub fn clear_ttl_seconds(&mut self) {
        self.ttl_seconds = ::std::option::Option::None;
    }

    pub fn has_ttl_seconds(&self) -> bool {
        self.ttl_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl_seconds(&mut self, v: i32) {
        self.ttl_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_ttl_seconds<'a>(&self) -> i32 {
        self.ttl_seconds.unwrap_or(0)
    }
}

impl ::protobuf::Message for GCPolicy {
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
                    self.ttl_seconds = ::std::option::Option::Some(tmp);
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
        for value in self.ttl_seconds.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ttl_seconds {
            try!(os.write_int32(1, v));
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
        ::std::any::TypeId::of::<GCPolicy>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GCPolicy {
    fn new() -> GCPolicy {
        GCPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<GCPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "ttl_seconds",
                    GCPolicy::has_ttl_seconds,
                    GCPolicy::get_ttl_seconds,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GCPolicy>(
                    "GCPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GCPolicy {
    fn clear(&mut self) {
        self.clear_ttl_seconds();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GCPolicy {
    fn eq(&self, other: &GCPolicy) -> bool {
        self.ttl_seconds == other.ttl_seconds &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GCPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AcctConfig {
    // message fields
    cluster_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AcctConfig {
    pub fn new() -> AcctConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AcctConfig {
        static mut instance: ::protobuf::lazy::Lazy<AcctConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AcctConfig,
        };
        unsafe {
            instance.get(|| {
                AcctConfig {
                    cluster_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string cluster_id = 1;

    pub fn clear_cluster_id(&mut self) {
        self.cluster_id.clear();
    }

    pub fn has_cluster_id(&self) -> bool {
        self.cluster_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_id(&mut self, v: ::std::string::String) {
        self.cluster_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_id<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.cluster_id.is_none() {
            self.cluster_id.set_default();
        };
        self.cluster_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_id(&mut self) -> ::std::string::String {
        self.cluster_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cluster_id<'a>(&'a self) -> &'a str {
        match self.cluster_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for AcctConfig {
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
                    let tmp = self.cluster_id.set_default();
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
        for value in self.cluster_id.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cluster_id.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<AcctConfig>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AcctConfig {
    fn new() -> AcctConfig {
        AcctConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<AcctConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cluster_id",
                    AcctConfig::has_cluster_id,
                    AcctConfig::get_cluster_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AcctConfig>(
                    "AcctConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AcctConfig {
    fn clear(&mut self) {
        self.clear_cluster_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AcctConfig {
    fn eq(&self, other: &AcctConfig) -> bool {
        self.cluster_id == other.cluster_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AcctConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PermConfig {
    // message fields
    read: ::protobuf::RepeatedField<::std::string::String>,
    write: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PermConfig {
    pub fn new() -> PermConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PermConfig {
        static mut instance: ::protobuf::lazy::Lazy<PermConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PermConfig,
        };
        unsafe {
            instance.get(|| {
                PermConfig {
                    read: ::protobuf::RepeatedField::new(),
                    write: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string read = 1;

    pub fn clear_read(&mut self) {
        self.read.clear();
    }

    // Param is passed by value, moved
    pub fn set_read(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.read = v;
    }

    // Mutable pointer to the field.
    pub fn mut_read<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.read
    }

    // Take field
    pub fn take_read(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.read, ::protobuf::RepeatedField::new())
    }

    pub fn get_read<'a>(&'a self) -> &'a [::std::string::String] {
        &self.read
    }

    // repeated string write = 2;

    pub fn clear_write(&mut self) {
        self.write.clear();
    }

    // Param is passed by value, moved
    pub fn set_write(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.write = v;
    }

    // Mutable pointer to the field.
    pub fn mut_write<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.write
    }

    // Take field
    pub fn take_write(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.write, ::protobuf::RepeatedField::new())
    }

    pub fn get_write<'a>(&'a self) -> &'a [::std::string::String] {
        &self.write
    }
}

impl ::protobuf::Message for PermConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.read));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.write));
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
        for value in self.read.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.write.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.read.iter() {
            try!(os.write_string(1, &v));
        };
        for v in self.write.iter() {
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
        ::std::any::TypeId::of::<PermConfig>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PermConfig {
    fn new() -> PermConfig {
        PermConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<PermConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "read",
                    PermConfig::get_read,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "write",
                    PermConfig::get_write,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PermConfig>(
                    "PermConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PermConfig {
    fn clear(&mut self) {
        self.clear_read();
        self.clear_write();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PermConfig {
    fn eq(&self, other: &PermConfig) -> bool {
        self.read == other.read &&
        self.write == other.write &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PermConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ZoneConfig {
    // message fields
    replica_attrs: ::protobuf::RepeatedField<Attributes>,
    range_min_bytes: ::std::option::Option<i64>,
    range_max_bytes: ::std::option::Option<i64>,
    gc: ::protobuf::SingularPtrField<GCPolicy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ZoneConfig {
    pub fn new() -> ZoneConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZoneConfig {
        static mut instance: ::protobuf::lazy::Lazy<ZoneConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZoneConfig,
        };
        unsafe {
            instance.get(|| {
                ZoneConfig {
                    replica_attrs: ::protobuf::RepeatedField::new(),
                    range_min_bytes: ::std::option::Option::None,
                    range_max_bytes: ::std::option::Option::None,
                    gc: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .cockroach.proto.Attributes replica_attrs = 1;

    pub fn clear_replica_attrs(&mut self) {
        self.replica_attrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_replica_attrs(&mut self, v: ::protobuf::RepeatedField<Attributes>) {
        self.replica_attrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_replica_attrs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Attributes> {
        &mut self.replica_attrs
    }

    // Take field
    pub fn take_replica_attrs(&mut self) -> ::protobuf::RepeatedField<Attributes> {
        ::std::mem::replace(&mut self.replica_attrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_replica_attrs<'a>(&'a self) -> &'a [Attributes] {
        &self.replica_attrs
    }

    // optional int64 range_min_bytes = 2;

    pub fn clear_range_min_bytes(&mut self) {
        self.range_min_bytes = ::std::option::Option::None;
    }

    pub fn has_range_min_bytes(&self) -> bool {
        self.range_min_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_min_bytes(&mut self, v: i64) {
        self.range_min_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_range_min_bytes<'a>(&self) -> i64 {
        self.range_min_bytes.unwrap_or(0)
    }

    // optional int64 range_max_bytes = 3;

    pub fn clear_range_max_bytes(&mut self) {
        self.range_max_bytes = ::std::option::Option::None;
    }

    pub fn has_range_max_bytes(&self) -> bool {
        self.range_max_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range_max_bytes(&mut self, v: i64) {
        self.range_max_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_range_max_bytes<'a>(&self) -> i64 {
        self.range_max_bytes.unwrap_or(0)
    }

    // optional .cockroach.proto.GCPolicy gc = 4;

    pub fn clear_gc(&mut self) {
        self.gc.clear();
    }

    pub fn has_gc(&self) -> bool {
        self.gc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc(&mut self, v: GCPolicy) {
        self.gc = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gc<'a>(&'a mut self) -> &'a mut GCPolicy {
        if self.gc.is_none() {
            self.gc.set_default();
        };
        self.gc.as_mut().unwrap()
    }

    // Take field
    pub fn take_gc(&mut self) -> GCPolicy {
        self.gc.take().unwrap_or_else(|| GCPolicy::new())
    }

    pub fn get_gc<'a>(&'a self) -> &'a GCPolicy {
        self.gc.as_ref().unwrap_or_else(|| GCPolicy::default_instance())
    }
}

impl ::protobuf::Message for ZoneConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.replica_attrs));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.range_min_bytes = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.range_max_bytes = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.gc.set_default();
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
        for value in self.replica_attrs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.range_min_bytes.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.range_max_bytes.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.gc.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.replica_attrs.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.range_min_bytes {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.range_max_bytes {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.gc.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ZoneConfig>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ZoneConfig {
    fn new() -> ZoneConfig {
        ZoneConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZoneConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "replica_attrs",
                    ZoneConfig::get_replica_attrs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "range_min_bytes",
                    ZoneConfig::has_range_min_bytes,
                    ZoneConfig::get_range_min_bytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "range_max_bytes",
                    ZoneConfig::has_range_max_bytes,
                    ZoneConfig::get_range_max_bytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "gc",
                    ZoneConfig::has_gc,
                    ZoneConfig::get_gc,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZoneConfig>(
                    "ZoneConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZoneConfig {
    fn clear(&mut self) {
        self.clear_replica_attrs();
        self.clear_range_min_bytes();
        self.clear_range_max_bytes();
        self.clear_gc();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ZoneConfig {
    fn eq(&self, other: &ZoneConfig) -> bool {
        self.replica_attrs == other.replica_attrs &&
        self.range_min_bytes == other.range_min_bytes &&
        self.range_max_bytes == other.range_max_bytes &&
        self.gc == other.gc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ZoneConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RangeTree {
    // message fields
    root_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RangeTree {
    pub fn new() -> RangeTree {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RangeTree {
        static mut instance: ::protobuf::lazy::Lazy<RangeTree> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RangeTree,
        };
        unsafe {
            instance.get(|| {
                RangeTree {
                    root_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes root_key = 1;

    pub fn clear_root_key(&mut self) {
        self.root_key.clear();
    }

    pub fn has_root_key(&self) -> bool {
        self.root_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_root_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.root_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.root_key.is_none() {
            self.root_key.set_default();
        };
        self.root_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_root_key(&mut self) -> ::std::vec::Vec<u8> {
        self.root_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_root_key<'a>(&'a self) -> &'a [u8] {
        match self.root_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RangeTree {
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
                    let tmp = self.root_key.set_default();
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
        for value in self.root_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.root_key.as_ref() {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<RangeTree>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RangeTree {
    fn new() -> RangeTree {
        RangeTree::new()
    }

    fn descriptor_static(_: ::std::option::Option<RangeTree>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "root_key",
                    RangeTree::has_root_key,
                    RangeTree::get_root_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RangeTree>(
                    "RangeTree",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RangeTree {
    fn clear(&mut self) {
        self.clear_root_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RangeTree {
    fn eq(&self, other: &RangeTree) -> bool {
        self.root_key == other.root_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RangeTree {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RangeTreeNode {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    black: ::std::option::Option<bool>,
    parent_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    left_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    right_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RangeTreeNode {
    pub fn new() -> RangeTreeNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RangeTreeNode {
        static mut instance: ::protobuf::lazy::Lazy<RangeTreeNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RangeTreeNode,
        };
        unsafe {
            instance.get(|| {
                RangeTreeNode {
                    key: ::protobuf::SingularField::none(),
                    black: ::std::option::Option::None,
                    parent_key: ::protobuf::SingularField::none(),
                    left_key: ::protobuf::SingularField::none(),
                    right_key: ::protobuf::SingularField::none(),
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

    // optional bool black = 2;

    pub fn clear_black(&mut self) {
        self.black = ::std::option::Option::None;
    }

    pub fn has_black(&self) -> bool {
        self.black.is_some()
    }

    // Param is passed by value, moved
    pub fn set_black(&mut self, v: bool) {
        self.black = ::std::option::Option::Some(v);
    }

    pub fn get_black<'a>(&self) -> bool {
        self.black.unwrap_or(false)
    }

    // optional bytes parent_key = 3;

    pub fn clear_parent_key(&mut self) {
        self.parent_key.clear();
    }

    pub fn has_parent_key(&self) -> bool {
        self.parent_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.parent_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.parent_key.is_none() {
            self.parent_key.set_default();
        };
        self.parent_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_parent_key(&mut self) -> ::std::vec::Vec<u8> {
        self.parent_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_parent_key<'a>(&'a self) -> &'a [u8] {
        match self.parent_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes left_key = 4;

    pub fn clear_left_key(&mut self) {
        self.left_key.clear();
    }

    pub fn has_left_key(&self) -> bool {
        self.left_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.left_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_left_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.left_key.is_none() {
            self.left_key.set_default();
        };
        self.left_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_left_key(&mut self) -> ::std::vec::Vec<u8> {
        self.left_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_left_key<'a>(&'a self) -> &'a [u8] {
        match self.left_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes right_key = 5;

    pub fn clear_right_key(&mut self) {
        self.right_key.clear();
    }

    pub fn has_right_key(&self) -> bool {
        self.right_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.right_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_right_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.right_key.is_none() {
            self.right_key.set_default();
        };
        self.right_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_right_key(&mut self) -> ::std::vec::Vec<u8> {
        self.right_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_right_key<'a>(&'a self) -> &'a [u8] {
        match self.right_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for RangeTreeNode {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.black = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.parent_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.left_key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.right_key.set_default();
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
        if self.black.is_some() {
            my_size += 2;
        };
        for value in self.parent_key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.left_key.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.right_key.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.black {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.parent_key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.left_key.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.right_key.as_ref() {
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
        ::std::any::TypeId::of::<RangeTreeNode>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RangeTreeNode {
    fn new() -> RangeTreeNode {
        RangeTreeNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<RangeTreeNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    RangeTreeNode::has_key,
                    RangeTreeNode::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "black",
                    RangeTreeNode::has_black,
                    RangeTreeNode::get_black,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "parent_key",
                    RangeTreeNode::has_parent_key,
                    RangeTreeNode::get_parent_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "left_key",
                    RangeTreeNode::has_left_key,
                    RangeTreeNode::get_left_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "right_key",
                    RangeTreeNode::has_right_key,
                    RangeTreeNode::get_right_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RangeTreeNode>(
                    "RangeTreeNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RangeTreeNode {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_black();
        self.clear_parent_key();
        self.clear_left_key();
        self.clear_right_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RangeTreeNode {
    fn eq(&self, other: &RangeTreeNode) -> bool {
        self.key == other.key &&
        self.black == other.black &&
        self.parent_key == other.parent_key &&
        self.left_key == other.left_key &&
        self.right_key == other.right_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RangeTreeNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Addr {
    // message fields
    network: ::protobuf::SingularField<::std::string::String>,
    address: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Addr {
    pub fn new() -> Addr {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Addr {
        static mut instance: ::protobuf::lazy::Lazy<Addr> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Addr,
        };
        unsafe {
            instance.get(|| {
                Addr {
                    network: ::protobuf::SingularField::none(),
                    address: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string network = 1;

    pub fn clear_network(&mut self) {
        self.network.clear();
    }

    pub fn has_network(&self) -> bool {
        self.network.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network(&mut self, v: ::std::string::String) {
        self.network = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_network<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.network.is_none() {
            self.network.set_default();
        };
        self.network.as_mut().unwrap()
    }

    // Take field
    pub fn take_network(&mut self) -> ::std::string::String {
        self.network.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_network<'a>(&'a self) -> &'a str {
        match self.network.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address<'a>(&'a self) -> &'a str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Addr {
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
                    let tmp = self.network.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.address.set_default();
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
        for value in self.network.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.address.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.network.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.address.as_ref() {
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
        ::std::any::TypeId::of::<Addr>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Addr {
    fn new() -> Addr {
        Addr::new()
    }

    fn descriptor_static(_: ::std::option::Option<Addr>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "network",
                    Addr::has_network,
                    Addr::get_network,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "address",
                    Addr::has_address,
                    Addr::get_address,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Addr>(
                    "Addr",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Addr {
    fn clear(&mut self) {
        self.clear_network();
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Addr {
    fn eq(&self, other: &Addr) -> bool {
        self.network == other.network &&
        self.address == other.address &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Addr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StoreCapacity {
    // message fields
    Capacity: ::std::option::Option<i64>,
    Available: ::std::option::Option<i64>,
    RangeCount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StoreCapacity {
    pub fn new() -> StoreCapacity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreCapacity {
        static mut instance: ::protobuf::lazy::Lazy<StoreCapacity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreCapacity,
        };
        unsafe {
            instance.get(|| {
                StoreCapacity {
                    Capacity: ::std::option::Option::None,
                    Available: ::std::option::Option::None,
                    RangeCount: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 Capacity = 1;

    pub fn clear_Capacity(&mut self) {
        self.Capacity = ::std::option::Option::None;
    }

    pub fn has_Capacity(&self) -> bool {
        self.Capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Capacity(&mut self, v: i64) {
        self.Capacity = ::std::option::Option::Some(v);
    }

    pub fn get_Capacity<'a>(&self) -> i64 {
        self.Capacity.unwrap_or(0)
    }

    // optional int64 Available = 2;

    pub fn clear_Available(&mut self) {
        self.Available = ::std::option::Option::None;
    }

    pub fn has_Available(&self) -> bool {
        self.Available.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Available(&mut self, v: i64) {
        self.Available = ::std::option::Option::Some(v);
    }

    pub fn get_Available<'a>(&self) -> i64 {
        self.Available.unwrap_or(0)
    }

    // optional int32 RangeCount = 3;

    pub fn clear_RangeCount(&mut self) {
        self.RangeCount = ::std::option::Option::None;
    }

    pub fn has_RangeCount(&self) -> bool {
        self.RangeCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_RangeCount(&mut self, v: i32) {
        self.RangeCount = ::std::option::Option::Some(v);
    }

    pub fn get_RangeCount<'a>(&self) -> i32 {
        self.RangeCount.unwrap_or(0)
    }
}

impl ::protobuf::Message for StoreCapacity {
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
                    self.Capacity = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.Available = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.RangeCount = ::std::option::Option::Some(tmp);
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
        for value in self.Capacity.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.Available.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.RangeCount.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Capacity {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.Available {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.RangeCount {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<StoreCapacity>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StoreCapacity {
    fn new() -> StoreCapacity {
        StoreCapacity::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreCapacity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "Capacity",
                    StoreCapacity::has_Capacity,
                    StoreCapacity::get_Capacity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "Available",
                    StoreCapacity::has_Available,
                    StoreCapacity::get_Available,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "RangeCount",
                    StoreCapacity::has_RangeCount,
                    StoreCapacity::get_RangeCount,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StoreCapacity>(
                    "StoreCapacity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreCapacity {
    fn clear(&mut self) {
        self.clear_Capacity();
        self.clear_Available();
        self.clear_RangeCount();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StoreCapacity {
    fn eq(&self, other: &StoreCapacity) -> bool {
        self.Capacity == other.Capacity &&
        self.Available == other.Available &&
        self.RangeCount == other.RangeCount &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StoreCapacity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NodeDescriptor {
    // message fields
    node_id: ::std::option::Option<i32>,
    address: ::protobuf::SingularPtrField<Addr>,
    attrs: ::protobuf::SingularPtrField<Attributes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl NodeDescriptor {
    pub fn new() -> NodeDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<NodeDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeDescriptor,
        };
        unsafe {
            instance.get(|| {
                NodeDescriptor {
                    node_id: ::std::option::Option::None,
                    address: ::protobuf::SingularPtrField::none(),
                    attrs: ::protobuf::SingularPtrField::none(),
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

    // optional .cockroach.proto.Addr address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: Addr) {
        self.address = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address<'a>(&'a mut self) -> &'a mut Addr {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> Addr {
        self.address.take().unwrap_or_else(|| Addr::new())
    }

    pub fn get_address<'a>(&'a self) -> &'a Addr {
        self.address.as_ref().unwrap_or_else(|| Addr::default_instance())
    }

    // optional .cockroach.proto.Attributes attrs = 3;

    pub fn clear_attrs(&mut self) {
        self.attrs.clear();
    }

    pub fn has_attrs(&self) -> bool {
        self.attrs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attrs(&mut self, v: Attributes) {
        self.attrs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attrs<'a>(&'a mut self) -> &'a mut Attributes {
        if self.attrs.is_none() {
            self.attrs.set_default();
        };
        self.attrs.as_mut().unwrap()
    }

    // Take field
    pub fn take_attrs(&mut self) -> Attributes {
        self.attrs.take().unwrap_or_else(|| Attributes::new())
    }

    pub fn get_attrs<'a>(&'a self) -> &'a Attributes {
        self.attrs.as_ref().unwrap_or_else(|| Attributes::default_instance())
    }
}

impl ::protobuf::Message for NodeDescriptor {
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
                    let tmp = self.address.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attrs.set_default();
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
        for value in self.node_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.address.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.attrs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.address.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.attrs.as_ref() {
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
        ::std::any::TypeId::of::<NodeDescriptor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodeDescriptor {
    fn new() -> NodeDescriptor {
        NodeDescriptor::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "node_id",
                    NodeDescriptor::has_node_id,
                    NodeDescriptor::get_node_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "address",
                    NodeDescriptor::has_address,
                    NodeDescriptor::get_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "attrs",
                    NodeDescriptor::has_attrs,
                    NodeDescriptor::get_attrs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeDescriptor>(
                    "NodeDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeDescriptor {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_address();
        self.clear_attrs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NodeDescriptor {
    fn eq(&self, other: &NodeDescriptor) -> bool {
        self.node_id == other.node_id &&
        self.address == other.address &&
        self.attrs == other.attrs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NodeDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StoreDescriptor {
    // message fields
    store_id: ::std::option::Option<i32>,
    attrs: ::protobuf::SingularPtrField<Attributes>,
    node: ::protobuf::SingularPtrField<NodeDescriptor>,
    capacity: ::protobuf::SingularPtrField<StoreCapacity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StoreDescriptor {
    pub fn new() -> StoreDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<StoreDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreDescriptor,
        };
        unsafe {
            instance.get(|| {
                StoreDescriptor {
                    store_id: ::std::option::Option::None,
                    attrs: ::protobuf::SingularPtrField::none(),
                    node: ::protobuf::SingularPtrField::none(),
                    capacity: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 store_id = 1;

    pub fn clear_store_id(&mut self) {
        self.store_id = ::std::option::Option::None;
    }

    pub fn has_store_id(&self) -> bool {
        self.store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store_id(&mut self, v: i32) {
        self.store_id = ::std::option::Option::Some(v);
    }

    pub fn get_store_id<'a>(&self) -> i32 {
        self.store_id.unwrap_or(0)
    }

    // optional .cockroach.proto.Attributes attrs = 2;

    pub fn clear_attrs(&mut self) {
        self.attrs.clear();
    }

    pub fn has_attrs(&self) -> bool {
        self.attrs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attrs(&mut self, v: Attributes) {
        self.attrs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attrs<'a>(&'a mut self) -> &'a mut Attributes {
        if self.attrs.is_none() {
            self.attrs.set_default();
        };
        self.attrs.as_mut().unwrap()
    }

    // Take field
    pub fn take_attrs(&mut self) -> Attributes {
        self.attrs.take().unwrap_or_else(|| Attributes::new())
    }

    pub fn get_attrs<'a>(&'a self) -> &'a Attributes {
        self.attrs.as_ref().unwrap_or_else(|| Attributes::default_instance())
    }

    // optional .cockroach.proto.NodeDescriptor node = 3;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: NodeDescriptor) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node<'a>(&'a mut self) -> &'a mut NodeDescriptor {
        if self.node.is_none() {
            self.node.set_default();
        };
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> NodeDescriptor {
        self.node.take().unwrap_or_else(|| NodeDescriptor::new())
    }

    pub fn get_node<'a>(&'a self) -> &'a NodeDescriptor {
        self.node.as_ref().unwrap_or_else(|| NodeDescriptor::default_instance())
    }

    // optional .cockroach.proto.StoreCapacity capacity = 4;

    pub fn clear_capacity(&mut self) {
        self.capacity.clear();
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: StoreCapacity) {
        self.capacity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capacity<'a>(&'a mut self) -> &'a mut StoreCapacity {
        if self.capacity.is_none() {
            self.capacity.set_default();
        };
        self.capacity.as_mut().unwrap()
    }

    // Take field
    pub fn take_capacity(&mut self) -> StoreCapacity {
        self.capacity.take().unwrap_or_else(|| StoreCapacity::new())
    }

    pub fn get_capacity<'a>(&'a self) -> &'a StoreCapacity {
        self.capacity.as_ref().unwrap_or_else(|| StoreCapacity::default_instance())
    }
}

impl ::protobuf::Message for StoreDescriptor {
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
                    self.store_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attrs.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.node.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.capacity.set_default();
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
        for value in self.store_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.attrs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.node.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.capacity.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.store_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.attrs.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.node.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.capacity.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<StoreDescriptor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StoreDescriptor {
    fn new() -> StoreDescriptor {
        StoreDescriptor::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "store_id",
                    StoreDescriptor::has_store_id,
                    StoreDescriptor::get_store_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "attrs",
                    StoreDescriptor::has_attrs,
                    StoreDescriptor::get_attrs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "node",
                    StoreDescriptor::has_node,
                    StoreDescriptor::get_node,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "capacity",
                    StoreDescriptor::has_capacity,
                    StoreDescriptor::get_capacity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StoreDescriptor>(
                    "StoreDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreDescriptor {
    fn clear(&mut self) {
        self.clear_store_id();
        self.clear_attrs();
        self.clear_node();
        self.clear_capacity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StoreDescriptor {
    fn eq(&self, other: &StoreDescriptor) -> bool {
        self.store_id == other.store_id &&
        self.attrs == other.attrs &&
        self.node == other.node &&
        self.capacity == other.capacity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StoreDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f,
    0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x1b, 0x0a, 0x0a, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x0d, 0x0a,
    0x05, 0x61, 0x74, 0x74, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x22, 0x58, 0x0a, 0x07,
    0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x12, 0x0f, 0x0a, 0x07, 0x6e, 0x6f, 0x64, 0x65, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x74, 0x6f, 0x72,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x2a, 0x0a, 0x05, 0x61, 0x74,
    0x74, 0x72, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x41, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x22, 0x72, 0x0a, 0x0f, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x44,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x61, 0x66,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a,
    0x07, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x2a,
    0x0a, 0x08, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x18, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x22, 0x1f, 0x0a, 0x08, 0x47, 0x43,
    0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x12, 0x13, 0x0a, 0x0b, 0x74, 0x74, 0x6c, 0x5f, 0x73, 0x65,
    0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0x20, 0x0a, 0x0a, 0x41,
    0x63, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x29, 0x0a,
    0x0a, 0x50, 0x65, 0x72, 0x6d, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x0c, 0x0a, 0x04, 0x72,
    0x65, 0x61, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x77, 0x72, 0x69,
    0x74, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x22, 0x99, 0x01, 0x0a, 0x0a, 0x5a, 0x6f, 0x6e,
    0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x32, 0x0a, 0x0d, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x5f, 0x61, 0x74, 0x74, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b,
    0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x17, 0x0a, 0x0f, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x69, 0x6e, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x03, 0x12, 0x17, 0x0a, 0x0f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x61,
    0x78, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x12, 0x25, 0x0a,
    0x02, 0x67, 0x63, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x43, 0x50, 0x6f,
    0x6c, 0x69, 0x63, 0x79, 0x22, 0x1d, 0x0a, 0x09, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x72, 0x65,
    0x65, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x6f, 0x6f, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x22, 0x64, 0x0a, 0x0d, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x72, 0x65, 0x65,
    0x4e, 0x6f, 0x64, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x12, 0x0d, 0x0a, 0x05, 0x62, 0x6c, 0x61, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x12, 0x0a, 0x0a, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x65, 0x66, 0x74, 0x5f, 0x6b, 0x65, 0x79,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x69, 0x67, 0x68, 0x74, 0x5f,
    0x6b, 0x65, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x28, 0x0a, 0x04, 0x41, 0x64, 0x64,
    0x72, 0x12, 0x0f, 0x0a, 0x07, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x22, 0x48, 0x0a, 0x0d, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x43, 0x61, 0x70, 0x61,
    0x63, 0x69, 0x74, 0x79, 0x12, 0x10, 0x0a, 0x08, 0x43, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09, 0x41, 0x76, 0x61, 0x69, 0x6c, 0x61,
    0x62, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x12, 0x12, 0x0a, 0x0a, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x22, 0x75, 0x0a,
    0x0e, 0x4e, 0x6f, 0x64, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x12,
    0x0f, 0x0a, 0x07, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x26, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x15, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x12, 0x2a, 0x0a, 0x05, 0x61, 0x74, 0x74, 0x72,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x73, 0x22, 0xb0, 0x01, 0x0a, 0x0f, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x44, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x74, 0x6f, 0x72,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x2a, 0x0a, 0x05, 0x61, 0x74,
    0x74, 0x72, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x63, 0x6b,
    0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x41, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x12, 0x2d, 0x0a, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x30, 0x0a, 0x08, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74,
    0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f,
    0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x43,
    0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x42, 0x07, 0x5a, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x4a, 0xe8, 0x2b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x7c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x08, 0x17, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x04, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x07, 0x12, 0x03, 0x04, 0x14, 0x1b, 0x0a, 0x80, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x08, 0x00, 0x0a, 0x01, 0x1a, 0x74, 0x20, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x69,
    0x6e, 0x67, 0x0a, 0x20, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x74, 0x6f, 0x70, 0x6f, 0x6c, 0x6f,
    0x67, 0x79, 0x2c, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x20, 0x63, 0x61, 0x70, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x08, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x09, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0xf7, 0x01, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x15, 0x01, 0x1a, 0xea, 0x01, 0x20, 0x52, 0x65, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20,
    0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x62, 0x79, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x49, 0x44, 0x20, 0x28, 0x63, 0x6f, 0x72,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x0a, 0x20, 0x20,
    0x68, 0x6f, 0x73, 0x74, 0x3a, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x76, 0x69, 0x61, 0x20, 0x6c, 0x6f,
    0x6f, 0x6b, 0x75, 0x70, 0x20, 0x6f, 0x6e, 0x20, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x20, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x29, 0x2c, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x49,
    0x44, 0x20, 0x28, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x20, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x29, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x2e, 0x20, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x52, 0x61,
    0x6e, 0x67, 0x65, 0x0a, 0x20, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x20, 0x72, 0x65, 0x63,
    0x6f, 0x72, 0x64, 0x73, 0x20, 0x28, 0x6d, 0x65, 0x74, 0x61, 0x31, 0x2c, 0x20, 0x6d, 0x65, 0x74,
    0x61, 0x32, 0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x11, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x11, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x1c, 0x1d, 0x0a, 0x36, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x14, 0x02, 0x31, 0x1a, 0x29, 0x20, 0x43, 0x6f, 0x6d, 0x62, 0x69,
    0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x26,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x14, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x14, 0x0b, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x27, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x14, 0x2f, 0x30, 0x0a, 0xcd, 0x01, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x1a, 0x00, 0x25, 0x01, 0x1a, 0xc0, 0x01, 0x20, 0x52, 0x61, 0x6e, 0x67,
    0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x61, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x20, 0x20, 0x41, 0x20, 0x72, 0x61, 0x6e,
    0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x20,
    0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69,
    0x76, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x61, 0x20,
    0x6e, 0x6f, 0x6e, 0x2d, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x20, 0x65, 0x6e,
    0x64, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x0a, 0x20, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x6c,
    0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20,
    0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x1b, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x1b, 0x1c, 0x0a, 0x4e, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x1f, 0x1a, 0x41, 0x20, 0x53, 0x74, 0x61, 0x72,
    0x74, 0x4b, 0x65, 0x79, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73,
    0x74, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d, 0x61, 0x79, 0x20,
    0x62, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1d, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x1d, 0x1d, 0x1e, 0x0a, 0xb0, 0x01, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x21,
    0x02, 0x1d, 0x1a, 0xa2, 0x01, 0x20, 0x45, 0x6e, 0x64, 0x4b, 0x65, 0x79, 0x20, 0x6d, 0x61, 0x72,
    0x6b, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x27, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62,
    0x6c, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x2e, 0x20, 0x20, 0x45, 0x6e, 0x64, 0x4b, 0x65, 0x79,
    0x20, 0x69, 0x74, 0x73, 0x65, 0x6c, 0x66, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x0a, 0x20,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x2d, 0x20, 0x69, 0x74, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65,
    0x6c, 0x79, 0x0a, 0x20, 0x20, 0x73, 0x75, 0x62, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x74, 0x20,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x21, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x1b, 0x1c, 0x0a,
    0x88, 0x01, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x02, 0x31, 0x1a, 0x7b, 0x20,
    0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20,
    0x6f, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x0a, 0x20, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x62, 0x65,
    0x69, 0x6e, 0x67, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x65, 0x72,
    0x6d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x24, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x24, 0x24, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24,
    0x2f, 0x30, 0x0a, 0x85, 0x02, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2c, 0x00, 0x31, 0x01, 0x1a,
    0xf8, 0x01, 0x20, 0x47, 0x43, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69,
    0x6e, 0x65, 0x73, 0x20, 0x67, 0x61, 0x72, 0x62, 0x61, 0x67, 0x65, 0x20, 0x63, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x69, 0x65, 0x73, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x4d,
    0x56, 0x43, 0x43, 0x0a, 0x20, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x7a, 0x6f, 0x6e, 0x65, 0x2e, 0x0a, 0x20, 0x20, 0x54, 0x4f,
    0x44, 0x4f, 0x28, 0x73, 0x70, 0x65, 0x6e, 0x63, 0x65, 0x72, 0x29, 0x3a, 0x20, 0x66, 0x6c, 0x65,
    0x73, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x69,
    0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x6e,
    0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x61, 0x73, 0x20, 0x77, 0x65, 0x6c, 0x6c, 0x20, 0x61, 0x73, 0x20, 0x77,
    0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x27, 0x73, 0x20, 0x61,
    0x6e, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x62,
    0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x73, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x54, 0x54, 0x4c, 0x20, 0x6f, 0x72,
    0x20, 0x61, 0x20, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0xca, 0x01, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x30, 0x02, 0x21, 0x1a, 0xbc, 0x01, 0x20, 0x54, 0x54, 0x4c, 0x53, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x69,
    0x74, 0x27, 0x73, 0x0a, 0x20, 0x20, 0x67, 0x61, 0x72, 0x62, 0x61, 0x67, 0x65, 0x20, 0x63, 0x6f,
    0x6c, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x2e, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6c,
    0x64, 0x65, 0x72, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x67, 0x61, 0x72, 0x62, 0x61,
    0x67, 0x65, 0x0a, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x2e, 0x20,
    0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x3c, 0x3d, 0x30, 0x20, 0x6d,
    0x65, 0x61, 0x6e, 0x20, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x65, 0x76, 0x65, 0x72, 0x20, 0x47, 0x43, 0x27,
    0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x11, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x1f, 0x20, 0x0a, 0x38, 0x0a, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x34, 0x00, 0x36, 0x01, 0x1a, 0x2c, 0x20, 0x41, 0x63, 0x63, 0x74, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x20, 0x68, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x34, 0x08,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x35, 0x02, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x35, 0x1f, 0x20, 0x0a, 0x54, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x39, 0x00,
    0x3e, 0x01, 0x1a, 0x48, 0x20, 0x50, 0x65, 0x72, 0x6d, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20,
    0x68, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x61, 0x64, 0x2f,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x41, 0x43, 0x4c, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x39, 0x08, 0x12, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x3b, 0x02, 0x1b, 0x1a, 0x28, 0x20, 0x41, 0x43, 0x4c, 0x20, 0x6c, 0x69, 0x73, 0x74,
    0x73, 0x20, 0x75, 0x73, 0x65, 0x72, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x72, 0x65, 0x61,
    0x64, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x3b, 0x19, 0x1a, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12,
    0x03, 0x3d, 0x02, 0x1c, 0x1a, 0x29, 0x20, 0x41, 0x43, 0x4c, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x73,
    0x20, 0x75, 0x73, 0x65, 0x72, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x77, 0x72, 0x69, 0x74,
    0x65, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x3d, 0x1a, 0x1b, 0x0a, 0x54, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x41,
    0x00, 0x4b, 0x01, 0x1a, 0x48, 0x20, 0x5a, 0x6f, 0x6e, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x20, 0x68, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x65,
    0x64, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x4b, 0x56, 0x20, 0x70, 0x61, 0x69, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x41, 0x08, 0x12, 0x0a, 0xd6, 0x01, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x45, 0x02, 0x39, 0x1a, 0xc8, 0x01, 0x20, 0x52, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x41, 0x74, 0x74, 0x72, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x73, 0x6c, 0x69,
    0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73,
    0x2c, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x73, 0x0a, 0x20, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63, 0x68,
    0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x7a, 0x6f, 0x6e, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20,
    0x69, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72,
    0x65, 0x64, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x20, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x41,
    0x74, 0x74, 0x72, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72,
    0x79, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x45, 0x0b, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45, 0x27, 0x34, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x45, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x01, 0x12, 0x03, 0x46, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x46, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x11,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x46, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x47, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x47, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x47, 0x11, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x47, 0x23, 0x24, 0x0a, 0x96, 0x01, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x4a,
    0x02, 0x2c, 0x1a, 0x88, 0x01, 0x20, 0x49, 0x66, 0x20, 0x47, 0x43, 0x20, 0x70, 0x6f, 0x6c, 0x69,
    0x63, 0x79, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x75,
    0x73, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x68, 0x69, 0x67,
    0x68, 0x65, 0x73, 0x74, 0x2c, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x6e, 0x75, 0x6c, 0x6c, 0x20, 0x70,
    0x6f, 0x6c, 0x69, 0x63, 0x79, 0x0a, 0x20, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x7a,
    0x6f, 0x6e, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x68, 0x69, 0x65, 0x72, 0x61,
    0x72, 0x63, 0x68, 0x79, 0x2c, 0x20, 0x75, 0x70, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x69,
    0x66, 0x20, 0x6e, 0x65, 0x63, 0x65, 0x73, 0x73, 0x61, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4a, 0x0b, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x4a, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x4a, 0x2a, 0x2b, 0x0a, 0x47, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4e, 0x00, 0x50,
    0x01, 0x1a, 0x3b, 0x20, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x72, 0x65, 0x65, 0x20, 0x68, 0x6f,
    0x6c, 0x64, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x6f, 0x74, 0x20, 0x6e, 0x6f, 0x64,
    0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x74, 0x72, 0x65, 0x65, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x00, 0x12, 0x03, 0x4f, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x4f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x11,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x1c, 0x1d, 0x0a,
    0x73, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x53, 0x00, 0x5e, 0x01, 0x1a, 0x67, 0x20, 0x52, 0x61,
    0x6e, 0x67, 0x65, 0x54, 0x72, 0x65, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x20, 0x68, 0x6f, 0x6c, 0x64,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x6e, 0x6f, 0x64,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x65, 0x64, 0x2d, 0x42, 0x6c, 0x61,
    0x63, 0x6b, 0x20, 0x54, 0x72, 0x65, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x72, 0x65, 0x66,
    0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x53, 0x08, 0x15,
    0x0a, 0xbc, 0x01, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x57, 0x02, 0x19, 0x1a, 0xae,
    0x01, 0x20, 0x55, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x27, 0x73, 0x20, 0x6d, 0x61, 0x72, 0x73, 0x68, 0x61, 0x6c, 0x65, 0x72, 0x20, 0x69, 0x73,
    0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x61, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x62, 0x75, 0x67, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x0a, 0x20, 0x20,
    0x6d, 0x61, 0x72, 0x73, 0x68, 0x61, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x73, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x65, 0x66, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4b, 0x65, 0x79, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x61, 0x73, 0x0a, 0x20, 0x20, 0x6e, 0x75, 0x6c, 0x6c, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x63, 0x74, 0x6c, 0x79, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x57, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x57, 0x17, 0x18, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x03, 0x59, 0x02, 0x1a, 0x1a, 0x27, 0x20, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20,
    0x62, 0x6c, 0x61, 0x63, 0x6b, 0x20, 0x69, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x72,
    0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x59, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x59, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x59, 0x18, 0x19, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x5b,
    0x02, 0x20, 0x1a, 0x33, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x75, 0x6c, 0x6c, 0x2c, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x6f, 0x74,
    0x20, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x5b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x5b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x11,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5b, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x5c, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x5c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x5c, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x5c, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x5d, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x03, 0x5d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5d, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x5d, 0x1d, 0x1e, 0x0a, 0x9a, 0x01, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x04, 0x62, 0x00, 0x65, 0x01, 0x1a, 0x8d, 0x01, 0x20, 0x41, 0x64, 0x64, 0x72, 0x20, 0x68,
    0x6f, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x69, 0x6d,
    0x69, 0x6c, 0x61, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x65, 0x74, 0x2e, 0x41, 0x64, 0x64, 0x72,
    0x0a, 0x20, 0x20, 0x68, 0x6f, 0x77, 0x65, 0x76, 0x65, 0x72, 0x20, 0x53, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6d,
    0x70, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x62,
    0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x63, 0x02, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x63, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12,
    0x03, 0x64, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x64,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x64, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x64, 0x12, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x1c, 0x1d, 0x0a, 0x4f, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x04, 0x68, 0x00, 0x6c, 0x01, 0x1a, 0x43, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x43, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x20, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x73, 0x74, 0x6f,
    0x72, 0x61, 0x67, 0x65, 0x20, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x68, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x00, 0x12, 0x03, 0x69, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x69, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x69,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x11, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x1c, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x6a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x6a, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x6a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x6b, 0x02, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x6b, 0x1e, 0x1f, 0x0a, 0x4d, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04,
    0x6f, 0x00, 0x73, 0x01, 0x1a, 0x41, 0x20, 0x4e, 0x6f, 0x64, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x20, 0x68, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x64, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x70, 0x68, 0x79, 0x73,
    0x69, 0x63, 0x61, 0x6c, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x74, 0x6f, 0x70,
    0x6f, 0x6c, 0x6f, 0x67, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03,
    0x6f, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x70, 0x02, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01,
    0x12, 0x03, 0x71, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x71, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x71, 0x0b,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x21, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x71, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x72, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x72, 0x0b, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x72, 0x27, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x72,
    0x2f, 0x30, 0x0a, 0x77, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x77, 0x00, 0x7c, 0x01, 0x1a, 0x6b,
    0x20, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72,
    0x20, 0x68, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x69,
    0x6e, 0x67, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x73, 0x2c, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x20, 0x20, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x20, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0c, 0x01, 0x12, 0x03, 0x77, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12,
    0x03, 0x78, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x78,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x78, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x11, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x78, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x79, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x79, 0x0b, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x79, 0x27, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x79, 0x2f,
    0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x7a, 0x02, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7a, 0x0b, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x7a, 0x2b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x7a, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x03,
    0x7b, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x06, 0x12, 0x03, 0x7b, 0x0b, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7b, 0x2a, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7b, 0x35, 0x36,
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
