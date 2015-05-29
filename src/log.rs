// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct LogEntry {
    // message fields
    severity: ::std::option::Option<i32>,
    time: ::std::option::Option<i64>,
    thread_id: ::std::option::Option<i32>,
    file: ::protobuf::SingularField<::std::string::String>,
    line: ::std::option::Option<i32>,
    format: ::protobuf::SingularField<::std::string::String>,
    args: ::protobuf::RepeatedField<LogEntry_Arg>,
    node_id: ::std::option::Option<i32>,
    store_id: ::std::option::Option<i32>,
    raft_id: ::std::option::Option<i64>,
    method: ::std::option::Option<i32>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    stacks: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl LogEntry {
    pub fn new() -> LogEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogEntry {
        static mut instance: ::protobuf::lazy::Lazy<LogEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogEntry,
        };
        unsafe {
            instance.get(|| {
                LogEntry {
                    severity: ::std::option::Option::None,
                    time: ::std::option::Option::None,
                    thread_id: ::std::option::Option::None,
                    file: ::protobuf::SingularField::none(),
                    line: ::std::option::Option::None,
                    format: ::protobuf::SingularField::none(),
                    args: ::protobuf::RepeatedField::new(),
                    node_id: ::std::option::Option::None,
                    store_id: ::std::option::Option::None,
                    raft_id: ::std::option::Option::None,
                    method: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    stacks: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 severity = 1;

    pub fn clear_severity(&mut self) {
        self.severity = ::std::option::Option::None;
    }

    pub fn has_severity(&self) -> bool {
        self.severity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_severity(&mut self, v: i32) {
        self.severity = ::std::option::Option::Some(v);
    }

    pub fn get_severity<'a>(&self) -> i32 {
        self.severity.unwrap_or(0)
    }

    // optional int64 time = 2;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i64) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time<'a>(&self) -> i64 {
        self.time.unwrap_or(0)
    }

    // optional int32 thread_id = 3;

    pub fn clear_thread_id(&mut self) {
        self.thread_id = ::std::option::Option::None;
    }

    pub fn has_thread_id(&self) -> bool {
        self.thread_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_thread_id(&mut self, v: i32) {
        self.thread_id = ::std::option::Option::Some(v);
    }

    pub fn get_thread_id<'a>(&self) -> i32 {
        self.thread_id.unwrap_or(0)
    }

    // optional string file = 4;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    pub fn has_file(&self) -> bool {
        self.file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ::std::string::String) {
        self.file = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.file.is_none() {
            self.file.set_default();
        };
        self.file.as_mut().unwrap()
    }

    // Take field
    pub fn take_file(&mut self) -> ::std::string::String {
        self.file.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file<'a>(&'a self) -> &'a str {
        match self.file.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 line = 5;

    pub fn clear_line(&mut self) {
        self.line = ::std::option::Option::None;
    }

    pub fn has_line(&self) -> bool {
        self.line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: i32) {
        self.line = ::std::option::Option::Some(v);
    }

    pub fn get_line<'a>(&self) -> i32 {
        self.line.unwrap_or(0)
    }

    // optional string format = 6;

    pub fn clear_format(&mut self) {
        self.format.clear();
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: ::std::string::String) {
        self.format = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_format<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.format.is_none() {
            self.format.set_default();
        };
        self.format.as_mut().unwrap()
    }

    // Take field
    pub fn take_format(&mut self) -> ::std::string::String {
        self.format.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_format<'a>(&'a self) -> &'a str {
        match self.format.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .cockroach.proto.LogEntry.Arg args = 7;

    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    // Param is passed by value, moved
    pub fn set_args(&mut self, v: ::protobuf::RepeatedField<LogEntry_Arg>) {
        self.args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_args<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<LogEntry_Arg> {
        &mut self.args
    }

    // Take field
    pub fn take_args(&mut self) -> ::protobuf::RepeatedField<LogEntry_Arg> {
        ::std::mem::replace(&mut self.args, ::protobuf::RepeatedField::new())
    }

    pub fn get_args<'a>(&'a self) -> &'a [LogEntry_Arg] {
        &self.args
    }

    // optional int32 node_id = 8;

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

    // optional int32 store_id = 9;

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

    // optional int64 raft_id = 10;

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

    // optional int32 method = 11;

    pub fn clear_method(&mut self) {
        self.method = ::std::option::Option::None;
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: i32) {
        self.method = ::std::option::Option::Some(v);
    }

    pub fn get_method<'a>(&self) -> i32 {
        self.method.unwrap_or(0)
    }

    // optional bytes key = 12;

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

    // optional bytes stacks = 13;

    pub fn clear_stacks(&mut self) {
        self.stacks.clear();
    }

    pub fn has_stacks(&self) -> bool {
        self.stacks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stacks(&mut self, v: ::std::vec::Vec<u8>) {
        self.stacks = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stacks<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.stacks.is_none() {
            self.stacks.set_default();
        };
        self.stacks.as_mut().unwrap()
    }

    // Take field
    pub fn take_stacks(&mut self) -> ::std::vec::Vec<u8> {
        self.stacks.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_stacks<'a>(&'a self) -> &'a [u8] {
        match self.stacks.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for LogEntry {
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
                    self.severity = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.thread_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.file.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.line = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.format.set_default();
                    try!(is.read_string_into(tmp))
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.args));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.node_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.store_id = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.raft_id = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.method = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.stacks.set_default();
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
        for value in self.severity.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.time.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.thread_id.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.file.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.line.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.format.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in self.args.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.node_id.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.store_id.iter() {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.raft_id.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.method.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(12, &value);
        };
        for value in self.stacks.iter() {
            my_size += ::protobuf::rt::bytes_size(13, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.severity {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.time {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.thread_id {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.file.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.line {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.format.as_ref() {
            try!(os.write_string(6, &v));
        };
        for v in self.args.iter() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.node_id {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.store_id {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.raft_id {
            try!(os.write_int64(10, v));
        };
        if let Some(v) = self.method {
            try!(os.write_int32(11, v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(12, &v));
        };
        if let Some(v) = self.stacks.as_ref() {
            try!(os.write_bytes(13, &v));
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
        ::std::any::TypeId::of::<LogEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogEntry {
    fn new() -> LogEntry {
        LogEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "severity",
                    LogEntry::has_severity,
                    LogEntry::get_severity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "time",
                    LogEntry::has_time,
                    LogEntry::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "thread_id",
                    LogEntry::has_thread_id,
                    LogEntry::get_thread_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "file",
                    LogEntry::has_file,
                    LogEntry::get_file,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "line",
                    LogEntry::has_line,
                    LogEntry::get_line,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "format",
                    LogEntry::has_format,
                    LogEntry::get_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "args",
                    LogEntry::get_args,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "node_id",
                    LogEntry::has_node_id,
                    LogEntry::get_node_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "store_id",
                    LogEntry::has_store_id,
                    LogEntry::get_store_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "raft_id",
                    LogEntry::has_raft_id,
                    LogEntry::get_raft_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "method",
                    LogEntry::has_method,
                    LogEntry::get_method,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    LogEntry::has_key,
                    LogEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "stacks",
                    LogEntry::has_stacks,
                    LogEntry::get_stacks,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogEntry>(
                    "LogEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogEntry {
    fn clear(&mut self) {
        self.clear_severity();
        self.clear_time();
        self.clear_thread_id();
        self.clear_file();
        self.clear_line();
        self.clear_format();
        self.clear_args();
        self.clear_node_id();
        self.clear_store_id();
        self.clear_raft_id();
        self.clear_method();
        self.clear_key();
        self.clear_stacks();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.severity == other.severity &&
        self.time == other.time &&
        self.thread_id == other.thread_id &&
        self.file == other.file &&
        self.line == other.line &&
        self.format == other.format &&
        self.args == other.args &&
        self.node_id == other.node_id &&
        self.store_id == other.store_id &&
        self.raft_id == other.raft_id &&
        self.method == other.method &&
        self.key == other.key &&
        self.stacks == other.stacks &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LogEntry_Arg {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    str: ::protobuf::SingularField<::std::string::String>,
    json: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl LogEntry_Arg {
    pub fn new() -> LogEntry_Arg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LogEntry_Arg {
        static mut instance: ::protobuf::lazy::Lazy<LogEntry_Arg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LogEntry_Arg,
        };
        unsafe {
            instance.get(|| {
                LogEntry_Arg {
                    field_type: ::protobuf::SingularField::none(),
                    str: ::protobuf::SingularField::none(),
                    json: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type<'a>(&'a self) -> &'a str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string str = 2;

    pub fn clear_str(&mut self) {
        self.str.clear();
    }

    pub fn has_str(&self) -> bool {
        self.str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_str(&mut self, v: ::std::string::String) {
        self.str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_str<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.str.is_none() {
            self.str.set_default();
        };
        self.str.as_mut().unwrap()
    }

    // Take field
    pub fn take_str(&mut self) -> ::std::string::String {
        self.str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_str<'a>(&'a self) -> &'a str {
        match self.str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes json = 3;

    pub fn clear_json(&mut self) {
        self.json.clear();
    }

    pub fn has_json(&self) -> bool {
        self.json.is_some()
    }

    // Param is passed by value, moved
    pub fn set_json(&mut self, v: ::std::vec::Vec<u8>) {
        self.json = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.json.is_none() {
            self.json.set_default();
        };
        self.json.as_mut().unwrap()
    }

    // Take field
    pub fn take_json(&mut self) -> ::std::vec::Vec<u8> {
        self.json.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_json<'a>(&'a self) -> &'a [u8] {
        match self.json.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for LogEntry_Arg {
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
                    let tmp = self.field_type.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.str.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.json.set_default();
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
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.str.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.json.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.str.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.json.as_ref() {
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
        ::std::any::TypeId::of::<LogEntry_Arg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LogEntry_Arg {
    fn new() -> LogEntry_Arg {
        LogEntry_Arg::new()
    }

    fn descriptor_static(_: ::std::option::Option<LogEntry_Arg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "field_type",
                    LogEntry_Arg::has_field_type,
                    LogEntry_Arg::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "str",
                    LogEntry_Arg::has_str,
                    LogEntry_Arg::get_str,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "json",
                    LogEntry_Arg::has_json,
                    LogEntry_Arg::get_json,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LogEntry_Arg>(
                    "LogEntry_Arg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LogEntry_Arg {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_str();
        self.clear_json();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LogEntry_Arg {
    fn eq(&self, other: &LogEntry_Arg) -> bool {
        self.field_type == other.field_type &&
        self.str == other.str &&
        self.json == other.json &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LogEntry_Arg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x6c, 0x6f, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x63, 0x6f, 0x63,
    0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa7, 0x02, 0x0a,
    0x08, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x65, 0x76,
    0x65, 0x72, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x68, 0x72,
    0x65, 0x61, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04,
    0x66, 0x69, 0x6c, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6c, 0x69,
    0x6e, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0e, 0x0a, 0x06, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x12, 0x2b, 0x0a, 0x04, 0x61, 0x72, 0x67, 0x73,
    0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61,
    0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x2e, 0x41, 0x72, 0x67, 0x12, 0x0f, 0x0a, 0x07, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f,
    0x69, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x03, 0x12, 0x0e, 0x0a, 0x06, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x73,
    0x18, 0x0d, 0x20, 0x01, 0x28, 0x0c, 0x1a, 0x2e, 0x0a, 0x03, 0x41, 0x72, 0x67, 0x12, 0x0c, 0x0a,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x73,
    0x74, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6a, 0x73, 0x6f, 0x6e,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x42, 0x07, 0x5a, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x4a,
    0xa3, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x24, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x08, 0x17, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x04, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x07, 0x12, 0x03, 0x04, 0x14, 0x1b, 0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00,
    0x24, 0x01, 0x1a, 0x32, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x63, 0x6b, 0x72, 0x6f, 0x61, 0x63, 0x68, 0x20,
    0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65,
    0x6e, 0x74, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07,
    0x08, 0x10, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x09, 0x02, 0x0e, 0x03,
    0x1a, 0x17, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x61, 0x72,
    0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0a, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x0a, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0a, 0x14, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0a, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0b, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0b, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0b, 0x14, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0b, 0x1a, 0x1b, 0x0a, 0x2e, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0d, 0x04, 0x1c, 0x1a, 0x1f, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x6a, 0x73, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x0d, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0d, 0x13, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0d, 0x1a, 0x1b, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10,
    0x02, 0x1e, 0x1a, 0x17, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x73, 0x65, 0x76, 0x65, 0x72, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x10, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x10, 0x1c, 0x1d, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x1a,
    0x1a, 0x30, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x2c, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65,
    0x64, 0x20, 0x69, 0x6e, 0x20, 0x6e, 0x61, 0x6e, 0x6f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73,
    0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x70, 0x6f, 0x63, 0x68,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x18, 0x19, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x14, 0x02, 0x1f, 0x1a, 0x1f, 0x20, 0x54, 0x68, 0x72, 0x65, 0x61, 0x64,
    0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x72,
    0x6f, 0x75, 0x74, 0x69, 0x6e, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x14, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14,
    0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x14, 0x1d, 0x1e,
    0x0a, 0x32, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x16, 0x02, 0x1b, 0x1a, 0x25, 0x20,
    0x46, 0x69, 0x6c, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x16,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x16, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x16, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x16, 0x19, 0x1a, 0x0a, 0x3a, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x18, 0x02, 0x1a, 0x1a, 0x2d, 0x20, 0x4c, 0x69, 0x6e, 0x65,
    0x20, 0x69, 0x6e, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x67,
    0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x18, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x18,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x18, 0x18, 0x19,
    0x0a, 0x22, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x02, 0x1d, 0x1a, 0x15, 0x20,
    0x4c, 0x6f, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1a,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1a, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1a, 0x12, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06,
    0x12, 0x03, 0x1b, 0x0b, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x1b, 0x29, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1b, 0x30,
    0x31, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x1d, 0x02, 0x1d, 0x1a, 0x36,
    0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65,
    0x74, 0x65, 0x72, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62,
    0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65,
    0x6e, 0x74, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x1d,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1d, 0x1b, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x1e, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x1e, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x1e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x1f, 0x02, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x1f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1f, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x1f, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a,
    0x12, 0x03, 0x20, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x20, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x20, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x20, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x20, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x21, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x05, 0x12, 0x03, 0x21, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x21, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x21,
    0x17, 0x19, 0x0a, 0x29, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x23, 0x02, 0x1d, 0x1a,
    0x1c, 0x20, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x20, 0x74, 0x72, 0x61, 0x63, 0x65, 0x73, 0x20, 0x69,
    0x66, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0c, 0x01, 0x12, 0x03, 0x23, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03,
    0x12, 0x03, 0x23, 0x1a, 0x1c,
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
