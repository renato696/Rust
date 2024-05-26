fn main() {
    conta_corrente();
}

struct Conta {
    titular : Titular,
    saldo: f64
}

impl Conta {
    
}

struct Titular {
    nome: String,
    sobrenome: String,
}

fn conta_corrente(){
    let titular = Titular{
        nome: String::from("Renato"), 
        sobrenome: String::from("Pires")};
 
    let conta: Conta = Conta {
        titular, //quando a variavel tiver o mesmo valor do atributo, eu posso omitir o valor e deixar apenas o atributo.
        saldo: 100.0
    }; 
    
    println!("Dados da Conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo); // como acessar os dados
}