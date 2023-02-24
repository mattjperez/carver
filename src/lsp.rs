#![allow(dead_code)]
/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification

use serde_json::{Value, Value::Array};

type Integer = i32;
type UInteger= u16;
type Decimal = i32;


#[non_exhaustive]
enum ErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ServerNotInitialized,
	UnknownErrorCode,
    RequestFailed,
    ServerCancelled,
    ContentModified,
    RequestCancelled,
    
    // jsonrpcReservedErrorRangeStart: integer = -32099;
    // export const jsonrpcReservedErrorRangeEnd = -32000;
    // export const lspReservedErrorRangeStart: integer = -32899;
    // 	export const lspReservedErrorRangeEnd: integer = -32800;
}

impl ErrorCode {
    fn code(self) -> i32 {
        use ErrorCode::*;
        match self {
            ParseError => -32700,
            InvalidRequest => -32600,
            MethodNotFound => -32601,
            InvalidParams => -32602,
            InternalError => -32603,
            ServerNotInitialized => -32002,
            UnknownErrorCode => -32001,
            RequestFailed =>-32803,
            ServerCancelled =>-32802,
            ContentModified => -32801,
            RequestCancelled => -32800,
            _ => 0, // change this
        }
    }
}

struct Header {
    content_length: usize, // Content-Length, Content length in bytes
    content_type: Option<String>, // Content-Type: "application/vscode-jsonrpc"
}

// encode using utf-8
struct Content(String);

struct RequestMessage {
    jsonrpc: String, // "2.0"
    id: Integer, // serde_json::Number
    method: String, //serde_json::String
    params: Option<Vec<String>>, //serde_json::Array
}

struct ResponseMessage {
    jsonrpc: String, // "2.0"
    result: Option<Value>, // serde_json::Value
    error: Option<ResponseError>, // serde_json::Value::Null
}

struct ResponseError {
    code: Integer,
    message: String,
    data: Option<Value>

}

struct NotificationMessage {
    jsonrpc: String, // "2.0",
    method: String,
    params: Option<Vec<Value>>, // serde_json::Value::Array
}
