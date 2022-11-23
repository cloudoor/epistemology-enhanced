
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