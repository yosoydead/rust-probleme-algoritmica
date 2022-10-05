/*Problema Sortare munte
Iconita bookmark
Cerință
Se dă un șir de n numere naturale. Să se ordoneze crescător numerele din stânga elementului maxim din șir și descrescător numerele din dreapta acestui element.

Date de intrare
Programul citește de la tastatură, de pe prima linie, numărul natural n, iar de pe a doua linie, cele n numere naturale ale șirului, separate prin câte un spațiu.

Date de ieșire
Programul afișează pe ecran șirul ordonat folosind criteriul menționat în cerință. Numerele afișate vor fi despărțite prin câte un spațiu.

Restricții și precizări
1 ≤ n ≤ 1000
1 ≤ cele n numere naturale ≤ 2.000.000.000
Cele n numere sunt distincte
Exemplu
6
4 5 2 9 3 7
2 4 5 9 7 3
Explicația exemplului
Elementul maxim al șirului este 9, aflat pe poziția 4. Astfel, elementele de pe pozițiile 1 – 3 s-au ordonat crescător (2, 4, 5), iar elementele de pe pozițiile 5 – 6 s-au ordonat descrescător (7, 3). */
pub fn sortareMunte() {
    let mut arr = [4, -2, 3, -50, 5, 2, 9, 3, 7, 6, -6, -1, 1, 2, 3];
    
    println!("arr: {:?}", arr);
    let big = biggestIndex(&arr);
    sort(&mut arr[0..big], true);
    sort(&mut arr[big+1..], false);
    println!("arr: {:?}", arr);
}

fn sort(a: &mut [i32], asc: bool) {
    let mut index = 1;

    while (index < a.len()) {
        let mut j = index;

        if (asc == true) {
            while (j > 0 && a[j-1] > a[j]) {
                let temp = a[j-1];
                a[j-1] = a[j];
                a[j] = temp;

                j -= 1;
            }
        } else {
            while (j > 0 && a[j-1] < a[j]) {
                let temp = a[j-1];
                a[j-1] = a[j];
                a[j] = temp;

                j -= 1; 
            }
        }
        
        index += 1;
    }
}

fn biggestIndex(a: &[i32]) -> usize {
    let mut resultIndex: usize = 0;
    let mut currValue = std::i32::MIN;

    for i in 0..a.len() {
        if (a[i] > currValue) {
            currValue = a[i];
            resultIndex = i;
        }
    }
    return resultIndex;
}