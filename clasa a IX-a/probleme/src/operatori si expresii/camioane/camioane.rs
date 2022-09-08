/* 
O firmă are două tipuri de camioane: camioane de tipul 1, care pot transporta
câte t1 tone de marfă pe zi, și camioane de tipul 2, care pot transporta câte t2 tone de marfă pe zi.

Știind că firma are n camioane de tipul 1 și m camioane de tipul 2, câte tone de marfă pot transporta camioanele firmei în z zile.

Intrare

3 5 4 2 5
Ieșire

110

Explicație
Sunt 4 camioane de tipul 1, care pot transporta câte 3 tone de marfă și 2 camioane de tipul 2, care pot transporta câte 5 tone de marfă. Ele vor putea transporta în 5 zile 110 tone de marfă.


*/

pub fn camioane(t1: i32, t2: i32, n: i32, m: i32, z: i32) -> i32 {
    let result = ((n * t1) + (m * t2)) * z;
    println!("Camioane rezultat: {}", result);
    return result;
}