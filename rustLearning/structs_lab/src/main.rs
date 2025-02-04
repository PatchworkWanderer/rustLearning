#![allow(warnings)]

// to allow other functions to use structs they must be declared globaly 
struct User {
		active: bool,
		username: String,
		email: String,
		sign_in_count: u64,
}	

//defining structs to use for this lab
struct Car {
	make: String,
	model: String,
	year: u16,
	all_wheel: bool,
}

//unit like struct
struct AlwaysEqual;


fn main() {

	//using the defined struct creating an object
	let mut alices_car = Car {
		make: String::from("Jeep"),
		model: String::from("Wrangler JK"),
		year: 2018,
		all_wheel: true,
	};

	//creation of template user
	let user_template = User {
		active: true,
		username: String::from("someusername"),
		email: String::from("another@example.com"),
		sign_in_count: 1,
	};
	
	//using the template to fill out generic data
	let user2 = User {
		username: String::from("another_username"),
		..user_template
	};

	//println!("the make of alice's car is {0}", alices_car.make);
	build_user(String::from("something@something.com"), String::from("bingobongo"));

	//example fn section
	handeler();
	handeler_tuple();
	handeler_structs();

}

fn build_user(email: String, username: String) -> User {
	//showcase to demonstrate field init while using structs
	User {
		active: true,
		username, //because we are passing username and email as parameters
		email,    //we can skip the repititon if the struct fields match with
		sign_in_count: 1, //the parameters passed
	}

}


/*
fn build_user(email: String, username: String) -> User {
	//if ran this fn will function the same as the uncommentated one but 
	//doesn't use field init shorthand.
	User {
		active: true,
		username: username,
		email: email,   
		sign_in_count: 1, 
	}

*/

fn tuple_structs() {
	//defines how to declare a tuple struct
	struct Color(u8, u8, u8);
	struct Point(i32, i32, i32);

	let red = Color(255,0,0);
	let origin = Point(0,0,0);
}

fn unit_like_structs() {
	let subject = AlwaysEqual;
}

//////////////////[example program w/o structs]//////////////////////////
fn handeler() {
	let width: u32 = 30;
	let height: u32 = 50;
	println!("the area of the square is {} pixles", area(width, height))
}

fn area (width: u32, height: u32) -> u32{
	width*height
}

///////////////////[example with tuples]/////////////////////////////////

fn handeler_tuple() {
	let rectangle = (30,50);
	println!("the area of the square is {} pixles", area(rectangle.1,rectangle.0));

}

///////////////////[example with structs]////////////////////////////////

struct Rectangle {
	height: u32,
	width: u32
}

fn handeler_structs() {
	let rect1 = Rectangle {
		width: 69,
		height: 69,
	};

	println!("the area of the square is {} pixles", area(rect1.width, rect1.height));

}




