#![allow(warnings)]

fn main() {
	//defining a struct to use for this lab
	struct Car {
		make: String,
		model: String,
		year: u16,
		all_wheel: bool,
	}

	//using the defined struct creating an object
	let mut alices_car = Car {
		make: String::from("Jeep"),
		model: String::from("Wrangler JK"),
		year: 2018,
		all_wheel: true,
	};
	println!("the make of alice's car is {0}", alices_car.make);
}
