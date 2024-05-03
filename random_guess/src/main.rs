use rand::Rng;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    
    let x = rand::thread_rng().gen_range(1..=100);
   
    loop {
        let mut guess = String::new();
        print!("Digite um número: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut guess)?;
  
        let g = guess.trim().parse::<i32>().unwrap();
    
        if x > g {
            println!("O número certo é maior.");
        } else if x < g {
            println!("O número certo é menor.");
        } else {
            println!("Acertou!");
            break;
        }
    }
    
    Ok(())
}
