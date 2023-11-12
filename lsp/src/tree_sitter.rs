use lsp_types::TextDocumentPositionParams;
use log::{error, debug};
use tree_sitter::{Parser, Point, Node};

use crate::{text_store::get_text_document, tree_sitter_queries::query_attr_values_for_completion};

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    AttributeName(String),
    AttributeValue { name: String, value: String },
}

fn find_element_referent_to_current_node(node: Node<'_>) -> Option<Node<'_>> {
    debug!("node: {:?}", node);
    if node.kind() == "element" || node.kind() == "fragment" {
        return Some(node);
    }
    return find_element_referent_to_current_node(node.parent()?);
}

fn query_position(root: Node<'_>, source: &str, trigger_point: Point) {
    debug!("query_position root: {:?}", root.to_sexp());
    let closest_node = root.descendant_for_point_range(trigger_point, trigger_point).expect("descendant_for_point_range");
    debug!("query_position closest node: {:?}", closest_node);
    let element = find_element_referent_to_current_node(closest_node);
    debug!("element: {:?}", element);

    let attr_completion = query_attr_values_for_completion(root, source, trigger_point);

    debug!("attr_completion: {:?}", attr_completion);
}

pub fn get_position_from_lsp_completion(text_params: TextDocumentPositionParams) {
    let text = get_text_document(text_params.text_document.uri).expect("text");
    error!("get_position_from_lsp_completion text: {:?}", text);
    let pos = text_params.position;
    error!("get_position_from_lsp_completion pos: {:?}", pos);

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_html::language())
        .expect("could not load html grammer");

    let tree = parser.parse(&text, None).expect("it to parse");
    let root_node = tree.root_node();
    let trigger_point = Point::new(pos.line as usize, pos.character as usize);
    query_position(root_node, &text, trigger_point);
}
