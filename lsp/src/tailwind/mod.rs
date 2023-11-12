use lsp_types::TextDocumentPositionParams;
use log::debug;

use crate::tree_sitter::get_position_from_lsp_completion;

pub fn tailwind_completion(text_params: TextDocumentPositionParams) {
    debug!("params: {:?}", text_params);
    get_position_from_lsp_completion(text_params);
}
