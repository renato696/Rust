fn main(){
    let multiplicador:u8 = 5;

    for i in 1..=10 {
        println!(" {} x {} = {}", multiplicador, i, multiplicador * i);
    }
}