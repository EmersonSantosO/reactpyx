use std::io::{self};
use swc_core::common::{FileName, SourceMap};
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax};

/// Minifies JavaScript code using `swc_core`.
pub fn minify_js_code(js: &str) -> Result<String, io::Error> {
    let cm = SourceMap::default();

    // Create a source file for the compiler
    let fm = cm.new_source_file(FileName::Custom("input.js".into()), js.into());

    // Use a lexer to parse JavaScript
    let lexer = Lexer::new(
        Syntax::Es(EsSyntax::default()), // Define syntax for ES
        EsVersion::Es2021,               // Target ES version
        StringInput::from(&*fm),         // Input source
        None,
    );

    // Parse JavaScript code into an AST
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error parsing JS: {:?}", e),
        )
    })?;

    // Emit and minify JavaScript code
    let mut buf = vec![];
    let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
    let mut emitter = Emitter {
        cfg: Default::default(),
        cm,
        comments: None,
        wr: writer,
    };

    // Emit code as JavaScript from the AST
    emitter.emit_module(&module).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error emitting JS: {:?}", e),
        )
    })?;

    // Convert buffer to a UTF-8 string and return it
    String::from_utf8(buf).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid UTF-8 output: {}", e),
        )
    })
}
