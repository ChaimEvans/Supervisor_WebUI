use serde::Deserialize;

//getVersion
//readMainLog
//-readProcessLog
pub static READ_PROCESS_LOG: &str = "readprocesslog";
//
pub static GET_API_VERSION: &str = "getapiversion";
pub static GET_SUPERVISOR_VERSION: &str = "getsupervisorversion";
pub static GET_IDENTIFICATION: &str = "getidentification";
pub static GET_STATE: &str = "getstate";
pub static GET_PID: &str = "getpid";
pub static READ_LOG: &str = "readlog";
pub static CLEAR_LOG: &str = "clearlog";
pub static SHUTDOWN: &str = "shutdown";
pub static RESTART: &str = "restart";
pub static GET_PROCESS_INFO: &str = "getprocessinfo";
pub static GET_ALL_PROCESS_INFO: &str = "getallprocessinfo";
pub static GET_ALL_CONFIG_INFO: &str = "getallconfiginfo";
pub static START_PROCESS: &str = "startprocess";
pub static START_ALL_PROCESSES: &str = "startallprocesses";
pub static START_PROCESS_GROUP: &str = "startprocessgroup";
pub static STOP_PROCESS: &str = "stopprocess";
pub static STOP_ALL_PROCESSES: &str = "stopallprocesses";
pub static STOP_PROCESS_GROUP: &str = "stopprocessgroup";
pub static SIGNAL_PROCESS: &str = "signalprocess";
pub static SIGNAL_PROCESS_GROUP: &str = "signalprocessgroup";
pub static SIGNAL_ALL_PROCESSES: &str = "signalallprocesses";
pub static SEND_PROCESS_STDIN: &str = "sendprocessstdin";
pub static SEND_REMOTE_COMM_EVENT: &str = "sendremotecommevent";
pub static RELOAD_CONFIG: &str = "reloadconfig";
pub static ADD_PROCESS_GROUP: &str = "addprocessgroup";
pub static REMOVE_PROCESS_GROUP: &str = "removeprocessgroup";
pub static READ_PROCESS_STDOUT_LOG: &str = "readprocessstdoutlog";
pub static READ_PROCESS_STDERR_LOG: &str = "readprocessstderrlog";
pub static TAIL_PROCESS_STDOUT_LOG: &str = "tailprocessstdoutlog";
pub static TAIL_PROCESS_STDOERR_LOG: &str = "tailprocessstderrlog";
pub static CLEAR_PROCESS_LOGS: &str = "clearprocesslogs";
pub static CLEAR_ALL_PROCESS_LOGS: &str = "clearallprocesslogs";

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RequestParams {
    StringI32I32 { a: String, b: i32, c: i32 },
    StringString { a: String, b: String },
    StringBool { a: String, b: bool },
    I32I32 { a: i32, b: i32 },
    String { a: String },
    Bool { a: bool },
    Empty {},
}

impl RequestParams {
    pub fn to_string_i32_i32(self) -> Result<(String, i32, i32), String> {
        if let RequestParams::StringI32I32 { a, b, c } = self {
            Ok((a, b, c))
        } else {
            Err("Bad request")?
        }
    }

    pub fn to_string_string(self) -> Result<(String, String), String> {
        if let RequestParams::StringString { a, b } = self {
            Ok((a, b))
        } else {
            Err("Bad request")?
        }
    }
    pub fn to_string_bool(self) -> Result<(String, bool), String> {
        if let RequestParams::StringBool { a, b } = self {
            Ok((a, b))
        } else {
            Err("Bad request")?
        }
    }
    pub fn to_i32_i32(self) -> Result<(i32, i32), String> {
        if let RequestParams::I32I32 { a, b } = self {
            Ok((a, b))
        } else {
            Err("Bad request")?
        }
    }
    pub fn to_string(self) -> Result<String, String> {
        if let RequestParams::String { a } = self {
            Ok(a)
        } else {
            Err("Bad request")?
        }
    }

    pub fn to_bool(self) -> Result<bool, String> {
        if let RequestParams::Bool { a } = self {
            Ok(a)
        } else {
            Err("Bad request")?
        }
    }

    pub fn is_empty(self) -> Result<(), String> {
        if let RequestParams::Empty {} = self {
            Ok(())
        } else {
            Err("Bad request")?
        }
    }
}

#[derive(Deserialize)]
pub struct LogRequestParams {
    pub offset: i32,
    pub length: i32,
    pub process: String,
}