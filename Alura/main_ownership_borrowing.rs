fn main(){
    ownership()
    
}

/* Esse código vai apresentar erro de ownership
fn ownership(){
	let uma_string = String::from("Renato");
	rouba(uma_string);

	println!("{}", uma_string);
}

fn rouba(string: string){
	println!("{}", string);
}
*/

// código corrigido passando uma referência
/* 
fn ownership(){
	let uma_string = String::from("Renato");
	rouba(&uma_string);

	println!("{}", uma_string);
}

fn rouba(string: &String){
	println!("{}", string);
}
*/

// código corrigido passando uma referência mutável
fn ownership(){
	let mut uma_string = String::from("Renato");
	rouba(&mut uma_string);

	println!("{}", uma_string);
}

fn rouba(string: &mut String){
	string.push_str("Pires");
	println!("{}", string);
}

