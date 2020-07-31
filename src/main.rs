extern crate swc_common;
extern crate swc_ecma_parser;

use std::{path::Path, sync::Arc};
use swc_atoms::JsWord;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_ast::{Decl, Module, ModuleItem, Pat, Stmt};
use swc_ecma_parser::{lexer::Lexer, Parser, SourceFileInput, Syntax};

fn main() {
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
        let cm: Arc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        let fm = cm
            .load_file(Path::new("test.js"))
            .expect("failed to load test.js");

        let lexer = Lexer::new(
            // We want to parse ecmascript
            Syntax::Es(Default::default()),
            // JscTarget defaults to es5
            Default::default(),
            SourceFileInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        for e in parser.take_errors() {
            e.into_diagnostic(&handler).emit();
        }

        let module = parser
            .parse_module()
            .map_err(|e| {
                // Unrecoverable fatal error occurred
                e.into_diagnostic(&handler).emit()
            })
            .expect("failed to parser module");

        if let Ok(identifier_name) = get_identifier_name(&module) {
            println!("identifier name: {}", identifier_name);
        }
    });
}

fn get_identifier_name(module: &Module) -> Result<&JsWord, ()> {
    for module_item in &module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) = module_item {
            for decl in &var.decls {
                if let Pat::Ident(identifier) = &decl.name {
                    return Ok(&identifier.sym);
                }
            }
        }
    }
    Err(())
}
