use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};
use markup5ever_rcdom::RcDom;
use std::io::{self, BufReader};

/// Minifies HTML code using `html5ever`.
pub fn minify_html_code(html: &str) -> Result<String, io::Error> {
    // Parse HTML into a DOM structure
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut BufReader::new(html.as_bytes()))
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error parsing HTML: {:?}", e),
            )
        })?;

    // Serialize DOM back to HTML with minification
    let mut minified_html = Vec::new();
    serialize(&mut minified_html, &dom.document, Default::default()).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error serializing HTML: {:?}", e),
        )
    })?;

    String::from_utf8(minified_html).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid UTF-8 output: {}", e),
        )
    })
}
