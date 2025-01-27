fn main() {

	// simple multiple condition if else statements
    let number1 = 40;

	if number1 < 40 {
		println!("number1 is less than 40!");
	} else if number1 > 40 {
		println!("number1 is greater than 40");
	} else {
		println!("number1 is equal to 40");
	}

	// if expressions can be used in a let statement to assign the outcome to a variable
	let example_condition = false;
	
	//if the else value is changed to a str value a compilation error will be thrown
	let conditional_number = if example_condition {6} else {9};
	println!("the conditional number is: {conditional_number}");

}
