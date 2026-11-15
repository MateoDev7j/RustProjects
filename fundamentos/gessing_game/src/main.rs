use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number: ");
                        /*rng() devuelve un generador de números 
                        aleatorios,y random_range usa ese sistema 
                        y genera un numero aleatorio entre 1 y 100 
                        */
    let secret_number = rand::rng().random_range(1..=100);
    
    loop{

        println!("Please Input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)/*.read_line() lee la linea
            pasada por teclado y la añade al final del string 
            creado vacio mutado y pasado por referencia &*/
            .expect("Failed to read line");/*expect() es para
            manejar los warning y posibles errores que nos lanza
            el metodo read_line(); */

        println!("You guessed: {guess}");
                                    /* parse() devuelve un tipo Result
                                    un fallo, que debe convertir segun al tipo
                                    de numero que he declarado mi variable ala que
                                    quiero asignar el parse, como aqui 
                                    guess: u32 un entero de 32 bits entonces eso indica
                                    a parse que solo string que tengan numeros de 32 bits
                                    y si Result no es el esperado lo controlamos con 
                                    please type a number! */
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_)=>continue,/*Aprovechamos que devuelveun tipo enum
                            para utilizar el match y si da ok devolvemos el numero 
                            y se asigna al nuevo guess de u32 y si da Err le decimos
                            al programa que continue y como seguimos en el loop
                            el programa se reinicia */
        };

        /* match busca coincidencias en las llaves
        y se queda con la primera coincidencia exitosa que lea
        en orden y las demas ya no las lee, pero si no coincide
        ignora el codigo y pasa al siguiente */
        match guess.cmp(&secret_number) {
            /* guess.cmp(&secret_number) 
                devuelve un objeto Ordering::
                ya sea Less, Greater, O Equal */
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
