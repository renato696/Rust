fn soma(a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b //retorno será utilizado
}


fn main(){
    println!("Soma = {}", soma(2, 2));
}