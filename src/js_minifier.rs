// core_reactpyx/src/js_minifier.rs

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use swc_common::sync::Lrc;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_visit::VisitMutWith;

/// Minifica el código JavaScript compatible con ES2023 usando SWC.
///
/// Args:
///     js_code (str): El código JavaScript a minificar.
///
/// Returns:
///     str: El código JavaScript minificado.
#[pyfunction]
#[pyo3(text_signature = "(js_code)")]
pub fn minify_js_code(js_code: &str) -> PyResult<String> {
    let cm = Lrc::new(swc_common::SourceMap::default());
    let fm = cm.new_source_file(
        swc_common::FileName::Custom("input.js".into()),
        js_code.into(),
    );

    let lexer = Lexer::new(
        Syntax::default(),
        swc_common::EsVersion::Es2023,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser
        .parse_module()
        .map_err(|e| PyValueError::new_err(format!("Error al parsear JS: {}", e)))?;

    let mut minified_module = optimize(module, Default::default());
    minified_module.visit_mut_with(&mut fixer(None));

    let mut buf = vec![];
    let mut emitter = swc_ecma_codegen::Emitter {
        cfg: swc_ecma_codegen::Config { minify: true },
        cm: cm.clone(),
        wr: Box::new(swc_ecma_codegen::text_writer::JsWriter::new(
            cm, "\n", &mut buf, None,
        )),
        comments: None,
    };

    emitter
        .emit_module(&minified_module)
        .map_err(|e| PyValueError::new_err(format!("Error al generar JS: {}", e)))?;

    Ok(String::from_utf8(buf)
        .map_err(|e| PyValueError::new_err(format!("Error de codificación: {}", e)))?)
}

/// Módulo de Python para la minificación.
#[pymodule]
fn my_minifier(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    Ok(())
}

fn optimize(
    module: swc_ecma_ast::Module,
    options: swc_ecma_transforms_base::optimizer::OptimizerOptions,
) -> swc_ecma_ast::Module {
    let unresolved_mark = swc_common::Mark::new();
    let top_level_mark = swc_common::Mark::new();

    swc_ecma_transforms_base::optimizer::optimize(
        &unresolved_mark,
        &top_level_mark,
        module,
        options,
    )
}
