use ::lazy_static::lazy_static;
use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::{self, Bytes},
    App, HttpResponse, HttpServer, Responder,
};
use futures::TryStreamExt;
use rust_embed::{EmbeddedFile, RustEmbed};
use serde_json::json;
use std::collections::HashMap;

mod api_info;

#[derive(RustEmbed)]
#[folder = "web/"]
#[prefix = "/"]
struct Asset;

lazy_static! {
    static ref HTML: EmbeddedFile = Asset::get("/index.html").unwrap();
    static ref JAVASCRIPT: EmbeddedFile = Asset::get("/main.js").unwrap();
    static ref CSS: EmbeddedFile = Asset::get("/style.css").unwrap();
    static ref MDUI_CSS: EmbeddedFile = Asset::get("/mdui.css").unwrap();
    static ref MDUI_JAVASCRIPT: EmbeddedFile = Asset::get("/mdui.global.js").unwrap();
    static ref ICON: EmbeddedFile = Asset::get("/icon.css").unwrap();
    static ref WOFF2: EmbeddedFile = Asset::get("/flUhRq6tzZclQEJ-Vdg-IuiaDsNc.woff2").unwrap();
    // static ref SERVER_URLS: Vec<String> = vec!["http://192.168.1.100:9000/RPC2".to_string()];
    static ref SERVER_URLS: HashMap<String, String> = {
        let mut urls: Vec<String> = std::env::args().collect();
        urls.remove(0);
        urls.remove(0);
        let mut urls_map: HashMap<String, String> = HashMap::new();
        for url in urls{
            if let Some(host) = dxr_client::Url::parse(&url).expect("Invalid Url.").host_str(){
                urls_map.insert(host.to_string(), url);
            }else {
                panic!("Invalid Url.")
            }
        }
        urls_map
    };
    static ref SERVER_HOSTS: Vec<String> = {
        let mut hosts: Vec<String> = Vec::new();
        for (key,_) in SERVER_URLS.iter() {
            hosts.push(key.clone());
        }
        hosts
    };

}

#[get("/")]
async fn get_html() -> impl Responder {
    HttpResponse::Ok().body(HTML.data.as_ref())
}

#[get("/main.js")]
async fn get_javascript() -> impl Responder {
    HttpResponse::Ok().body(JAVASCRIPT.data.as_ref())
}

#[get("/style.css")]
async fn get_css() -> impl Responder {
    HttpResponse::Ok().body(CSS.data.as_ref())
}

#[get("/mdui.global.js")]
async fn get_mdui_javascript() -> impl Responder {
    HttpResponse::Ok().body(MDUI_JAVASCRIPT.data.as_ref())
}

#[get("/mdui.css")]
async fn get_mdui_css() -> impl Responder {
    HttpResponse::Ok().body(MDUI_CSS.data.as_ref())
}

#[get("/icon.css")]
async fn get_icon() -> impl Responder {
    HttpResponse::Ok().body(ICON.data.as_ref())
}

#[get("/flUhRq6tzZclQEJ-Vdg-IuiaDsNc.woff2")]
async fn get_woff2() -> impl Responder {
    HttpResponse::Ok().body(WOFF2.data.as_ref())
}

#[get("/servers")]
async fn servers() -> impl Responder {
    HttpResponse::Ok().json(json!(&*SERVER_HOSTS))
}

#[get("/infos")]
async fn infos() -> impl Responder {
    // let mut servers: HashMap<String,> = HashMap::new();
    let mut serverinfos: HashMap<String, serde_json::Value> = HashMap::new();
    let mut processes: HashMap<String, Option<Vec<supervisor_xmlrpc::res_struct::GetProcessInfo>>> =
        HashMap::new();
    for (key, url) in SERVER_URLS.iter() {
        let state_res = supervisor_xmlrpc::url(url).get_state().await;
        let state_res = if state_res.is_ok() {
            Some(state_res.unwrap())
        } else {
            Some(supervisor_xmlrpc::res_struct::GetState {
                statecode: -2,
                statename: "NetWorkError".to_string(),
            })
        };
        let ver_res = supervisor_xmlrpc::url(url).get_supervisor_version().await;
        let ver_res = if ver_res.is_ok() {
            Some(ver_res.unwrap())
        } else {
            None
        };
        let api_res = supervisor_xmlrpc::url(url).get_api_version().await;
        let api_res = if api_res.is_ok() {
            Some(api_res.unwrap())
        } else {
            None
        };
        let ident_res = supervisor_xmlrpc::url(url).get_identification().await;
        let ident_res = if ident_res.is_ok() {
            Some(ident_res.unwrap())
        } else {
            None
        };

        serverinfos.insert(
            key.clone(),
            json!({
                "state": state_res,
                "version":ver_res,
                "api":api_res,
                "identification":ident_res}),
        );

        if let Ok(res) = supervisor_xmlrpc::url(url).get_all_process_info().await {
            processes.insert(key.clone(), Some(res));
        } else {
            processes.insert(key.clone(), None);
        }
    }
    HttpResponse::Ok().json(json!({"Servers":serverinfos,"Processes": processes}))
}

#[post("/api/{index}/{api}")]
async fn api(
    url_path: web::Path<(String, String)>,
    info: web::Json<api_info::RequestParams>,
) -> impl Responder {
    if !SERVER_URLS.contains_key(&url_path.0) {
        return HttpResponse::Ok().json(json!({"code":0,"data":"Server not found."}));
    }
    match api_handler(info, url_path.0.clone(), url_path.1.clone()).await {
        Ok(res) => HttpResponse::Ok().json(json!({"code":1,"data":res})),
        Err(e) => HttpResponse::Ok().json(json!({"code":0,"data":e.to_string()})),
    }
}

async fn api_handler(
    info: web::Json<api_info::RequestParams>,
    server_index: String,
    api_name: String,
) -> Result<serde_json::Value, APIHandlerError> {
    Ok(match api_name.to_lowercase().as_str() {
        s if s == api_info::GET_API_VERSION => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_api_version()
                    .await?
            )
        }
        s if s == api_info::GET_SUPERVISOR_VERSION => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_supervisor_version()
                    .await?
            )
        }
        s if s == api_info::GET_IDENTIFICATION => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_identification()
                    .await?
            )
        }
        s if s == api_info::GET_STATE => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_state()
                    .await?
            )
        }
        s if s == api_info::GET_PID => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_pid()
                    .await?
            )
        }
        s if s == api_info::READ_LOG => {
            let (value1, value2) = info.into_inner().to_i32_i32()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .read_log(value1, value2)
                    .await?
            )
        }
        s if s == api_info::CLEAR_LOG => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .clear_log()
                    .await?
            )
        }
        s if s == api_info::SHUTDOWN => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .shutdown()
                    .await?
            )
        }
        s if s == api_info::RESTART => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .restart()
                    .await?
            )
        }
        s if s == api_info::GET_PROCESS_INFO => {
            let value1 = info.into_inner().to_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_process_info(&value1)
                    .await?
            )
        }
        s if s == api_info::GET_ALL_PROCESS_INFO => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_all_process_info()
                    .await?
            )
        }
        s if s == api_info::GET_ALL_CONFIG_INFO => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .get_all_config_info()
                    .await?
            )
        }
        s if s == api_info::START_PROCESS => {
            let (value1, value2) = info.into_inner().to_string_bool()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .start_process(&value1, value2)
                    .await?
            )
        }
        s if s == api_info::START_ALL_PROCESSES => {
            let value1 = info.into_inner().to_bool()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .start_all_processes(value1)
                    .await?
            )
        }
        s if s == api_info::START_PROCESS_GROUP => {
            let (value1, value2) = info.into_inner().to_string_bool()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .start_process_group(&value1, value2)
                    .await?
            )
        }
        s if s == api_info::STOP_PROCESS => {
            let (value1, value2) = info.into_inner().to_string_bool()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .stop_process(&value1, value2)
                    .await?
            )
        }
        s if s == api_info::STOP_ALL_PROCESSES => {
            let value1 = info.into_inner().to_bool()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .stop_all_processes(value1)
                    .await?
            )
        }
        s if s == api_info::STOP_PROCESS_GROUP => {
            let (value1, value2) = info.into_inner().to_string_bool()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .stop_process_group(&value1, value2)
                    .await?
            )
        }
        s if s == api_info::SIGNAL_PROCESS => {
            let (value1, value2) = info.into_inner().to_string_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .signal_process(&value1, &value2)
                    .await?
            )
        }
        s if s == api_info::SIGNAL_PROCESS_GROUP => {
            let (value1, value2) = info.into_inner().to_string_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .signal_process_group(&value1, &value2)
                    .await?
            )
        }
        s if s == api_info::SIGNAL_ALL_PROCESSES => {
            let value1 = info.into_inner().to_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .signal_all_processes(&value1)
                    .await?
            )
        }
        s if s == api_info::SEND_PROCESS_STDIN => {
            let (value1, value2) = info.into_inner().to_string_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .send_process_stdin(&value1, &value2)
                    .await?
            )
        }
        s if s == api_info::SEND_REMOTE_COMM_EVENT => {
            let (value1, value2) = info.into_inner().to_string_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .send_remote_comm_event(&value1, &value2)
                    .await?
            )
        }
        s if s == api_info::RELOAD_CONFIG => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .reload_config()
                    .await?
            )
        }
        s if s == api_info::ADD_PROCESS_GROUP => {
            let value1 = info.into_inner().to_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .add_process_group(&value1)
                    .await?
            )
        }
        s if s == api_info::REMOVE_PROCESS_GROUP => {
            let value1 = info.into_inner().to_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .remove_process_group(&value1)
                    .await?
            )
        }
        s if s == api_info::READ_PROCESS_LOG => {
            let (value1, value2, value3) = info.into_inner().to_string_i32_i32()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .read_process_log(&value1, value2, value3)
                    .await?
            )
        }
        s if s == api_info::READ_PROCESS_STDOUT_LOG => {
            let (value1, value2, value3) = info.into_inner().to_string_i32_i32()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .read_process_stdout_log(&value1, value2, value3)
                    .await?
            )
        }
        s if s == api_info::READ_PROCESS_STDERR_LOG => {
            let (value1, value2, value3) = info.into_inner().to_string_i32_i32()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .read_process_stderr_log(&value1, value2, value3)
                    .await?
            )
        }
        s if s == api_info::TAIL_PROCESS_STDOUT_LOG => {
            let (value1, value2, value3) = info.into_inner().to_string_i32_i32()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .tail_process_stdout_log(&value1, value2, value3)
                    .await?
            )
        }
        s if s == api_info::TAIL_PROCESS_STDOERR_LOG => {
            let (value1, value2, value3) = info.into_inner().to_string_i32_i32()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .tail_process_stderr_log(&value1, value2, value3)
                    .await?
            )
        }
        s if s == api_info::CLEAR_PROCESS_LOGS => {
            let value1 = info.into_inner().to_string()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .clear_process_logs(&value1)
                    .await?
            )
        }
        s if s == api_info::CLEAR_ALL_PROCESS_LOGS => {
            info.into_inner().is_empty()?;
            json!(
                supervisor_xmlrpc::url(SERVER_URLS.get(&server_index).unwrap())
                    .clear_all_process_logs()
                    .await?
            )
        }
        _ => Err("Bad request.".to_string())?,
    })
}

#[get("/log/{method}/{server}")]
async fn read_log(
    path: web::Path<(String, String)>,
    params: web::Query<api_info::LogRequestParams>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let (method, server) = path.into_inner();
    let res = reqwest::Client::new()
        .post(SERVER_URLS.get(&server).unwrap())
        .body(format!(
            r#"<?xml version='1.0'?>
        <methodCall>
        <methodName>supervisor.{}</methodName>
        <params>
        {}
        <param>
        <value><int>{}</int></value>
        </param>
        <param>
        <value><int>{}</int></value>
        </param>
        </params>
        </methodCall>"#,
            method,
            if params.process.is_empty() {
                "".to_string()
            } else {
                format!(
                    "<param><value><string>{}</string></value></param>",
                    params.process
                )
            },
            params.offset,
            params.length
        ))
        .send()
        .await;
    match res {
        Ok(response) => {
            let stream = response
                .bytes_stream()
                .map_err(|e| {
                    println!("error: {}", e);
                    actix_web::error::ErrorInternalServerError("Internal error")
                })
                .map_ok(move |chunk| {
                    // std::thread::sleep(std::time::Duration::from_secs(5));
                    // 清理xml标签
                    let content = String::from_utf8_lossy(&chunk);
                    let res_content = lazy_regex::regex_replace_all!(r#"<.*?>\n?"#i, &content, "");
                    Bytes::from(res_content.to_string())
                });
            Ok(actix_web::HttpResponse::Ok()
                .insert_header(ContentType::plaintext())
                .streaming(stream))
        }
        Err(_) => Ok(actix_web::HttpResponse::InternalServerError().finish()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let params: Vec<String> = std::env::args().collect();
    let port = params[1].parse::<u16>().unwrap();
    HttpServer::new(|| {
        App::new()
            .service(get_html)
            .service(get_javascript)
            .service(get_css)
            .service(get_mdui_javascript)
            .service(get_mdui_css)
            .service(get_icon)
            .service(get_woff2)
            .service(api)
            .service(servers)
            .service(infos)
            .service(read_log)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

enum APIHandlerError {
    A(String),
    B(dxr_client::ClientError),
}

impl From<String> for APIHandlerError {
    fn from(value: String) -> Self {
        APIHandlerError::A(value)
    }
}

impl From<dxr_client::ClientError> for APIHandlerError {
    fn from(value: dxr_client::ClientError) -> Self {
        APIHandlerError::B(value)
    }
}

impl ToString for APIHandlerError {
    fn to_string(&self) -> String {
        match self {
            APIHandlerError::A(v) => v.clone(),
            APIHandlerError::B(v) => v.to_string(),
        }
    }
}
