use builtin;
use datum::Datum;
use environment::Environment;
use error::RuntimeError;
use lexer::Lexer;
use parser::Parser;
use repl;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use vm::VirtualMachine;

pub struct Interpreter {
    root: Rc<RefCell<Environment>>
}

impl Interpreter {
    pub fn new() -> Self {
        // Define the built-in procedures.
        let mut root = Environment::new();
        for (name, datum) in builtin::get_builtins() {
            root.define(name, datum);
        }
        let interp = Interpreter {root: Rc::new(RefCell::new(root))};
        interp.evaluate(include_str!("core.scm"))
            .expect("Error in the core scheme library");
        interp
    }
    pub fn root<'a>(&'a self) -> Ref<Environment> {
        self.root.borrow()
    }
    pub fn root_mut<'a>(&'a mut self) -> RefMut<Environment> {
        self.root.borrow_mut()
    }
    pub fn with_root<F: Fn(&mut Environment)>(&mut self, func: F) {
        use std::ops::DerefMut;
        let mut env = self.root.borrow_mut();
        let mut deref = env.deref_mut();
        func(deref);
    }
    pub fn run_repl(&self) {
        repl::run("> ", |s| {
            let res = try!(self.evaluate(&s));
            Ok(format!("{}", res))
        });
    }
    pub fn evaluate(&self, s: &str) -> Result<Datum, String> {
        // Lex.
        let mut lexer = Lexer::new(s.chars());
        let tokens = match lexer.lex_all() {
            Ok(t) => t,
            Err(e) => return Err(e.msg)
        };

        // Parse.
        let mut parser = Parser::new(tokens.into_iter());
        let data = match parser.parse_all() {
            Ok(d) => d,
            Err(e) => return Err(e.msg)
        };

        if data.len() == 0 {return Err("".to_string());}

        // Evaluate.
        let mut res = Datum::EmptyList;
        for datum in data {
            res = match self.evaluate_datum(&datum) {
                Ok(d) => d,
                Err((e, trace)) =>
                    return Err(format!("{}\n\nStack trace:\n{}", e.msg, trace))
            }
        }
        Ok(res)
    }
    pub fn evaluate_datum(&self, datum: &Datum) ->
        Result<Datum, (RuntimeError, String)>
    {
        let mut vm = VirtualMachine::new();
        vm.run(self.root.clone(), datum)
    }
}
