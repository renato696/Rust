fn main(){
    pattern_matching()
}

fn pattern_matching(){
    for x in 1..=20 {
        println!("{}, {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho", //note o operador PIPE, diferente do || OU
            4..=10 => "Um bocado", //note o uso do range nessa forma, a forma padrão 4..11 não compila
            _ if x % 2 == 0 => "Uma boa quandidade", // ele vai pegar os numeros que não entraram em mattching antes, acima do 10, e vai pegar os pares usando IF.
            _ => "Muito"
        })}
    
/* outra possibilidade pegando conjunto de dois valores
 
        match point {
        (0,0) => "origem",
        (0, _) => "eixo x",
        (_, 0) => "eixo y"
    }
*/

}