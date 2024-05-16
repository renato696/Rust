fn main(){

    let linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "web",
        "Kotlin" => "android",
        "Python" => "data science",
        _ => "desconhecido" // o underline é para todo o resto....
    };

    println!("o proposito de {} eh {}", linguagem, proposito);

}