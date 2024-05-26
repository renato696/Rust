fn main() {
    conta_corrente();
}

struct Conta {
    titular : String,
    saldo: f64
}

fn conta_corrente(){
    let conta: Conta = Conta{
        titular: String::from("Renato Pires"),
        saldo: 100.0
    }; // essa struct substitui as variáveis abaixo

    //let titular_conta: String = String::from("Renato Pires");
    //let saldo_conta: f64 = 100.0;

    println!("Dados da Conta: Titular = {}, Saldo = {}", conta.titular, conta.saldo); // como acessar os dados
}