use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};
use markup5ever_rcdom::RcDom;
use std::io::{self, BufReader};

/// Minifica código HTML usando `html5ever`.
pub fn minify_html_code(html: &str) -> Result<String, io::Error> {
    // Analiza HTML en una estructura DOM
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut BufReader::new(html.as_bytes()))
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error al analizar HTML: {:?}", e),
            )
        })?;

    // Serializa DOM de vuelta a HTML con minificación
    let mut minified_html = Vec::new();
    serialize(&mut minified_html, &dom.document, Default::default()).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error al serializar HTML: {:?}", e),
        )
    })?;

    String::from_utf8(minified_html).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Salida UTF-8 inválida: {}", e),
        )
    })
}
