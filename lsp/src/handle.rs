use crate::{text_store::TEXT_STORE, tailwind::tailwind_completion};
use log::{debug, error, warn};
use lsp_server::{Notification, Request};
use lsp_types::{CompletionContext, CompletionParams, CompletionTriggerKind};

#[derive(serde::Deserialize, Debug)]
struct Text {
    text: String,
}

#[derive(serde::Deserialize, Debug)]
struct TextDocumentLocation {
    uri: String,
}

#[derive(serde::Deserialize, Debug)]
struct TextDocumentChanges {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentLocation,

    #[serde(rename = "contentChanges")]
    content_changes: Vec<Text>,
}

#[derive(serde::Deserialize, Debug)]
struct TextDocumentOpened {
    uri: String,

    text: String,
}

#[derive(serde::Deserialize, Debug)]
struct TextDocumentOpen {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentOpened,
}

pub fn handle_notification(noti: Notification) {
    match noti.method.as_str() {
        "textDocument/didChange" => handle_did_change(noti),
        "textDocument/didOpen" => handle_did_open(noti),
        s => {
            debug!("unhandled notification {}", s);
        }
    }
}

pub fn handle_request(req: Request) {
    error!("handle request");
    match req.method.as_str() {
        "textDocument/completion" => handle_completion(req),
        _ => {
            warn!("unhandled request {:?}", req);
        }
    }
}

fn handle_did_open(noti: Notification) {
    debug!("handle_did_open params {:?}", noti.params);

    let text_document_changes = match serde_json::from_value::<TextDocumentOpen>(noti.params) {
        Ok(p) => p.text_document,
        Err(err) => {
            error!("handle did open params error {:?}", err);
            return;
        }
    };

    TEXT_STORE
        .get()
        .expect("text store not initialized")
        .lock()
        .expect("text store mutex poisoned")
        .texts
        .insert(
            text_document_changes.uri,
            text_document_changes.text.to_string(),
        );
}

fn handle_did_change(noti: Notification) {
    let text_document_changes: TextDocumentChanges = match serde_json::from_value(noti.params) {
        Ok(p) => p,
        Err(err) => {
            error!("did change params error {:?}", err);
            return;
        }
    };
    let uri = text_document_changes.text_document.uri;
    let text = text_document_changes.content_changes[0].text.to_string();

    if text_document_changes.content_changes.len() > 1 {
        error!("more than one content change, please be wary");
    }

    TEXT_STORE
        .get()
        .expect("text store not initialized")
        .lock()
        .expect("text store mutex poisoned")
        .texts
        .insert(uri, text);
}

fn handle_completion(req: Request) {
    let completion: CompletionParams =
        serde_json::from_value(req.params).expect("completion params");
    error!("handle completion: {:?}", completion);

    match completion.context {
        Some(CompletionContext {
            trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
            ..
        })
        | Some(CompletionContext {
            trigger_kind: CompletionTriggerKind::INVOKED,
            ..
        }) => {
            tailwind_completion(completion.text_document_position);
        }
        _ => {
            error!("unhandled completion: {:?}", completion.context);
        }
    }
}
