fn main() {
    
    let notas: [f32; 4] = [10f32, 8f32, 9.5, 6.0]; //array

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }
}    
