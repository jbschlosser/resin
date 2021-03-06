use super::*;

#[macro_export]
macro_rules! systest {
    ($input:expr => Error) => {{
        let interp = Interpreter::new();
        match interp.evaluate($input) {
            Ok(d) => panic!("Expected error, got {}", d),
            Err(_) => ()
        }
    }};
    ($input:expr => $output:expr) => {{
        let interp = Interpreter::new();
        match interp.evaluate($input) {
            Ok(d) => assert_eq!($output, format!("{}", d)),
            Err(e) => panic!("Error occurred: {}", e)
        }
    }};
}

#[test]
fn test_basic_evaluation() {
    systest!("5" => "5");
    systest!("#t" => "#t");
    systest!("#f" => "#f");
    systest!("#\\a" => "#\\a");
    systest!("\"hello world\"" => "\"hello world\"");
    systest!("#(1 2 3)" => "#(1 2 3)");
    systest!("'a" => "a");
    systest!("()" => Error);
    systest!("'()" => "()");
    systest!("(quote ())" => "()");
    systest!("'(1 2 3)" => "(1 2 3)");
}

#[test]
fn test_if_form() {
    systest!("(if)" => Error);
    systest!("(if (= 2 2))" => Error);
    systest!("(if (= 2 2) 1 3)" => "1");
    systest!("(if (= 2 4) 1 3)" => "3");
    systest!("(if (= 2 4) 1 3 5)" => Error);
}

#[test]
fn test_let_form() {
    systest!("(let)" => Error);
    systest!("(let ())" => Error);
    systest!("(let ((x 1)))" => Error);
    systest!("(let () 5)" => "5");
    systest!("(let ((x 1)) x)" => "1");
    systest!("(let ((x 1)) 2 3 4 x)" => "1");
    systest!("(let ((a 1) (b 2)) (+ a b))" => "3");
    systest!("(let ((a 1) (b a)) (+ a b))" => Error);
    systest!("(letrec ((a 1) (b a)) (+ a b))" => "2");
}

#[test]
fn test_begin_form() {
    systest!("(begin)" => Error);
    systest!("(begin 1)" => "1");
    systest!("(begin 1 2 3)" => "3");
}

#[test]
fn test_arithmetic() {
    systest!("(+)" => "0");
    systest!("(+ 2)" => "2");
    systest!("(+ 2 3 4)" => "9");
    systest!("(*)" => "1");
    systest!("(* 2)" => "2");
    systest!("(* 3 5 2)" => "30");
    systest!("(-)" => Error);
    systest!("(- 2)" => "-2");
    systest!("(- 5 3)" => "2");
    systest!("(- 5 3 1)" => "1");
    systest!("(=)" => "#t");
    systest!("(= 2)" => "#t");
    systest!("(= 2 2)" => "#t");
    systest!("(= 2 2 2 2)" => "#t");
    systest!("(= 2 2 3 2)" => "#f");
}

#[test]
fn test_car_cdr_cons() {
    systest!("(car)" => Error);
    systest!("(car '(1 . 2))" => "1");
    systest!("(car '(1 2))" => "1");
    systest!("(car '(1 2) '(3 4))" => Error);
    systest!("(cdr)" => Error);
    systest!("(cdr '(1 . 2))" => "2");
    systest!("(cdr '(1 2))" => "(2)");
    systest!("(cdr '(1 2) '(3 4))" => Error);
    systest!("(cons)" => Error);
    systest!("(cons 1)" => Error);
    systest!("(cons 1 2)" => "(1 . 2)");
    systest!("(cons 1 '(2 3 4))" => "(1 2 3 4)");
    systest!("(cons '(1 2) '(3 4))" => "((1 2) 3 4)");
    systest!("(cons 1 2 3)" => Error);
    systest!("(car (cons 1 2))" => "1");
    systest!("(cdr (cons 1 2))" => "2");
}

#[test]
fn test_symbol_string() {
    systest!("(symbol->string 'flying-fish)" => "\"flying-fish\"");
    systest!("(symbol->string 'Martin)" => "\"martin\"");
    systest!("(symbol->string (string->symbol \"Malvina\"))" => "\"Malvina\"");
    systest!("(string->symbol \"mISSISSIppi\")" => "mISSISSIppi");
}

#[test]
fn test_predicates() {
	systest!("(symbol? 'foo)" => "#t");
	systest!("(symbol? (car '(a b)))" => "#t");
	systest!("(symbol? \"bar\")" => "#f");
	systest!("(symbol? 'nil)" => "#t");
	systest!("(symbol? '())" => "#f");
	systest!("(symbol? #f)" => "#f");
}

#[test]
fn test_list() {
    systest!("(list)" => "()");
    systest!("(list 1)" => "(1)");
    systest!("(list 1 2 3)" => "(1 2 3)");
    systest!("(list 'a (+ 3 4) 'c)" => "(a 7 c)");
}

#[test]
fn test_length() {
    systest!("(length)" => Error);
    systest!("(length '(1) '(2))" => Error);
    systest!("(length '(a b c))" => "3");
    systest!("(length '(a (b) (c d e)))" => "3");
    systest!("(length '())" => "0");
}

#[test]
fn test_lambda() {
    systest!("(lambda)" => Error);
    systest!("((lambda ()))" => Error);
    systest!("((lambda (x . y) y) 1)" => "()");
    systest!("((lambda () 1 2 3 4))" => "4");
    systest!("((lambda (x) x) 5)" => "5");
    systest!("((lambda (x y) (+ x y)) 4 5)" => "9");
    systest!("((lambda x x) 1 2 3 4 5)" => "(1 2 3 4 5)");
    systest!("((lambda (x . y) x) 1 2 3 4 5)" => "1");
    systest!("((lambda (x . y) y) 1 2 3 4 5)" => "(2 3 4 5)");
    systest!("((lambda (a b c . d) d) 1 2 3 4 5)" => "(4 5)");
    systest!("((lambda (a . b) b) (+ 1 1) (+ 2 2) (+ 3 3))" => "(4 6)");
}

#[test]
fn test_append() {
    systest!("(append)" => "()");
    systest!("(append '())" => "()");
    systest!("(append '(1 2))" => "(1 2)");
    systest!("(append '(1 2) '(3 4))" => "(1 2 3 4)");
    systest!("(append '(a b) '(c . d))" => "(a b c . d)");
    systest!("(append '(x) '(y))" => "(x y)");
    systest!("(append '(a) '(b c d))" => "(a b c d)");
    systest!("(append '(a (b)) '((c)))" => "(a (b) (c))");
    systest!("(append '() 'a)" => "a");
    systest!("(append '() 5)" => "5");
}

#[test]
fn test_apply() {
    systest!("(apply)" => Error);
    systest!("(apply +)" => Error);
    systest!("(apply '())" => Error);
    systest!("(apply '(1 2 3))" => Error);
    systest!("(apply + '(1 2 3))" => "6");
    systest!("(apply + '())" => "0");
    systest!("(apply + 4 '(1 2 3))" => "10");
    systest!("(apply + 4 5 6 '(1 2 3))" => "21");
    systest!("(apply + '(4 5 6) '(1 2 3))" => Error);
    systest!("(apply + '((+ 1 1) 2 3))" => Error);
}

#[test]
fn test_map() {
    systest!("(map)" => Error);
    systest!("(map +)" => Error);
    systest!("(map (lambda (x) (* x x)) '(1 2 3 4))" => "(1 4 9 16)");
    systest!("(map + '(1 2 3 4) '(2 3 4 5))" => "(3 5 7 9)");
}
