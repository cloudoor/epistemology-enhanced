
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use clap::Parser;
use core::panic;
use futures::StreamExt;
use gbnf::Grammar;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::certs;
use rustls_pemfile::rsa_private_keys;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Serialize, Deserialize)]
struct OlamaResponse {
    model: String,
    created_at: String,
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct OlamaRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    messages: Vec<Message>,
}

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct EpistemologyCliArgs {
    #[arg(short, value_name = "GGUF_MODEL", help = "Path to GGUF model")]
    model: PathBuf,

    #[arg(
        short,
        value_name = "OLLAMA_HOST",
        help = "Address of OLLAMA server http://localhost:11434"
    )]
    ollama_host: Option<String>,

    #[arg(
        short = 'e',
        long,
        value_name = "LLAMMA_CPP_MAIN_EXE_PATH",
        help = "Path to LLAMMA CPP main executable"
    )]
    exe_path: Option<PathBuf>,

    #[arg(
        short = 'd',
        long,
        value_name = "LLAMMA_CPP_EMBEDDING_EXE_PATH",
        help = "Path to LLAMMA CPP embedding executable"
    )]
    embedding_path: Option<PathBuf>,

    // num threads
    #[arg(
        short = 't',
        long,
        value_name = "NUM_THREADS",
        help = "Number of threads to use for LLM generation (default: 4)"
    )]
    threads: Option<u32>,

    #[arg(
        short = 'l',
        long,
        value_name = "NUM_GPU_LAYERS",
        help = "Number of layers to delegate to GPU"
    )]
    n_gpu_layers: Option<u32>,

    #[arg(
        short = 'g',
        long,
        value_name = "GRAMMAR_PATH",
        help = "Path to grammar file (optional)"
    )]
    grammar: Option<PathBuf>,

    //context length
    #[arg(
        short = 'c',
        long,
        value_name = "CONTEXT_LENGTH",
        help = "Context length of LLM generation (default: 512)"
    )]
    ctx_size: Option<u32>,

    #[arg(
        short = 'j',
        long,
        value_name = "JSON_SCHEMA_PATH",
        help = "Path to JSON schema file to constrain output (optional)"
    )]
    json_schema: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "UI_PATH",
        help = "Path to UI static files folder"
    )]
    ui: Option<PathBuf>,

    // Output length with default 512
    #[arg(
        short = 'n',
        long,
        value_name = "OUTPUT_LENGTH",
        help = "Output length of LLM generation"
    )]
    n_predict: Option<i32>,

    // Optional origin instead of localhost
    #[arg(
        short = 'a',
        long,
        value_name = "ADDRESS",
        help = "Optional address instead of default (e.g 0.0.0.0), default is localhost"
    )]
    address: Option<String>,

    // Port to serve on
    #[arg(short, long, value_name = "PORT", help = "Port to serve on")]
    port: Option<u16>,

    // HTTPS key file
    #[arg(
        long,
        value_name = "HTTPS_KEY_FILE",
        help = "HTTPS key file (optional)"
    )]
    https_key_file: Option<PathBuf>,

    // HTTPS cert file
    #[arg(
        long,
        value_name = "HTTPS_CERT_FILE",
        help = "HTTPS cert file (optional)"
    )]
    https_cert_file: Option<PathBuf>,
}

#[derive(Deserialize)]
struct TextCompletationRequestQuery {
    prompt: String,
}

async fn handle_completion_get(
    data: web::Data<EpistemologyCliArgs>,
    query: web::Query<TextCompletationRequestQuery>,
) -> impl Responder {
    run_streaming_llm(Mode::Completion, &data, query.prompt.clone())
}

async fn handle_completion_post(
    data: web::Data<EpistemologyCliArgs>,
    body: String,
) -> impl Responder {
    run_streaming_llm(Mode::Completion, &data, body)
}

async fn handle_chat_post(
    data: web::Data<EpistemologyCliArgs>,
    body: web::Json<ChatRequest>,
) -> impl Responder {
    run_chat(Mode::Chat, &data, body.into_inner())
}

async fn handle_embedding_post(
    data: web::Data<EpistemologyCliArgs>,
    body: String,
) -> impl Responder {
    run_streaming_llm(Mode::Embedding, &data, body)
}

async fn app() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/javascript"))
        .body(include_str!("./app.js"))
}

async fn lit() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/javascript"))
        .body(include_str!("./lit.js"))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("./index.html"))
}

async fn css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("./index.css"))
}

async fn inter() -> impl Responder {
    let font_bytes = include_bytes!("./Inter-Thin.ttf");
    HttpResponse::Ok()
        .content_type("font/ttf")
        .body(font_bytes.to_vec())
}

async fn icon() -> impl Responder {
    let icon_bytes = include_bytes!("./icon.png");
    HttpResponse::Ok()
        .content_type("image/png")
        .body(icon_bytes.to_vec())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli: EpistemologyCliArgs = EpistemologyCliArgs::parse();

    let port = cli.port.unwrap_or(8080);

    // let's make these parameters available to the web server for all requests to use
    let app_data = web::Data::new(cli.clone());

    let address = cli.address.unwrap_or("localhost".to_string());

    // ensure we have both key and cert if either is provided
    if cli.https_key_file.is_some() != cli.https_cert_file.is_some() {
        panic!("Must provide both HTTPS key and cert files");
    }

    let protocol = "https";

    // let's print out some helpful information for the user
    if let Some(ui) = &cli.ui {
        println!(
            "Serving UI on {}://{}:{}/ from {}",
            protocol,
            address,
            port,
            match fs::canonicalize(ui) {
                Ok(full_path) => full_path.display().to_string(),
                Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
            }
        );
    } else {
        println!(
            "Serving UI on {}://{}:{}/ from built-in UI",
            protocol, address, port
        );
    }
    println!(
        r#"Listening with GET and POST on {}://{}:{}/api/completion
Examples:
    * {}://{}:{}/api/completion?prompt=famous%20qoute:
    * curl -X POST -d "famous qoute:" {}://{}:{}/api/completion
    * curl -X POST -d "robots are good" {}://{}:{}/api/embedding"#,
        protocol,
        address,
        port,
        protocol,
        address,
        port,
        protocol,
        address,
        port,
        protocol,
        address,
        port
    );

    let s = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _req_head| true)
            .allowed_methods(vec!["GET", "POST"]);
        let mut a = App::new()
            .app_data(app_data.clone())
            .wrap(cors)
            .service(
                web::resource("/api/completion")
                    .route(web::get().to(handle_completion_get))
                    .route(web::post().to(handle_completion_post)),
            )
            .service(web::resource("/api/chat").route(web::post().to(handle_chat_post)))
            .service(web::resource("/api/embedding").route(web::post().to(handle_embedding_post)));

        // let's serve the UI if the user provided a path to a static folder of files
        if let Some(ui_path) = &cli.ui {
            a = a.service(
                actix_files::Files::new(
                    "/",
                    match fs::canonicalize(ui_path) {
                        Ok(full_path) => full_path.display().to_string(),
                        Err(err) => {
                            panic!("Failed to serve UI: {}", err)
                        }
                    },
                )
                .index_file("index.html"),
            );
        } else {
            a = a.route("/", web::get().to(index));
            a = a.route("/index.css", web::get().to(css));
            a = a.route("/Inter-Light.ttf", web::get().to(inter));
            a = a.route("/app.js", web::get().to(app));
            a = a.route("/lit.js", web::get().to(lit));
            a = a.route("/icon.png", web::get().to(icon));
        }

        a
    });

    if let (Some(key_file), Some(cert_file)) = (&cli.https_key_file, &cli.https_cert_file) {
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth();

        // load TLS key/cert files
        let cert_file = &mut BufReader::new(File::open(cert_file).unwrap());
        let key_file = &mut BufReader::new(File::open(key_file).unwrap());

        // convert files to key/cert objects
        let cert_chain: Vec<Certificate> = certs(cert_file)
            .map(|d| {
                let der = d.unwrap();
                Certificate(der.to_vec())
            })
            .collect();
        let mut keys: Vec<PrivateKey> = rsa_private_keys(key_file)
            .map(|d| {
                let der = d.unwrap();
                // convert to PKCS 8 key
                PrivateKey(der.secret_pkcs1_der().to_vec())
            })
            .collect();

        // exit if no keys could be parsed
        if keys.is_empty() {
            eprintln!("Could not load private keys, should be RSA? Did you use mkcert?");
            std::process::exit(1);
        }

        let sc = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();

        s.bind_rustls(format!("{}:{}", address, port), sc)?
            .run()
            .await
    } else {
        let cert = rcgen::generate_simple_self_signed(vec![address.to_owned()]).unwrap();
        let cert_file = cert.serialize_der().unwrap();
        let key_file = cert.serialize_private_key_der();
        let pk = PrivateKey(key_file);

        let cert_chain = Certificate(cert_file);
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth();

        let sc: rustls::ServerConfig = config.with_single_cert(vec![cert_chain], pk).unwrap();
        s.bind_rustls(format!("{}:{}", address, port), sc)?
            .run()
            .await
    }
}

#[derive(PartialEq)]
enum Mode {
    Chat,
    Completion,
    Embedding,
}

fn run_streaming_llm(mode: Mode, args: &EpistemologyCliArgs, prompt: String) -> impl Responder {
    if let Mode::Embedding = mode {
        if args.embedding_path.is_none() {
            return HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("Embedding mode requires embedding path, look at help for more information");
        }
    }

    let (tx, rx) = mpsc::unbounded_channel();

    let a = args.clone();

    if args.ollama_host.is_some() {
        return HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("Ollama completions not supported");
    } else {
        // Spawn a thread to execute the command and send output to the channel
        thread::spawn(move || match run_llama_cli(mode, &a, prompt, tx) {
            Ok(_) => {}
            Err(_) => eprintln!("Something went wrong while executing the llama.cpp exe"),
        });
    }

    // Convert the synchronous Flume receiver into an asynchronous stream
    let async_stream = UnboundedReceiverStream::from(rx)
        .map(|line| Ok::<_, actix_web::Error>(web::Bytes::from(line)));

    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(async_stream)
}

fn run_chat(mode: Mode, args: &EpistemologyCliArgs, chat_request: ChatRequest) -> impl Responder {
    if let Mode::Embedding = mode {
        if args.embedding_path.is_none() {
            return HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("Embedding mode requires embedding path, look at help for more information");
        }
    }