use std::collections::HashMap;

use log::{error, debug};
use tree_sitter::{Node, Point, Query, QueryCursor};

use crate::tree_sitter::Position;

#[derive(Debug)]
struct CaptureDetails {
    value: String,
    end_position: Point,
}

fn query_props(query_string: &str, node: Node<'_>, source: &str, trigger_point: Point) -> HashMap<String, CaptureDetails> {
    let query = Query::new(tree_sitter_html::language(), query_string).expect("get_position_by_query invalid query");
    let mut cursor_qry = QueryCursor::new();
    let capture_names = query.capture_names();
    let matches = cursor_qry.matches(&query, node, source.as_bytes());
    matches
        .into_iter()
        .flat_map(|m| {
            m.captures
                .iter()
                .filter(|capture| capture.node.start_position() <= trigger_point)
        })
        .fold(HashMap::new(), |mut acc, capture| {
            let key = capture_names[capture.index as usize].to_owned();
            let value = if let Ok(capture_value) = capture.node.utf8_text(source.as_bytes()) {
                capture_value.to_owned()
            } else {
                error!("query_props capture.node.utf8_text failed {key}");
                "".to_owned()
            };
            acc.insert(key, CaptureDetails { value, end_position: capture.node.end_position() });
            acc
        })
}

pub fn query_attr_values_for_completion(node: Node<'_>, source: &str, trigger_point: Point) -> Option<Position> {
    let query_string = r#"(
        [
          (ERROR
            (tag_name)

            (attribute_name) @attr_name
            (_)
          ) @open_quote_error

          (_
            (tag_name)

            (attribute
              (attribute_name) @attr_name
              (_)
            ) @last_item

            (ERROR) @error_char
          )

          (_
            (tag_name)

            (attribute
              (attribute_name) @attr_name
              (quoted_attribute_value) @quoted_attr_value

              (#eq? @quoted_attr_value "\"\"")
            ) @empty_attribute
          )

          (_
            (tag_name)

            (attribute
              (attribute_name) @attr_name
              (quoted_attribute_value (attribute_value) @attr_value)

              ) @non_empty_attribute
          )
        ]

        (#match? @attr_name "class")
    )"#;

    let props = query_props(query_string, node, source, trigger_point);

    let attr_name = props.get("attr_name")?;
    debug!("query_attr_values_for_completion attr_name {:?}", attr_name);

    if props.get("open_quote_error").is_some() || props.get("empty_attribute").is_some() {
        return Some(Position::AttributeValue {
            name: attr_name.value.to_owned(),
            value: "".to_string(),
        });
    }

    if let Some(capture) = props.get("non_empty_attribute") {
        if trigger_point >= capture.end_position {
            return None;
        }
    }

    Some(Position::AttributeValue {
        name: attr_name.value.to_owned(),
        value: "".to_string(),
    })

}
