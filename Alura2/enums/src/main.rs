fn main() {
    
    println!("é fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    // let dia: DiaDaSemana = DiaDaSemana::Sexta; - comentado para evitar erro de variavel não utilizada

    cores();
   
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool{
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }

}

#[allow(dead_code)] //retiro os warnings por codigo não utilizado, sintaxe de atributos, onde adiciono informações, ou metainformações, a algumas estruturas como enum, funções. ver metaprogramação.
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //uso parenteses para valores
    CymkColor{cyan: u8, magengta: u8, yellow: u8, black: u8} //uso chaves para nomes e tipos
}

fn cores(){
    let cor = Color::CymkColor{cyan: 100, magengta: 50, yellow: 70, black: 255};

    println!("Cor = {}", match cor { 
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "azul",
        Color::RgbColor(0, 0, 0) 
            | Color::CymkColor{cyan: _, magengta: _, yellow: _, black: 255} => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor{cyan: _, magengta: _, yellow: _, black: _} => "Cymk desconhecido"
    });
    //note que o compilador entende que desejo RGB, e se eu retirar uma das cores, ele vai acusar erro, pois eu não deixei uma posição em aberto com underscore para cobrir todas as possibilidades.
}
