/*
Problema Cate cifre de fiecare tip
Cerință

Se dă un număr natural n. Să se determine câte cifre de 0, 1, …, 9 are numărul.
Date de intrare

Programul citește de la tastatură un singur număr natural, n.
Date de ieșire

Programul afișează pe ecran 10 valori, despărțite printr-un spațiu: numărul de cifre de 0 din număr, numărul de cifre de 1 din număr, …, numărul de cifre de 9 din număr.
Restricții și precizări

    0 ≤ n ≤ 2.000.000.000

Exemplu

50030

3 0 0 1 0 1 0 0 0 0

Explicația exemplului

50030 are 3 cifre de 0, 1 cifră de 3 și 1 cifră de 5.
*/
pub fn cifreDeFiecareTip() {
    let n = 50030;
    let mut cp = n;

    let mut vec: [u8; 10] = [0; 10];
    
    while (cp != 0) {
        let c = cp % 10;
        
        vec[c] += 1;
        cp = cp / 10;
    }
    println!("{:?}", vec);
}