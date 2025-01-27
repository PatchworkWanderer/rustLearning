fn main() {
	//declaring vars

	//ints
	let int1 :i16 = -30;
	println!("this is a signed 16-bit int {int1}");

	//floats
	let float1 = 64.423;
	println!("this is a float {float1}");

	//Booleans
	let is_true :bool = true;
	println!("this is {is_true}");

	//char
	let char1 = 'z';
	println!("the secret char is {char1}");

	// Compount variable types

	// Tuples
	let tup1 = (500, 6.4, 1);
	println!("this is the first tuple {:?}", tup1);

	let (x,y,z) = tup1;
	println!("the value of index 1 is {y}");

	let tup2: (i64, f64, u8) = (455, 23.4, 5);
	println!("this is the value of the second tuple {:?}", tup2);
	
	println!("the second value of the second tuple is {:?}", tup2.2);

	//Arrays
	let mut array1 = [1,2,3,4,5];
	println!("this is the first array {:?}", array1);
	array1[0] += 1;
	println!("this is the first element of the array plus 1 {:?}", array1[0]);

	//initalizing an array with kown size and a constant value
	let array2 = [5;6];
	println!("this is an array of six fives {:?}", array2);

	//rust doesn't allow you to use -1 to access the last element of an array
	







}
