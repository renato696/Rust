fn main() {
    vectors();
}

fn vectors(){
    let mut notas:Vec<f32> = vec![10.0, 8.0, 6.5]; 

    println!("{:?}", notas);

    notas.push(7.5);

    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);

    println!("Nota 6 = {}", match notas.get(7){
        Some(n) => *n, //uso o * para desreferenciar o valor
        None => 0.0
    });

    if let Some(nota) = notas.pop(){
        println!("Ultimo valor = {}", nota);
        println!("{:?}", notas);
    }

    /* 
    while let Some(nota) = notas.pop(){
        println!("Valor removido = {}", nota);
    }*/

    for nota in &notas { //detalhe do borrow
        println!("Nota = {}", nota);
    }
    println!("{:?}", notas);

}