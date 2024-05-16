fn shadowing() {

	let a = 123;
		{
		let a = 777;
		println!("a de dentro = {}", a);
		}
	println!("a de fora = {}", a);

}

fn main() {
    shadowing();
}