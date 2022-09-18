/*
Problema Sortare prima cifra
Iconita bookmark
Cerință
Se dă un șir de n numere naturale. Să se ordoneze descrescător numerele după prima cifră. Dacă două numere au aceeași primă cifră, să se ordoneze crescător.

Date de intrare
Programul citește de la tastatură, de pe prima linie, numărul natural n, iar de pe a doua linie, cele n numere naturale ale șirului, separate prin câte un spațiu.

Date de ieșire
Programul afișează pe ecran șirul ordonat folosind criteriul menționat în cerință. Numerele afișate vor fi despărțite prin câte un spațiu.

Restricții și precizări
1 ≤ n ≤ 1000
1 ≤ cele n numere naturale ≤ 2.000.000.000
Exemplu
5
32 382 93 875 32
93 875 32 32 382
Explicația exemplului
S-au ordonat numerele în ordine descrescătoare după prima cifră: 9, 8, 3. Pentru numerele cu aceeași primă cifră (32, 382, 32), numerele s-au ordonat crescător.
*/
pub fn sortarePrimaCifra() {
    // let mut arr: [i32; 10] = [395, 680, 244, 826, 844, 59, 542, 881, 347, 421];
    let mut arr = [32, 382, 93, 875, 32];
    println!("arr {:?}", arr);
    sort(&mut arr);
    println!("arr {:?}", arr);
}

fn primaCifra(n: i32) -> i32 {
    let mut cp = n;
    let mut digit = -1;

    while (cp != 0) {
        digit = cp % 10;
        cp = cp / 10;
    }

    return digit;
}

fn sort(arr: &mut [i32]) {
    let mut index = 1;

    while (index < arr.len()) {
        let mut j = index;

        while (j > 0) {
            let b = primaCifra(arr[j-1]);
            let a = primaCifra(arr[j]);

            if (a < b) {
                break;
            }

            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;
            j -= 1;
        }

        index += 1;
    }
}