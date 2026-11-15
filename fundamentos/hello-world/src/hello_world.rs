use std::collections::{HashMap, HashSet};

fn main(){
    //Hola mundo
    /* Hola mundo */
    println!("Hola Rust!");


    //Varibales
    let mut my_string: &str= "Esto es una cadena de texto";
    println!("{}",my_string);
    my_string ="Aqui cambio el valor de la cadena de texto";
    println!("{}",my_string);

    // my_string = 6; Error
    //my_string = 6;
    //println!("{my_string}");

    let my_string2: String = String::from("Esta es otra cadena de texto");
    println!("{my_string2}");

    let mut my_int: i32 = 7; 
    my_int = my_int + 4;
    println!("{my_int}");//11
    println!("{}", my_int -1);//10
    println!("{my_int}");//Sigue valiendo 11

    println!("Este es el valor de la variable my_int: {}", my_int);

    let  my_int64: i64 = 7;//Si trabajas con numeros muy grandes
    println!("{my_int64}");

    let  my_float: f64 =6.5;
    println!("{my_float}");
    //my_float = my_float + my_int;  Error no se pueden mezclar tipos: rust tipado fuerte
    let  my_float2: f32 = 6.5;
    println!("{my_float2}");

    let mut my_bool: bool=false;
    println!("{my_bool}");
    my_bool=true;
    println!("{my_bool}");
    //Constantes
    const MY_CONST: &str ="Mi propiedad constante";
    println!("{MY_CONST}");

    //Control de flujo 

    if my_int == 10 && my_string == "Hola" {
        println!("El valor es 10")
    } else if my_int ==11 || my_string=="Hola"{
        println!("El valor es 11")
    }
    else{
        println!("El valor no es 10 ni 11")
    }

    // Lista
    let mut my_list: Vec<&str> = vec!["Mateo", "Ortunyo","@mateoDev7j"];
    my_list.push("Mateo");
    println!("{:?}",my_list);
    println!("{:?}",my_list[0]);
    println!("{}",my_list[1]);

    // Sets 
    let mut my_set: HashSet<&str> = vec!["Mateo", "Ortunyo","@mateoDev7j"].into_iter().collect();
    my_set.insert("Mateo");
    println!("{:?}",my_set);//Set no puede repetir datos por lo que no se inserta Mateo
    // y la collecion set develve los datos desordenados                                        //Esto lo hace una collecion iterable, y collect la comvierte en collection
    
    //Mapas
    let mut my_map: HashMap<&str, i32> = vec![("mateo", 36),("javier", 46),("mesi", 56)]
    .into_iter()
    .collect();
    my_map.insert("Emi",26);
    println!("{:?}",my_map);

    // Bucles
    for value in &my_list {
        println!("{}",value);
    }
    for value in &my_set{
        println!("{}",value);
    }
    for (key,value) in &my_map{
        println!("{} {}", key , value);
    }
    let mut my_counter:usize = 0;
    while my_counter < my_list.len(){
        println!("{}",my_list[my_counter]);
        my_counter+=1;
    }
    my_function();
    my_function();
    my_function();

    //Estructuras
    let my_struct = MyStruct::new("Mateo", 20);
    println!("{} {}",my_struct.name, my_struct.age)

}
fn my_function(){
        println!("Esto es una funcion");
}
struct MyStruct{
    name: String,
    age: i32,
}
impl MyStruct {
    fn new(name:&str , age:i32) -> MyStruct{
        MyStruct{name: String::from(name),
            age,
        }
    }
}