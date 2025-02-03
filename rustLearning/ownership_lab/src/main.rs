#![allow(warnings)] // stops annoying warnings about unused vars
					// and functions from this project

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
	//return_ownership();
	//intro_reference();
	slice_lab();


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

//////////////////////////[basics of ownership]//////////////////////////

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

///////////////////////[basics of references]//////////////////////////////

fn intro_reference() {

	let str1 = String::from("this str is used for reference"); 
		// str1 allocated to the heap
	let str1_len = calc_len(&str1); 
		// passes the reference of str1 to calc_len
	println!("The length of '{str1}' is: {str1_len}");
		// because the value of str1 was not droped it can be used

	let mut str2 = String::from("Hello,");
		//creation of a new mut str to demonstrate a mut reference
	mutable_reference(&mut str2);
		//passes the reference to the function where the value of 
		//str2 will be changed
	println!("{str2}"); //prints the updated str2 

//functions to showcase uses of references and what is impossible to do
//with them

	//multiple_reference_type_use();
	//dangling_reference();
	
}

fn calc_len(s: &String) -> usize{ // since a reference of str1 was 
                                  // passed the type has to be a 
								  // string reference
	s.len()
} // the value of s goes out of scope but since it doesn't have 
  // ownership of that value it isn't dropped
/*
  the '&' used in the above section is to signify that we want to use
  the refernce of the variable rather that the variable itself 
  using the variable without taking ownership of the variable.
  _________________________
  |    s      |    str1    |________________ 
  |name|value | name|value | index  | value |
  |ptr |  x ->| ptr |  x ->|   0    |    h  |
  ------------| len |  5   |   1    |    e  |
              | cap |  5   |   2    |    l  |
              -------------|   3    |    l  |
                           |   4    |    o  |
                           ------------------ 
*/
fn mutable_reference(s: &mut String) {
	s.push_str(" world!"); // changes the value of the reference 
						   // owner and exits
}

fn multiple_reference_type_use() {
// fn shows how to use immutable and mutable references together

	// combining immutable and mutable references while
	// all references are active
	let mut str1 = String::from("hello, world!");

	let str1r1 = &str1; //no problem
	let str1r2 = &str1; //no problem
	//let str1r3 = &mut str1; //big problem
	//println!("{str1r1},{str1r2},{str1r3}");//will throw an error 
										   //for combining multiple
										   //types of references

	//the above can be fixed by using the immutable references
	//first then making the mutable one
	println!("{str1r1},{str1r2}"); //vars will no longer be used

	let str1r3 = &mut str1; //no problem declaring a mutable ref
						    //as all immutable refs are already used
	println!("{str1r3}");
}

fn dangling_reference() {
	//trying to create a dangling reference to show
	//that rust will prevent it
	
	let ref_to_nothing = dangle();//s is dropped when dangle()
								  //is returned ie nothing exits
}

fn dangle() -> String {
	let s = String::from("hello");
	
	//&s
	s
}//value of &s goes out of scope and is dropped
 //could return s and it would work though

//////////////////////////[Basics of Slices]////////////////////////////////

fn slice_lab() {
	
//functions to showcase uses of slices and what 
//can be done with them
	let mut str_slices = String::from("hello world!");
	let word = getting_word_best_ver(&str_slices);
			// will get the first word of the string can take 
			// string literals or slices of a string
	let word_place = getting_word_index(&str_slices);
			// word_place will get the value of 5
	//let word = &str_slices[..word_place];	
			// makes a slice of str_slices from index 0 to 5

//	str_slices.clear(); // clears the string making it == ""
		// word still has the value 5 but no string to meanigfully 
		// use it on. making it totally invalid
	println!("{word_place}");
	println!("{word}");
	// if we wanted to get the first word with slices we could use
	// the above to take the len of the first word and feed it 
	// into a slice to grab the word

}

fn getting_word_index(s: &String) -> usize{
//creating a way to return the index of a word, indicated by a space
	let bytes = s.as_bytes(); //converting str to an array of bytes

	for (i, &item) in bytes.iter().enumerate() { //iterator going 
			   							// through each element of the 
										// byte array created
		if item == b' ' {// looking for the byte value of space
			return i;    // returning the index of the space
		}
	}
	s.len()
}

fn getting_word_best_ver(s: &str) -> &str{
//creating a way to return just the slice of the word, indicated by a space
	let bytes = s.as_bytes(); //converting str to an array of bytes

	for (i, &item) in bytes.iter().enumerate() { //iterator going 
										// through each element of the 
										// byte array created
		if item == b' ' {// looking for the byte value of space
			return &s[0..i];    // returning the index of the space
		}
	}
	&s[..]
}











