fn main() {
    conteudo_opcional();
}

fn conteudo_opcional(){
    let conteudo_arquivo = ler_arquivo(String::from(""));
        
    match &conteudo_arquivo { //lembrar do borrow &
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };

    println!("{:?}", conteudo_arquivo); //lembrar do borrow &
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Algum Conteúdo"))
    //None
}