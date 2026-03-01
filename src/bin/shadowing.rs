fn print_type_of <T>(_:&T) {
    println!("{}", std::any::type_name::<T>());
}
fn main() {
     let x = 5;
    
    let x = x + 1;

    let x = x.to_string();

    print_type_of(&x);
    
    println!("O valor de x e: {}",x);

}