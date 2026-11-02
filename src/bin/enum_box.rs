// 1. Rewrite this enum to use references instead of Box.
// 2. Make the example compile and run
// 3. Make use of the build_add_exp function to build the expression

enum Expr {
    Val(i64),
    Add(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Val(i) => *i,
        Expr::Add(l, r) => eval(l) + eval(r),
    }
}

fn build_add_exp(l: i64, r: i64) -> Expr {
    Expr::Add(Box::new(Expr::Val(l)), Box::new(Expr::Val(r)))
}

fn main() {
    let t = Expr::Add(Box::new(Expr::Val(1)), Box::new(Expr::Val(2)));
    // let t = build_add_exp(1, 2);
    println!("{}", eval(&t));
}


// Explanation:
// `l` and `r` are of type `&Box<Expr>` in the `Expr::Add` match arm.
// Box implements the Deref trait (Box<T> implements Deref<Target = T>).
// Rust automatically applies deref coercion.
// We could write it manually like `eval(&**l)`