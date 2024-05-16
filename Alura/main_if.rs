fn main(){

    let idade:u8 = 18;
    let respons_auth:bool = true;
    let eh_maior = idade >=18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && respons_auth {
        println!("Pode entrar com autorização");
    } else {
        println!("Fora da balada");
    }

    let condicao = if eh_maior { "maior" } else { "menor" }; // if em forma de expressão;

    println!("É {} de idade", condicao);

}