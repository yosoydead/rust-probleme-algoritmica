/*
Problema Determinare vector 5
Cerință

Se dă un șir de n numere naturale, împreună cu o altă valoare naturală x. Să se verifice dacă printre elementele șirului se găsesc toți divizorii lui x.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului. De pe al treilea rând, se citește numărul natural x.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă toți divizorii lui x apar în șir, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 10.000
    1 ≤ elementele șirului, x ≤ 2.000.000.000

Exemplu

5
1 3 5 7 9
9

DA

Explicația exemplului

Divizorii numărului x = 9 sunt 1, 3, 9. Toți acești trei divizori apar în șirul dat, așadar, pe ecran se afișează mesajul DA.
*/
pub fn determinare5() {
    // let arr = [1, 3, 5, 7, 9];
    let arr = [3, 9, 5, 1, 7];
    let x = 9;
    
    let mut cp: [i32; 5] = [0; 5];

    copy(&arr, &mut cp);
    sort(&mut cp);

    let res = hasAllDivisors(&cp, x);

    if (res == true) {
        println!("DA");
    } else {
        println!("NU");
    }
}

fn copy(src: &[i32], to: &mut [i32]) {
    if (src.len() != to.len()) {
        println!("Nu pot sa copiez. Diferenta de marime in array-uri");
        return;
    }

    for i in 0..src.len() {
        to[i] = src[i];
    }
}

fn sort(arr: &mut [i32]) {
    let mut index: usize = 1;

    while (index < arr.len()) {
        let mut j = index;

        while (j > 0 && arr[j-1] > arr[j]) {
            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }

        index += 1;
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

fn hasAllDivisors(arr: &[i32], n: i32) -> bool {
    for i in 1..n+1 {
        if (n % i == 0) {
            let hasNum = findNum(&arr, i);
            if (hasNum == false) {
                return false;
            }
        }
    }

    return true;
}
