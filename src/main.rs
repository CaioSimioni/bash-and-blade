use bash_and_blade::utils::math;

fn main() {
    println!("Inicio...");
    println!("Soma de 2 + 3: {}", math::add(2, 3));
    println!("Subtração de 5 - 3: {}", math::subtract(5, 3));
    println!("Multiplicação de 2 * 3: {}", math::multiply(2, 3));
    match math::divide(6, 3) {
        Ok(result) => println!("Divisão de 6 / 3: {}", result),
        Err(e) => println!("Erro: {}", e),
    }
    print!("Divisão de 6 / 0: ");
    match math::divide(6, 0) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Erro: {}", e),
    }
    println!("Fim...")
}
