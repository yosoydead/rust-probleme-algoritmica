/* 
Să se scrie un program care citeşte de la 
    tastatura două numere naturale şi determină suma lor.
*/
pub fn sum() {
    let mut one = String::new();
    println!("Primul nr:");
    let b1 = std::io::stdin().read_line(&mut one).unwrap();

    let mut two = String::new();
    println!("Al2lea nr:");
    let b2 = std::io::stdin().read_line(&mut two).unwrap();

    let a = one.trim().to_string().parse::<i32>().unwrap();
    let b = two.trim().to_string().parse::<i32>().unwrap();

    println!("{}", a+b);
}