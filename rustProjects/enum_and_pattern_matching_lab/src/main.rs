#![allow(warnings)]

fn main() {
	//modules
	basic_enum();
	enums_with_values();
	enum_data_types();
	option_enum();
	match_flow();
	control_main();
	catch_all_main();
	concise_control_flow();
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
	let six = IpAddrKind::V6;  //both four & six are of the same type
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
	
	let m = Message::Write(String::from(
		"enums can also have their own methods"));
	m.call();
}


///////////////////////////[Option enum]/////////////////////////////////

fn option_enum() {
	//below is the Option enum as defined in the standard library
	//techincally its included in the prelude making it within scope
	//even without explicitly bringing it into scope. same with the 
	//varients Some(T) & None
	
	/*
	enum Option<T> {
		None,
		Some(T),
	}
	*/
	
	let num1 = Some(5); //sets the type as Option<i32>
	let char1 = Some('e'); //sets the type as option<char>

	let absent_num: Option<i32> = None; //tells to set the type as 
										//Option<i32> but keeps the 
										//value as None

	println!("the value of the absent_num is: {:?}\n\n", absent_num);
		//absent_num will return None as there is no valid number here

	let x: i8 = 5;
	let y: Option<i8> = Some(5);

	//let bad_sum = x + y; //this will throw an error 
	//because i8 and Option<i8> are incompatible types

}

///////////////////[Control Flow with match constructs]//////////////////

enum Coin {
	Penny,
	Nickle,
	Dime,
	Quarter(US_state),
}

#[derive(Debug)]
enum US_state { //creating an enum for the US states for the quarters
	Alabama, Alaska, Arizona, Arkansas, California, Colorado,
	Connecticut, Delaware, Florida, Georgia, Hawaii, Idaho,
	Kansas, Kentucky, Louisiana, Maine, Montana, Nebraska,
	Nevada, New_Hampshire, New_Jersey, New_Mexico, New_York,
	North_Carolina, North_Dakota, Ohio, Oklahoma, Oregon,
	Maryland, Massachusetts, Michigan, Minnesota, Mississippi,
	Missouri, Pennsylvania, Rhode_Island, South_Carolina, South_Dakota,
	Tennessee, Texas, Utah, Vermont, Virginia, Washington,
	West_Virginia, Wisconsin, Wyoming,
}

fn control_main(){
	//gives the plus_one fn stuff to do
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None); //demonstrates that none will break the 
							   //match of the plus_one fn stopping it 
							   //at the first arm of the 
	
}

fn match_flow() {
	let mystery_coin = Coin::Quarter(US_state::Washington);
	println!("the value of the coin is: {}", 
		value_in_cents(mystery_coin));
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => { // the expression executed can have multiple 
			             // lines
			println!("Lucky Penny!!");
			1
		}              //when the match finds the right value it will
		Coin::Nickle => 5,  //return the value or execute the expression
		Coin::Dime => 10,   //that is associated with that arm 
		Coin::Quarter(state) => {
			println!("State Quarter from {:?}!", state);
			25
		},
	}
}



fn plus_one(x: Option<i32>) -> Option<i32> {
	//matching with Option<T> 
	match x {
		None => None, //if this line is commented out it will throw an 
					  //error that because we need something to cover
					  //this possibility 
		Some(i) => Some(i+1),
	}
}

///////////////////[Catch-all Patterns and _ placeholder]////////////////


fn catch_all_main(){
	standard_catch_all();
	placeholder_catch_all();
}

fn standard_catch_all() {
	// uses a catch-all while using the value of the catch-all
	let dice_roll = 9;

	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(other),//catch-all must be at the end		
	}
}

fn placeholder_catch_all() {
	//uses the _ placeholder as a catch all without using the value 
	//of the matching arm
	let dice_roll = 10;

	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => reroll(), 
//      _ => (),  //this line can be used if we don't want the code
				  //to run anything if it matches.
	}
}

//erroneous fns that exists so that the examples don't freak out
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}


//////////////[Concise Control Flow with if let]////////////////////////

fn concise_control_flow() {
	let config_max = Some(3u8);

	match config_max {
		Some(max) => println!("The max is configured to be {max}"),
		_ => (),
	}

	let config_min = Some(3u8);

	//same thing as the above match example but using the if let syntax
	if let Some(min) = config_min {
		println!("The min is configured to be {min}");
	}
/*
since we only want the one arm to match and we don't care about
exhaustivly checking all of our options we can use the let if 
syntax to reduce the boilerplate code and beautify the code we use
*/
}


