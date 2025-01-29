fn main() {
	//creating a string that allocates data on the heap making it mutable 
	let mut s = String::from("hello"); // s is now in scope

	s.push_str(", world!"); //appends a litteral to the string

	println!("{s}"); 
	
	// below are functions that demonstrate various concepts on ownership
	// and the difference between data stored on the stack vs. heap
	// uncomment any of them to run.

	//moving_variables();
	//deep_copy();
	//stack_copy();
	//ownership();
	return_ownership();




} 
// at the ending curly bracket rust calls the drop function 
// where the author of string can put the code to return the memory

// scope is now over and s is no longer valid and the memory
// assigned to it is released.

fn moving_variables() {

	// x & y are both the same and are set to the same size
	// cause they are a know fixed size and are pushed to the stack
	let _x = 5;
	let _y = _x;

	// str2 doesn't copy just the value from str1 but the 
	// ptr, len, and capacity of str1 because it is allocated to the heap
	let str1 = String::from("hello");
	let str2 = str1; //after declaring str2 rust will consider str1 invalid
	
	println!("{str2}"); 
		//attempting to run str1 here will cause a compilation error
}

fn deep_copy() {
	// the clone method allows the variable str2 to copy the data
	// from str1 directly from the heap
	let str1 = String::from("hello");
	let str2 = str1.clone();

	println!("str1 = {str1}, str2 = {str2}");

}

fn stack_copy() {
	// with data on the stack since they are know in size it is
	// possible to copy all the data from the variables directly from
	// the stack. by using the copy method secretly	

	let int1 = 5;
	let int2 = int1;

	println!("int1 = {int1}, int2 = {int2}");	

}

//////////////////////////////////////////////////////////////////////////

fn ownership() {
	let str1 = String::from("hello"); // str1 comes into scope and is
									  // allocated on the heap
	
	take_ownership(str1);			  // str1's value is moved into the
									  // function and is no longer valid
									  // here

	//println!("{str1}");             // this will cause rust to throw a
									  // compilation because of the move

	let int1 = 5;					  // int1 comes into scope

	make_copy(int1);				  // since int1 is able to implement
									  // the copy method int1 is still 
									  // able to be used here

	println!("{int1}");  		      // since int1 was only copied it is
									  // still able to be used in scope

} // int1 goes out of scope, then str1. However, since str1's value was
  // moved, nothing special happens 

fn take_ownership(str2: String) { //str2 comes into scope
	println!("{str2}");
} //str2 goes out of scope and 'drop' is called freeing the memory

fn make_copy(int2: i32) { // int2 is called into scope
	println!("{int2}");
} // int2 goes out of scope, nothing special happens

//////////////////////////////////////////////////////////////////////////

fn return_ownership() { 
	let str1 = gives_ownership();	// gives_ownership moves its return
									// value into str1

	let str2 = String::from("hello"); //str2 comes into scope

	let str3 = takes_and_gives_back_ownership(str2); // str2 is moved into
								    // the function, which also moves its 
									// value into 
	println!("{str1}");
}
// str3 goes out of scope and is dropped, str2 was moved so nothing
// happens, and str1 goes out of scope and is dropped.

fn gives_ownership() -> String {
//fn will move its return value into the function that calls it
	let str4 = String::from("this is from inside the function"); 
		// str4 comes into scope

	str4 // str4 is returned and is moved out to the calling fn	
}

fn takes_and_gives_back_ownership(str5: String) -> String {
// fn will move the value from the parameter passed into the fn
// and return a string into another variable
	str5
}

///////////////////////////////////////////////////////////////////////





























