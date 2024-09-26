use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::FoldWith;

/// Minifica el código JavaScript compatible con ES2023 usando SWC.
#[pyfunction]
pub fn minify_js_code(js_code: &str) -> PyResult<String> {
    let cm = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(FileName::Custom("input.js".into()), js_code.into());

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        swc_ecma_ast::EsVersion::Es2022,
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser
        .parse_module()
        .map_err(|e| PyValueError::new_err(format!("Error al parsear JS: {:?}", e)))?;

    let minifier = swc_ecma_minifier::optimize(
        module,
        cm.clone(),
        None,
        None,
        &swc_ecma_minifier::option::MinifyOptions::default(),
        &swc_ecma_minifier::option::ExtraOptions {
            top_level_mark: swc_common::Mark::fresh(swc_common::Mark::root()),
            unresolved_mark: swc_common::Mark::fresh(swc_common::Mark::root()),
        },
    );

    let mut buf = vec![];
    {
        let mut emitter = swc_ecma_codegen::Emitter {
            cfg: swc_ecma_codegen::Config { minify: true },
            cm: cm.clone(),
            comments: None,
            wr: Box::new(swc_ecma_codegen::text_writer::JsWriter::new(
                cm.clone(),
                "\n",
                &mut buf,
                None,
            )),
        };

        minifier
            .emit_with(&mut emitter)
            .map_err(|e| PyValueError::new_err(format!("Error al generar JS: {:?}", e)))?;
    }

    String::from_utf8(buf)
        .map_err(|e| PyValueError::new_err(format!("Error de codificación: {}", e)))
}
