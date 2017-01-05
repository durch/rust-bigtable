use protobuf;
use serde_json;
use std;

use protobuf::Message;
use protobuf::descriptor::FieldDescriptorProto_Type;
use protobuf::reflect::FieldDescriptor;
use serde_json::Value;

pub fn proto_to_json(message: &Message) -> Value {
    let mut map = serde_json::Map::new();

    for field in message.descriptor().fields() {
        match field_to_json(message, field) {
            Some(x) => {
                map.insert(field.name().to_string(), x)
            },
            None => continue
        };
    }
    Value::Object(map)
}

fn field_to_json(m: &Message, fd: &FieldDescriptor) -> Option<Value> {
    if fd.is_repeated() {
        match fd.len_field(m) {
            0 => None,
            _ => Some(repeated_field_to_json(m, fd)),
        }
    } else if fd.has_field(m) {
        Some(singular_field_to_json(m, fd))
    } else {
        None
    }
}

// Extracts a Vec<T> from a repeated proto field.
// Most field types already have a function for extracting a Vec<T> directly,
// however a few (e.g. Message) only have "len" and "get_item(i)" functions.
// This function uses the len & get_item functions in order to create vector.
#[allow(dead_code)]
fn extract_vec_shim<'a, T>(
    message: &'a Message,
    get_size_fn: &Fn(&Message) -> usize,
    extract_one_fn: &Fn(&'a Message, usize) -> &'a T) -> Vec<&'a T> {

    let size = get_size_fn(message);
    let mut v = Vec::new();
    for i in 0..size {
        v.push(extract_one_fn(message, i));
    }
    v
}

fn repeated_to_serde_array<T>(
    message: &Message,
    extract_fn: &Fn(&Message) -> Vec<T>,
    convert_one_fn: &Fn(T) -> serde_json::Value) -> Value {

    serde_json::Value::Array(
        extract_fn(message).into_iter().map(convert_one_fn).collect())
}

fn repeated_field_to_json(message: &Message,
                          field_descriptor: &FieldDescriptor) -> Value {

    match field_descriptor.proto().get_field_type() {
        FieldDescriptorProto_Type::TYPE_DOUBLE => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_f64(m).to_vec(),
                &Value::F64)
        },
        FieldDescriptorProto_Type::TYPE_FLOAT => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_f32(m).to_vec(),
                &|v| Value::F64(v as f64))
        },
        FieldDescriptorProto_Type::TYPE_INT32 |
        FieldDescriptorProto_Type::TYPE_SINT32 |
        FieldDescriptorProto_Type::TYPE_SFIXED32 => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_i32(m).to_vec(),
                &|v| Value::I64(v as i64))
        },
        FieldDescriptorProto_Type::TYPE_INT64 |
        FieldDescriptorProto_Type::TYPE_SINT64 |
        FieldDescriptorProto_Type::TYPE_SFIXED64 => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_i64(m).to_vec(),
                &Value::I64)
        },
        FieldDescriptorProto_Type::TYPE_UINT32 |
        FieldDescriptorProto_Type::TYPE_FIXED32 => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_u32(m).to_vec(),
                &|v| Value::U64(v as u64))
        },
        FieldDescriptorProto_Type::TYPE_UINT64 |
        FieldDescriptorProto_Type::TYPE_FIXED64 => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_u64(m).to_vec(),
                &Value::U64)
        },
        FieldDescriptorProto_Type::TYPE_BOOL => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_bool(m).to_vec(),
                &Value::Bool)
        },
        FieldDescriptorProto_Type::TYPE_STRING => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_str(m).to_vec(),
                &Value::String)
        },
        FieldDescriptorProto_Type::TYPE_BYTES => {
            repeated_to_serde_array(
                message,
                &|m| field_descriptor.get_rep_bytes(m).to_vec(),
                &|v| Value::String(std::str::from_utf8(&v).unwrap().to_string()))
        },
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            let mut sub_messages: Vec<&protobuf::Message> = Vec::new();
            for i in 0..field_descriptor.len_field(message) {
                sub_messages.push(
                    field_descriptor.get_rep_message_item(message, i));
            }

            Value::Array(sub_messages.into_iter().map(
                |sub_message| proto_to_json(sub_message)).collect())

            /* TODO: why doesn't this work?
            return repeated_to_serde_array(
                message,
                &|m1: &protobuf::Message| extract_vec_shim(
                    m1,
                    &|m2| field_descriptor.len_field(m2),
                    &|m2, i| field_descriptor.get_rep_message_item(m2, i),
                ),
                &|m: &protobuf::Message| proto_to_json(m));
             */
        },
        FieldDescriptorProto_Type::TYPE_ENUM => {
            let mut enums = Vec::new();
            for i in 0..field_descriptor.len_field(message) {
                enums.push(field_descriptor.get_rep_enum_item(message, i));
            }
            Value::Array(enums.into_iter().map(
                |e| Value::String(e.name().to_string())).collect())
        },
        FieldDescriptorProto_Type::TYPE_GROUP => unimplemented!(),
    }
}

fn singular_field_to_json(message: &protobuf::Message,
                          field_descriptor: &protobuf::reflect::FieldDescriptor) -> serde_json::Value {
    match field_descriptor.proto().get_field_type() {
        FieldDescriptorProto_Type::TYPE_DOUBLE => {
            Value::F64(field_descriptor.get_f64(message))
        },
        FieldDescriptorProto_Type::TYPE_FLOAT => {
            Value::F64(field_descriptor.get_f32(message) as f64)
        },
        FieldDescriptorProto_Type::TYPE_INT32 |
        FieldDescriptorProto_Type::TYPE_SINT32 |
        FieldDescriptorProto_Type::TYPE_SFIXED32 => {
            Value::I64(field_descriptor.get_i32(message) as i64)
        },
        FieldDescriptorProto_Type::TYPE_INT64 |
        FieldDescriptorProto_Type::TYPE_SINT64 |
        FieldDescriptorProto_Type::TYPE_SFIXED64 => {
            Value::I64(field_descriptor.get_i64(message))
        },
        FieldDescriptorProto_Type::TYPE_UINT32 |
        FieldDescriptorProto_Type::TYPE_FIXED32 => {
            Value::U64(field_descriptor.get_u32(message) as u64)
        },
        FieldDescriptorProto_Type::TYPE_UINT64 |
        FieldDescriptorProto_Type::TYPE_FIXED64 => {
            Value::U64(field_descriptor.get_u64(message))
        },
        FieldDescriptorProto_Type::TYPE_BOOL => {
            Value::Bool(field_descriptor.get_bool(message))
        },
        FieldDescriptorProto_Type::TYPE_STRING => {
            Value::String(field_descriptor.get_str(message).to_string())
        },
        FieldDescriptorProto_Type::TYPE_BYTES => {
            Value::String(
                std::str::from_utf8(
                    field_descriptor.get_bytes(message)).unwrap().to_string())
        },
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            let sub_message: &protobuf::Message =
                field_descriptor.get_message(message);
            proto_to_json(sub_message)
        },
        FieldDescriptorProto_Type::TYPE_ENUM => {
            Value::String(
                field_descriptor.get_enum(message).name().to_string())

        },
        FieldDescriptorProto_Type::TYPE_GROUP => unimplemented!(),
    }
}