fn main() {
	//function calls
	function_two();
	parameter_fn(6);
	multi_arguments(34.12, 's');

	// statement examples
		let state1 = "bingbong";
		let state2 = 23;

	/*statements can't returne values like the following
	let invalidstate = (let x = 9);

	the code above will throw an error calling it unstable because the (let x = 9)
	does not return a value making the variable invalidstate empty
	*/

	// expression examples

		// in this expression because the final line does not end in a 
		// semicolon it will return the value 4

		let expres1 = {
			let x = 3;
			x + 1
			};
		println!("the value of the expression is: {expres1}");

	//return value section
	
	//in the followinging block the the variable is set to the return value of the function
	let return_value = return_to_sender();
	println!("the return value of the function is: {return_value}");



}

fn return_to_sender () -> i32 {
	// by using the arrow in the declaration the return value is defined as a signed 32-bit int
	-250
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
