# 🦀 Rust, study notes 
> Author: João Victor Rocha

## 📖 Introducao

Vamos primeiramente fazer uma breve introducao relacionado ao conceito que e o coracao da linguagem **Rust** que e justamente o **Ownership**, conceito este que esta diretamente ligado a como a memoria funciona ou melhor dizendo como manipulamos a memoria em runtime. Logo, este conceito esta diretamente ligado a **Stack** e **Heap**. Dentro do Ownership tambem, abordaremos o **Borrowing** e por fim vamos abordar o **Shadowing** que fecha um dos principais conceitos do Rust.

Posteriormente, vamos falar sobre os exemplos abordados aqui para aprofundar um pouco mais e sair do nivel superficial.

## 🔧 Stack e Heap
> Adicionar as anotacoes previamente feitas aqui

## 🤝🏻 Shadowing
> Adicionar aqui o conceito de shadowing

## 🛠️ Conceito de Ownership - (Posse)
- Cada valor tem um dono;
- So pode ter um dono por vez;
- Dono fora do escopo, valor sera eliminado

## 🔨 Borrowing - (Ceder Posse) 
> Adicionar aqui o conceito de borrowing

## 🚨 Regras de referencias
- So uma referencia mutavel por vez;
- Referencias sempre serao validas;
- Varias referencias imutaveis ao mesmo tempo

So podemos ter uma referencia mutavel por vez por regras fundamentais do borrow checker:

Logo, podemos ter ou varias referencias imutaveis (&T), ou uma unica referencia mutavel (&mut T)

## 🧪 Exemplos:

```rust
    fn main() {
        let nome : String = String::from("Rust");
        let novo_nome : String = nome;


        println!("Nome: {}", novo_nome);
}
```
2) Example with String clone:

```rust

    fn main() {
        let nome: String = String::from("Rust");
        println!("Nome {}", nome);

        let novo_nome: String = nome.clone();
        println!("Nome {}", novo_nome);
    }

```  
3) Example with Array type:

```rust
    fn main() {

        let lista_a : [i32; 5] = [1,2,3,4,5];
        let lista_b : [i32,5] = lista_a;

        println!("{:?}", lista_a);
        println!("{:?}", lista_b);
}
```

## 🥇 License
The [MIT License]() (MIT)