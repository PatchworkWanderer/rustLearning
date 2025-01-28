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

	// loop section
	// uncomment each function below to test
		//passing_value_loop();
		//loop_types(30);
		//loop_labels();
		//while_loop();
		for_loop();
}

fn passing_value_loop() {
	//passing the value of a loop to the rest of the code
	let mut counter = 0;
	let result = loop {
		counter +=1;

		if counter == 10 {
			break counter * 2;
		}
	};
	println!("The result of the loop is {result}");
}

fn loop_types(loop_iterations: u16) {
	// starts a loop that will break after counter == loop_iterations
	let mut counter = 0;
	loop {
		counter +=1;
		
		if counter % 10 == 0 {
			println!("this is a multiple of 10");
		} else {
			println!("this is loop: {counter}");
		}

		if counter == loop_iterations {
			break
		}
	}
}

fn loop_labels() {
	// function to demonstrate usage of loop labels
	let mut i = 0;
	//'counting_up is the loop label
	'counting_up: loop{
		println!("count = {i}");
		let mut remaining = 10;
		
		loop {
			println!("remaining = {remaining}");
			if remaining == 9 {	
				break; // this break will only exit the inner loop
			 }
			if i == 2 {
				break 'counting_up; //using the loop label to break here will exit the outer most loop leaving i == 2
			}
			remaining -=1;
		}
		i +=1;
	}
	println!("End count = {i}");
}


fn while_loop() {
	// demonstrates the use of while loops
	let mut i = 10;
	
	while i != 0 {
		println!("{i}");
		i-=1;
	}
	println!("Lift off!!");
}

fn for_loop() {
	// demonstrates the use of for loops primairly used for iterating through a list
	let simple_array = [10,20,30,40,50,60,70,80,90,100];

	for element in simple_array {
		println!("the value is: {element}");
	}
}



























