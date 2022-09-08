/*
Într-un brad sunt a globuri albe, de două ori mai multe globuri roșii,
iar globuri verzi cu 3 mai puține ca numărul de globuri roșii.
Câte globuri sunt în total ?
*/
pub fn globuri(albe: i32) {
    let rosii = albe * 2;
    let verzi = rosii - 3;
    let total = albe + rosii + verzi;

    println!("Albe: {}. Rosii: {}. Verzi: {}. Total: {}", albe, rosii, verzi, total);
}