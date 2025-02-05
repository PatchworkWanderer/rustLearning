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
	handeler_methods();

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
#[derive(Debug)] // placing this here will allow us to print Rectangle
				 // structs
struct Rectangle {
	height: u32,
	width: u32
}



fn handeler_structs() { // now ft. the dbg! macro
	let scale = 2;
	let rect1 = Rectangle {
		width: dbg!(30 * scale), //calls the debug macro which will
		height: 69,				 //print the expression in () as well
	};							 //as what width equates to
	
	dbg!(&rect1); // by calling the macro here we are able to see what
				  // the value of the specified field
	  			  // since we didn't want dbg! to take ownership we used
				  // a referenced
	
	println!("the area of the square is {} pixles", area_structs(&rect1));
	println!("\nthe values of the rectangle are {:?}", rect1);
		// printing a struct to see its values
		// the option of {:?} or {:#?} are available 

}

fn area_structs(rect: &Rectangle) -> u32 {
	rect.height * rect.width
}

/////////////////////////////[method area]///////////////////////////////
#[derive(Debug)]
struct Triangle { //basic struct to work with
	base: f32,
	height: f32,
}

impl Triangle { // impl tells rust to define the following functions
				// with in the context of the struct Triangle

	fn area(&self) -> f32 { //&self passes a reference of the type into 
							//the method.
		self.base * self.height * 0.5
	}

	fn can_hold(&self,tri2: &Triangle) -> bool {
		if self.base > tri2.base && self.height > tri2.height {
			return true;
		} else {
			return false;
		}
	}

	fn triangle(size: f32) -> Self { //this is an associated function as it doesn't
									 //reference self as a parameter
		Self {
			base: size,
			height: size,
		}
	}

}

fn handeler_methods() {
	let tri1 = Triangle{
		base: 10.0,
		height: 15.0,
	};

	let tri2 = Triangle{
		base: 5.0,
		height: 10.0,
	};

	let tri3 = Triangle::triangle(6.0); //calling the associated function
										//to construct a new instance of Triangle

	println!("\n\nthe area of the triangle is {} square pixels", tri1.area());
	println!("\n\ncan tri1 hold tri2: {}", tri1.can_hold(&tri2));
	println!("\n\ncan tri2 hold tri1: {}", tri2.can_hold(&tri1));
	println!("the value of tri3 is: {:?}", tri3);

}






