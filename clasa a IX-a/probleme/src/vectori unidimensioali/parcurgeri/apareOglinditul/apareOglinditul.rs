/*
Problema Apare oglinditul

Oglinditul unui număr natural n reprezintă numărul format prin așezarea cifrelor în ordine inversă (răsturnarea/oglindirea cifrelor sale). Spre exemplu, oglinditul numărului 124 este 421.
Cerință

Se dă un șir de n numere naturale, împreună cu o altă valoare naturală x. Să se verifice dacă printre elementele șirului se găsește și numărul egal cu oglinditul lui x.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului. De pe al treila rând, se citește numărul natural x.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă în șir apare numărul egal cu oglinditul numărului x, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 1000
    1 ≤ elementele șirului, x ≤ 2.000.000.000

Exemplu

5
1 324 31 7 8
13

DA

Explicația exemplului

Oglinditul numărului x = 13 este 31, iar acest număr apare în șir, așadar pe ecran se afișează mesajul DA.
*/
pub fn apareOglinditul() {
    let arr = [1, 324, 31, 7, 8];
    let mut cp = [1, 324, 31, 7, 8];

    let n = 13;
    let oglinda = oglindit(n);
    sort(&mut cp);
    let r = findNum(&cp, oglinda);
    println!("Oglinda lui n {} este {}. Se afla oglinda in array? {:?} Raspuns: {}", n, oglinda, cp, r);
}

fn oglindit(n: i32) -> i32 {
    if (n < 10) {
        return n;
    }

    let mut cp = n;
    let mut res = 0;

    while (cp != 0) {
        let c = cp % 10;
        cp = cp / 10;
        res = ((res * 10) + c);
    }

    return res;
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
    let mut r = arr.len();

    while (l < r) {
        let m = (((l+r) as f32) / 2.0).floor() as usize;

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