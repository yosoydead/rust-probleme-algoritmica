/*
Problema Determinare vector 6
Cerință

Se dă un șir de n numere naturale, împreună cu o altă valoare naturală x. Să se verifice dacă printre elementele șirului se găsește și numărul egal cu numărul de cifre ale lui x.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului. De pe al treilea rând, se citește numărul natural x.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă în șir apare numărul egal cu numărul de cifre ale lui x, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 10.000
    0 ≤ elementele șirului, x ≤ 2.000.000.000

Exemplu

5
1 34 2340 3 4
143

DA

Explicația exemplului

Numărul x = 143 are 3 cifre. Cum numărul 3 apare în șirul dat, pe ecran se afișează mesajul DA.
*/
pub fn determinare6() {
    let arr = [1, 34, 2340, 3, 4];
    let mut cp: [i32; 5] = [-1; 5];

    copy(&arr, &mut cp);
    sort(&mut cp);

    let x = 143;
    let digits = numOfDigits(x);

    let r = hasNum(&cp, digits);

    if (r == true) {
        println!("DA");
    } else {
        println!("NU");
    }
}

fn copy(src: &[i32], to: &mut [i32]) {
    if (src.len() != to.len()) {
        return;
    }

    for i in 0..src.len() {
        to[i] = src[i];
    }
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

fn numOfDigits(n: i32) -> i32 {
    if (n > 0 && n < 10) {
        return 1;
    }

    let mut cp = n;

    let mut res = 0;

    while (cp != 0) {
        cp = cp / 10;
         res += 1;
    }

    return res;
}

fn hasNum(arr: &[i32], n: i32) -> bool {
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
