const PI:f32 = 3.14; // constante global, tempo de compilação. const pode ser global, fora da função, ou estar dentro da função.
static mut VARIABLE_GLOBAL:u8 = 1; //static é uma variavel global com endereço de memória, deve ser escrita com letras maiusculas e underscore para separar palavras

fn main () {

    println!("PI = {}", PI);

    unsafe { //bloco unsafe por causa da variável global mutável
    println!("variavel static = {}", VARIABLE_GLOBAL);
    }

    let variable:i32 = 300; // variavel imutável, tempo de execução
    println!( "variable = {}, tamanho = {} bytes", variable, std::mem::size_of_val(&variable));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let mut boolean:bool = false; //variavel mutável, tempo de execução
    boolean = true;
    println!("boolean = {}, tamanho boolean = {}",boolean, std::mem::size_of_val(&boolean));

    let letra:char = 'C';
    println!("tamanho do char = {}", std::mem::size_of_val(&letra));
}