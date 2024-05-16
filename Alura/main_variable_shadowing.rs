fn shadowing() {

	let a = 123;
		{
		let a = 777;
		println!("a de dentro = {}", a);
		}
	println!("a de fora = {}", a);

}

fn main() { //quando declaro uma função, preciso especificar fn na frente do nome da função.
    shadowing(); //quando chamo uma função, uso apenas o nome da função.
}