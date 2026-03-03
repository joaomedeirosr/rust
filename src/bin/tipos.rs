fn main() {
    // Tipo primitivo int (signed (i) e unsigned (u))
    let age : i8 = 10;
    println!("{}",age);

    let number : i16 = 12;
    println!("{}",number);

    let number_x : i32 = -32;
    println!("{}",number_x);

    let number_y : u8 = 34;
    println!("{}",number_y);

    // Tipo primitivo float
    let x = 2.0; // Double Precision
    println!("{}",x);

    let y: f32 = 3.0; // Single Precision
    println!("{}",y);
    
    operations();
    bolleans();
    char_type();
    coumpound_types();
    array_type();
}

fn operations() {
    let soma = 5 + 10;

    println!("{}",soma);

    let diferenca = 95.5 - 4.3;
    println!("{}", diferenca);

    let produto = 4 * 30;
    println!("{}",produto);

    let quociente = 56.7 / 32.2;
    println!("{}",quociente);

    let resto = 43 % 5;
    println!("{}",resto);
}

fn bolleans() {
    let t = true;
    println!("{}",t);
    let f = false;
    println!("{}",f);
}

fn char_type() {
    let c = 'z';
    println!("{}",c);

    let z = 'ℤ';
    println!("{}",z);

    let heart_cat = '😻';
    println!("{}",heart_cat);
}

fn coumpound_types() {
    // Rust tem dois tipos compostos tuplas e vetores
    let tup = (500, 6.4, 1);
    println!("{:?}",tup); // Utilizado para interpolar valores de colecoes

    let (_x,y,_z) = tup;
    println!("O valor de y e: {}",y);

    // Acessando os valores da tupla pelo indice, parecido com lista em Python
    let acess_tup = (500, 6.4 ,1);

    let five_hundred = acess_tup.0;
    let _six_four = acess_tup.1; // "Simbolo (_) underline, diz para compilador Eu sei que a variavel nao vai ser utilizada"
    let _one = acess_tup.2;

    println!("{}",five_hundred);
}

fn array_type() {
    // Array type no Rust deve necessariamente, possuir todos os elementos do mesmo tipo diferente da Tupla e possuem tamanho fixo nao pode aumentar de tamanho
    let months = [1,2,3,4,5];
    println!("{:?}",months);

    let month_names = ["January","Febreuary","March","April","May","June","July","August","September","October","November","December",];
    let first_month = month_names[0];
    let second_month = month_names[2];
 
    /*
    &str, na realidade e um slice, uma fatia do Array, o array e dificilmente usado pois tem valores fixos que nao podem ser alterados
    ou seja eles sao "enviados" para a Stack que possui tamanho fixo e e mais rapida. Entretanto, na maior parte das vezes, vamos utilizar
    Vetores (Vectors) que e um tipo dispooniabilizado pela Biblioteca padrao da linguagem
    */ 

    println!("{}",first_month);
    println!("{}",second_month);

}