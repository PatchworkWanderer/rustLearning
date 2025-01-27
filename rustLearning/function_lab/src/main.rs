fn main() {
	function_two();
	parameter_fn(6);
	multi_arguments(34.12, 's');


}

fn function_two() {
	println!("function 2 here");
}

fn parameter_fn(x: i32) {
	println!("the value passed to this function is {x}");
}

fn multi_arguments(float1: f64, char1: char){
	println!("value of float passed is: {float1}. The value of the char passed is: {char1}");
}
