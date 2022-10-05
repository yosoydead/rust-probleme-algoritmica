/*
Problema Toate diferite
Cerință

Se dă un șir de n numere naturale. Să se verifice dacă toate elementele șirului sunt diferite între ele sau nu.
Date de intrare

Programul citește de la tastatură, de pe primul rând, numărul natural n, iar de pe următorul rând, n numere naturale separate prin câte un spațiu, reprezentând valorile șirului.
Date de ieșire

Programul afișează pe ecran mesajul DA, dacă elementele șirului sunt diferite două câte două, respectiv NU în caz contrar.
Restricții și precizări

    1 ≤ n ≤ 1000
    1 ≤ elementele șirului ≤ 2.000.000.000

Exemple

5
1 3 2 4 5

DA

5
1 3 2 3 5

NU

Explicația exemplelor

Pentru primul exemplu: nu există nicio pereche de elemente egale, astfel că se afișează pe ecran mesajul DA.

Pentru al doilea exemplu: al doilea și al patrulea număr din șir sunt egale, așadar se afișează mesajul NU (deși restul elementelor erau diferite două câte două).
*/
pub fn toateDiferite() {
    let arr = [1, 3, 2, 4, 5];
    let arr2 = [1, 3, 2, 3, 5];

    let a = det(&arr);
    println!("{:?}", arr);
    if a == true {
        println!("DA");
    } else {
        println!("NU");
    }
}

fn det(arr: &[i32]) -> bool {
    let mut cp: Vec<i32> = Vec::new();
    
    for i in 0..arr.len() {
        cp.push(arr[i]);
    }

    sort(&mut cp);

    // println!("{:?}", cp);
    for i in 0..cp.len()-1 {
        if (cp[i] == cp[i+1]) {
            return false;
        }
    }

    return true;
}

fn sort(arr: &mut [i32]) {
    let mut index = 1;

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