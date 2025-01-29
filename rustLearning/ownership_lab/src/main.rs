fn main() {
	//creating a string that allocates data on the heap making it mutable 
	let mut s = String::from("hello"); // s is now in scope

	s.push_str(", world!"); //appends a litteral to the string

	println!("{s}"); 
	
	//moving_variables();
	deep_copy();

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
