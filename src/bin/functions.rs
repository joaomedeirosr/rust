fn main(){
    numbers(5);
    soma(3,5);

    // O Rust assim como, Python e outras linguagens podemos atribuir chamadas de funcoes, a variaveis
    let x = five_function();
    println!("O valor de x e: {}",x);

    expressions_teste();

    let z = soma_um(5);

    println!("O valor de z e: {}",z);
}

fn numbers(x:i32){
    println!("O valor de x e: {}",x);
}

// Precisamos informar, o tipo do parametros / variaveis locais no momento da assinatura da funcao
fn soma(x:i32, y:i32) {
    let soma = x +y;
    println!("{}",soma);
}

// Funcoes com valor de retorno precisamos informar, o retorno ex: i32 retorna um int32 lembrando representa o valor da expressao final do bloco
fn five_function() -> i32{
    return 5
}

fn expressions_teste(){
    let _x = 5;

    let y = {
        let x = 3;
           x+1 // Expressao retorna um valor e armazena na variavel y
    };

    println!("O valor de y e : {}", y);
}

// Informo o valor do parametro e do retorno
fn soma_um(x:i32) -> i32{
     x + 1
}

// Posso fazer esse tipo de declaracao, que e semelhante ao pass do Python, mas isso e apenas algo explicito porque em Rust o escopo vazio ja e valido


/*
Em Rust temos Declaracoes e Expressoes, Declaracoes nao retornam valor ja Expressoes sim,
Definicoes de funcao tambem sao declaracoes, logo nao podemos declarar uma variavel de outra
Na maior parte das vezes em Rust quase tudo e uma Expressao e isso e bem comum de se encontrar no Rust
quando nao temos uma declaracao, certamente temos uma expressao

Ex: let x = (let y = 6); (ERRADO em Rust)

Em outras linguagens o trecho (let y = 6); poderia retornar um valor e ser atribuido a x e entao se torna uma expressao 
e entao retorna valor, no Rust isso nao existe, voce poderia fazer como e no Python x = y = 6, ou seja, neste caso
tanto x quanto y, vai receber o valor de 6, isso tambem nao funciona em Rust

As chamadas de funcao, ou chamadas de macro, sao expressoes logo isso fica implicido que elas retornam um valor, ate
mesmo o bloco que utilizamos para criar um escopo e uma expressao isso singifica que todo escopo pode ou nao retornar 
um valor

Um aspecto interessante mas sutil e que em Rust, toda declaracao precisa de um ponto e virgula (;), ja as expressoes
nao precisam de um ponto-virgula e mostra que elas retornam valores, elas pode estar acompanhadas ou nao de um valor

Na maior parte das vezes em Rust, a funcao retorna o ultimo valor implicitamente. Ou posso utilizar a palavra reservada
return como em outra linguagens

*/
