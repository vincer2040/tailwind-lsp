use anyhow::Result;

use lsp_types::{InitializeParams, ServerCapabilities,};

use lsp_types::{TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions};

use lsp_server::{Connection, Message};

use log::{warn, info};

use text_store::init_text_store;

use crate::handle::{handle_notification, handle_request};

mod text_store;
mod handle;
mod tailwind;
mod tree_sitter;
mod tree_sitter_queries;

pub fn start_lsp() -> Result<()> {

    init_text_store();

    info!("look mom, I'm starting an lsp server for tailwindcss");

    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec!["\"".to_string(), " ".to_string()]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
            all_commit_characters: None,
            completion_item: None,
        }),

        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),

        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    warn!("shutting down server");
    Ok(())
}

fn main_loop(connection: Connection, params: serde_json::Value) -> Result<()> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    info!("starting example main loop");
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                handle_request(req);
            }
            Message::Notification(not) => {
                handle_notification(not);
            }
            Message::Response(resp) => {
                info!("got response: {resp:?}");
            }
        }
    }
    Ok(())
}
