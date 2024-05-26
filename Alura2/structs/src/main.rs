fn main() {
    conta_corrente();
}

struct Conta {
    titular : Titular,
    saldo: f64
}

impl Conta {
    fn sacar(& mut self, valor: f64) { //se tivesse retorno deveria informar
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String,
}

fn conta_corrente(){
    let titular = Titular{
        nome: String::from("Renato"), 
        sobrenome: String::from("Pires")};
 
    let mut conta: Conta = Conta {
        titular, //quando a variavel tiver o mesmo valor do atributo, eu posso omitir o valor e deixar apenas o atributo.
        saldo: 100.0
}; 

conta.sacar(50.0);

    println!("Dados da Conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo); // como acessar os dados
}