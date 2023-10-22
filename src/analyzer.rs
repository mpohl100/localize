use swc_ecma_ast::{
    BlockStmt, BlockStmtOrExpr, Decl, Expr, JSXElement, JSXElementChild, JSXExpr, JSXFragment, Lit,
    Module, ModuleItem, Stmt,
};

pub struct AnalysisResult {
    pub strings: Vec<String>,
    pub template_strings: Vec<String>,
    pub jsx_texts: Vec<String>,
}

pub struct Analyzer {
    pub result: AnalysisResult,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            result: AnalysisResult {
                strings: Vec::new(),
                template_strings: Vec::new(),
                jsx_texts: Vec::new(),
            },
        }
    }

    pub fn analyze_module(&mut self, module: &Module) {
        for item in &module.body {
            match item {
                ModuleItem::Stmt(statement) => self.analyze_statement(statement),
                ModuleItem::ModuleDecl(_) => {}
            }
        }
    }

    fn analyze_statement(&mut self, statement: &Stmt) {
        match statement {
            // If the statement is a function (traditional function component)
            Stmt::Decl(Decl::Fn(fn_decl)) => {
                self.analyze_block_statement(fn_decl.function.body.clone().unwrap());
            }
            // If the statement is a variable declaration, it might be a functional component.
            Stmt::Decl(Decl::Var(var_decl)) => {
                for var_declarator in &var_decl.decls {
                    // Check if the variable declarator's init is an arrow function or any function expression.
                    if let Some(init_expr) = &var_declarator.init {
                        match &**init_expr {
                            Expr::Arrow(arrow_expr) => match &*arrow_expr.body {
                                BlockStmtOrExpr::BlockStmt(block_stmt) => {
                                    self.analyze_block_statement(block_stmt.clone());
                                }
                                BlockStmtOrExpr::Expr(expr) => {
                                    self.analyze_expression(expr);
                                }
                            },
                            Expr::Fn(fn_expr) => {
                                self.analyze_block_statement(
                                    fn_expr.function.body.clone().unwrap(),
                                );
                            }
                            // other arms => TODO!
                            _ => {}
                        }
                    }
                }
            }
            Stmt::Return(return_stmt) => {
                if let Some(return_expr) = &return_stmt.arg {
                    self.analyze_expression(return_expr);
                }
            }
            Stmt::Expr(expr) => {
                self.analyze_expression(&expr.expr);
            }
            // other arms => TODO!
            _ => {}
        }
    }

    fn analyze_block_statement(&mut self, block_stmt: BlockStmt) {
        // If block statement, analyze each statement in the block.
        let block = block_stmt;
        for stmt in &block.stmts {
            self.analyze_statement(stmt);
        }
    }

    fn analyze_expression(&mut self, expression: &Expr) {
        match expression {
            Expr::Paren(par) => self.analyze_expression(&par.expr),
            Expr::Lit(lit) => {
                match lit {
                    Lit::Str(string_literal) => {
                        self.result.strings.push(string_literal.value.to_string());
                    }
                    // TODO! -> handle other kinds of literals
                    _ => {}
                }
            }
            Expr::Tpl(template_literal) => {
                // If it's a template literal, save the "quasis" (the string parts)
                for quasi in &template_literal.quasis {
                    self.result.template_strings.push(quasi.raw.to_string());
                }
            }
            Expr::JSXElement(jsx_element) => self.analyze_jsx_element(jsx_element),
            Expr::JSXFragment(jsx_fragment) => self.analyze_jsx_fragment(jsx_fragment),
            // TODO! -> handle other kinds of expressions
            _ => {}
        }
    }

    fn analyze_jsx_fragment(&mut self, jsx_fragment: &JSXFragment) {
        // Traverse the JSX fragment's children
        for child in &jsx_fragment.children {
            match child {
                JSXElementChild::JSXText(text) => {
                    self.result.jsx_texts.push(text.value.to_string())
                }
                JSXElementChild::JSXElement(el) => self.analyze_jsx_element(el),
                JSXElementChild::JSXFragment(frag) => self.analyze_jsx_fragment(frag),
                JSXElementChild::JSXExprContainer(_) => todo!(),
                JSXElementChild::JSXSpreadChild(_) => todo!(),
            }
        }
    }

    fn analyze_jsx_element(&mut self, jsx_element: &JSXElement) {
        for child in &jsx_element.children {
            match child {
                JSXElementChild::JSXText(text) => {
                    let trimmed_text = text.value.trim();

                    if !trimmed_text.is_empty() {
                        self.result.jsx_texts.push(trimmed_text.to_string());
                    }
                }
                JSXElementChild::JSXElement(el) => self.analyze_jsx_element(el),
                JSXElementChild::JSXExprContainer(expr_container) => {
                    // Expression container -> there's a JavaScript expression within the JSX.
                    match &expr_container.expr {
                        JSXExpr::JSXEmptyExpr(_) => {}
                        JSXExpr::Expr(expr) => self.analyze_expression(&expr),
                    }
                }
                JSXElementChild::JSXSpreadChild(_) => todo!(),
                JSXElementChild::JSXFragment(_) => todo!(), // handle other cases as necessary
            }
        }

        // for attr in &jsx_element.opening.attrs {
        //     todo!()
        // }
    }
}
