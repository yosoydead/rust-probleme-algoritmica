/*
Problema Comenzi

Sergej și-a luat un job de vară la un magazin online. Are o grămadă de sarcini, care mai de care mai plictisitoare. Una dintre sarcinile lui este să extragă din două liste lungi de comenzi, comenzile care se regăsesc în ambele liste.

Comenzile sunt exprimate prin numere naturale.
Cerință

Dându-se cele două liste de comenzi, de lungime n și m, să se afișeze numărul de comenzi care se regăsesc în ambele liste.
Date de intrare

Din fișierul de intrare comenzi.in se citește, de pe prima linie, numărul natural n, de pe a doua linie, n numere naturale despărțite prin câte un spațiu reprezentând comenzile primei liste, de pe a treia linie, numărul natural m, iar de pe a patra linie, m numere naturale despărțite prin câte un spațiu reprezentând comenzile celei de-a doua zile.
Date de ieșire

În fișierul de ieșire comenzi.out se va afișa un singur număr, reprezentând numărul de comenzi care se regăsesc pe ambele liste.
Restricții și precizări

    1 ≤ n, m ≤ 50.000
    1 ≤ fiecare comandă ≤ 2.000.000.000

Exemplu

5
342 934 32 103 55
3
1000 342 33

1

Explicația exemplului

Dintre cele 5 comenzi ale primei liste (342 934 32 103 55), doar una singură aparține și celei de-a doua liste (1000 342 33): 342. Astfel, răspunsul este 1.
*/
pub fn comenzi() {
    let mut a = [342, 934, 32, 103, 55];
    let b = [1000, 342, 33];

    sort(&mut a);

    let mut common = 0;

    for i in 0..b.len() {
        let res = findNum(&a, b[i]);

        if (res == true) {
            common += 1;
        }
    }

    println!("Comenzi in comun {}", common);
}

fn sort(arr: &mut [i32]) {
    let mut i = 1;

    while (i < arr.len()) {
        let mut j = i;

        while (j > 0 && arr[j-1] > arr[j]) {
            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }
        i += 1;
    }
}

fn findNum(arr: &[i32], n: i32) -> bool {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while (l <= r) {
        let m = ((l + r) as f32 / 2.0).floor() as usize;

        if (arr[m] < n) {
            l += 1;
        } else if (arr[m] > n) {
            r -= 1;
        } else {
            return true;
        }
    }

    return false;
}