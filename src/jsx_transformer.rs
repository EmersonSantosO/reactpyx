use pyo3::prelude::*;
use regex::Regex;

/// Incremental transformation of JSX code to a Python-compatible format
#[pyfunction]
pub fn incremental_jsx_transform(js_code: &str) -> PyResult<String> {
    parse_jsx(js_code)
}

/// Complete transformation of JSX code to Python-compatible code
#[pyfunction]
pub fn parse_jsx(js_code: &str) -> PyResult<String> {
    // Simple state machine parser for JSX in Python
    // This is a significant improvement over simple string replacement
    // but still a simplified implementation compared to a full AST parser.

    let mut output = String::new();
    let mut chars = js_code.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            // Potential JSX start
            // Check if it's a tag and not a less-than operator
            // Heuristic: followed by a letter or /
            let next = chars.peek();
            if let Some(&n) = next {
                if n.is_alphabetic() || n == '/' || n == '>' {
                    // It is likely a tag
                    let tag_content = read_until_matching_bracket(&mut chars);
                    let transformed = transform_tag(&tag_content);
                    output.push_str(&transformed);
                    continue;
                }
            }
            output.push(c);
        } else {
            output.push(c);
        }
    }

    Ok(output)
}

fn read_until_matching_bracket(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1; // We already consumed the first '<'

    // Note: This is a simplified reader that doesn't handle strings containing '>' correctly
    // A full implementation would need to track string state.
    while let Some(c) = chars.next() {
        if c == '>' {
            depth -= 1;
            if depth == 0 {
                break;
            }
            content.push(c);
        } else if c == '<' {
            depth += 1;
            content.push(c);
        } else {
            content.push(c);
        }
    }
    content
}

fn transform_tag(tag_content: &str) -> String {
    // tag_content is what's inside <...>, e.g. "div className='foo'" or "/div"

    let trimmed = tag_content.trim();

    if trimmed.starts_with('/') {
        // Closing tag: </div> -> "])"
        // We need to match the structure of create_element
        // create_element(tag, props, [children...])
        // So a closing tag ends the children array and the function call.
        return "])".to_string();
    }

    // Opening tag
    // Parse tag name and attributes
    // <div className="foo"> -> create_element("div", {"className": "foo"}, [

    let mut parts = trimmed.split_whitespace();
    let tag_name = parts.next().unwrap_or("div");

    // Handle self-closing tags ending with /
    let is_self_closing = trimmed.ends_with('/');
    let clean_tag_name = if is_self_closing && tag_name.ends_with('/') {
        &tag_name[..tag_name.len() - 1]
    } else {
        tag_name
    };

    // Parse attributes
    // This is a naive attribute parser
    let mut props_str = String::from("{");
    let mut first_prop = true;

    // Re-join the rest to parse attributes properly (handling spaces in values)
    let rest = &trimmed[tag_name.len()..].trim();
    if !rest.is_empty() {
        let attr_regex = Regex::new(r#"(\w+)=(?:\{([^}]+)\}|"([^"]+)"|'([^']+)')"#).unwrap();
        for cap in attr_regex.captures_iter(rest) {
            if !first_prop {
                props_str.push_str(", ");
            }
            let key = &cap[1];
            let value = if let Some(v) = cap.get(2) {
                // Expression: {value}
                v.as_str().to_string()
            } else if let Some(v) = cap.get(3) {
                // Double quoted: "value"
                format!("\"{}\"", v.as_str())
            } else {
                // Single quoted: 'value'
                format!("\"{}\"", cap.get(4).unwrap().as_str())
            };

            // Handle className -> class mapping if needed, or keep as is
            let final_key = if key == "className" { "class" } else { key };

            // If value is a string literal, quote it, otherwise keep as expression
            // The regex logic above already handles quoting for string literals
            // But we need to be careful about the expression case.

            if cap.get(2).is_some() {
                props_str.push_str(&format!("\"{}\": {}", final_key, value));
            } else {
                props_str.push_str(&format!("\"{}\": {}", final_key, value));
            }

            first_prop = false;
        }
    }
    props_str.push('}');

    if is_self_closing {
        format!("create_element(\"{}\", {}, [])", clean_tag_name, props_str)
    } else {
        format!("create_element(\"{}\", {}, [", clean_tag_name, props_str)
    }
}
