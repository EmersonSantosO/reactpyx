use std::io::{self};
use swc_core::common::{FileName, SourceMap};
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax};

/// Minifica código JavaScript usando `swc_core`.
pub fn minify_js_code(js: &str) -> Result<String, io::Error> {
    let cm = SourceMap::default();

    // Crea un archivo fuente para el compilador
    let fm = cm.new_source_file(FileName::Custom("input.js".into()), js.into());

    // Usa un lexer para analizar JavaScript
    let lexer = Lexer::new(
        Syntax::Es(EsSyntax::default()), // Define la sintaxis para ES
        EsVersion::Es2021,               // Versión ES objetivo
        StringInput::from(&*fm),         // Fuente de entrada
        None,
    );

    // Analiza el código JavaScript en un AST
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error al analizar JS: {:?}", e),
        )
    })?;

    // Emite y minifica el código JavaScript
    let mut buf = vec![];
    let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
    let mut emitter = Emitter {
        cfg: Default::default(),
        cm,
        comments: None,
        wr: writer,
    };

    // Emite el código como JavaScript desde el AST
    emitter.emit_module(&module).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Error al emitir JS: {:?}", e),
        )
    })?;

    // Convierte el buffer a una cadena UTF-8 y la devuelve
    String::from_utf8(buf).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Salida UTF-8 inválida: {}", e),
        )
    })
}
