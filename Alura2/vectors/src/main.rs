fn main() {
    vectors();
}

fn vectors(){
    let mut notas:Vec<f32> = vec![10.0, 8.0, 6.5]; 

    println!("{:?}", notas);

    notas.push(7.5);

    println!("{:?}", notas);
}