use swc_common::{
    // errors::{ColorConfig, Handler},
    FileName,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

// Function to parse TypeScript file and return the AST
pub fn parse_tsx_file(file_content: &str) -> swc_ecma_ast::Module {
    // Create a SourceMap
    let source_map = SourceMap::default();
    let source_file = source_map.new_source_file(
        FileName::Custom("file.tsx".to_string()),
        file_content.to_string(),
    );

    // Create an error handler
    // let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, None);

    // Create a lexer
    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: false,
            dts: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*source_file),
        None,
    );

    // Create a parser
    let mut parser = Parser::new_from(lexer);

    // Parse the script
    let module = parser.parse_module().unwrap();

    module
}
