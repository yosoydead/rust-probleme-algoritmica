/*
Într-un parc sunt n alei. Pe fiecare alee sunt n arbori.
Fiecare arbore are n crengi. Pe fiecare creanga sunt n cuiburi.
În fiecare cuib sunt n păsări. Câte păsări sunt în parc?
*/
pub fn parc2(n: i32) {
    let alei = n;
    let arbori = alei * n;
    let crengi = arbori * n;
    let cuiburi = crengi * n;
    let pasari = cuiburi * 3;
    println!("Sunt {} pasari in parc.", pasari);
}