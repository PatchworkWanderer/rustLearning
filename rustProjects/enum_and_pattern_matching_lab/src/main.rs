#![allow(warnings)]

fn main() {
	//modules
	basic_enum();
	enums_with_values();
	enum_data_types();
	option_enum();

}

fn basic_enum() {
	enum IpAddrKind { //declaring an enum that contains the possible 
		V4,   		            //varients associated with IP version
		V6,
	}

	struct IpAddr {
		kind: IpAddrKind, // we can use enums as a data type here as well
		address: String,
	}

	let four = IpAddrKind::V4; //declaring two instances of the enum type
	let six = IpAddrKind::V6;   //both four & six are of the same type
                                               //making them usable as a type in functions
     
    let home = IpAddr {        
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
     
    let loop_back = IpAddr {
        kind: IpAddrKind::V6,
    	address: String::from("::1"),
     };
     
     println!("\nthe home address is: {} \nthe loop back address is {}"
			  ,home.address, loop_back.address);
}

fn enums_with_values() {
	#[derive(Debug)] //just to see the values :3
	enum IpAddr   { //declaring the same enum as the previous example 
		V4(String), //but making it so that each varient has a String 
		V6(String), //value associated with the varient type.
		
		V4Octets(u8,u8,u8,u8), //made so that instead of a string 
							   //its 4 ints that function as the 
							   //the octets of an IPv4 addr 
	}

	let home = IpAddr::V4(String::from("127.0.0.1"));
	let loop_back = IpAddr::V6(String::from("::1"));

	let local_addr = IpAddr::V4Octets(10,42,0,69); //hehe funny numbers

	println!("\n\nthe value of local_addr is: {:?}",local_addr);

}

fn enum_data_types() {
	#[derive(Debug)]
	enum Message {//copy of the message enum from the standard library
				  //that shows more obscure data types that enum can take

		Quit, // no data associated with
		Move {x: i32, y:i32}, //has named fields like structs
		Write(String), // includes a single string
		ChangeColor(i32,i32,i32) // includes three i32 values
	}

	impl Message {
		fn call(&self) {
			println!("\n\n{:?}\n", self);
		}
	}
	
	let m = Message::Write(String::from("enums can also have their own methods"));
	m.call();
}


///////////////////////////[Option enum]////////////////////////////////////////

fn option_enum() {
	//below is the Option enum as defined in the standard library
	//techincally its included in the prelude making it within scope
	//even without explicitly bringing it into scope. same with the 
	//varients Some(T) & None
	enum Option<T> {
		None,
		Some(T),
	}

	let num1 = Some(5); //sets the type as Option<i32>
	let char1 = Some('e'); //sets the type as option<char>

	let absent_num: Option<i32> = None; //tells to set the type as Option<i32>
										//but keeps the value as Non



}







