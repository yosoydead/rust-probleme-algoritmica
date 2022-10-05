/*
Problema Sortare intre indici
Iconita bookmark
Cerință
Se dă un șir de n numere naturale, precum și doi indici st și dr. Să se ordoneze crescător numerele din șir cu indicii între st și dr, inclusiv.

Date de intrare
Programul citește de la tastatură, de pe prima linie, numerele naturale n, st și dr, iar de pe a doua linie, cele n numere naturale ale șirului. Numerele de pe aceleași rând sunt separate prin câte un spațiu.

Date de ieșire
Programul afișează pe ecran șirul ordonat folosind criteriul menționat în cerință. Numerele afișate vor fi despărțite prin câte un spațiu.

Restricții și precizări
1 ≤ n ≤ 1000
1 ≤ cele n numere naturale ≤ 2.000.000.000
Se consideră că șirul este indexat de la 1
Exemplu
6 2 4
4 5 2 9 3 7
4 2 5 9 3 7
Explicația exemplului
Se ordonează crescător doar numerele dintre indicii st = 2 și dr = 4, celelalte elemente rămânând neschimbate. Astfel, se obține șirul dat.
*/
pub fn sortareIntreIndici() {
    let mut arr = [-818, -978, -447, 190, 550, -299, 483, -800, -801, 972, 36, 74, -998, -829, 901, 437, 395, 16, 9, 446, -792, -823, -660, -269, 350];
    let stanga = 3;
    let dreapta = 12;

    println!("arr {:?}", arr);
    sort(&mut arr[stanga..dreapta+1]);
    println!("arr {:?}", arr);
}

fn sort(a: &mut [i32]) {
    let mut index = 1;

    while (index < a.len()) {
        let mut j = index;

        while (j > 0 && a[j-1] > a[j]) {
            let temp = a[j-1];
            a[j-1] = a[j];
            a[j] = temp;

            j -= 1;
        }

        index += 1;
    }
}