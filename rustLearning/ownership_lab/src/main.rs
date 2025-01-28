fn main() {
	//creating a string that allocates data on the heap making it mutable 
	let mut s = String::from("hello"); // s is now in scope

	s.push_str(", world!"); //appends a litteral to the string

	println!("{s}"); 



} //scope is now over and s is no longer valid
