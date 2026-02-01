mod variables;
mod functions;


fn main() {
    variables::var::variable_i();
    println!("result is:{}",functions::fun_style::take_something_return_something(1, 2));
}
