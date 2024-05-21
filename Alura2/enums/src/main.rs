fn main() {
    
    println!("é fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    let dia: DiaDaSemana = DiaDaSemana::Sexta;
   
}

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
