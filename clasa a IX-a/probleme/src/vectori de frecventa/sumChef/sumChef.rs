/*
Cerința

Fiind dat un şir cu n elemente numere naturale, să se calculeze suma cuburilor cifrelor tuturor numerelor din şir.
Date de intrare

Programul citește de la tastatură numărul n şi cele n elemente ale şirului.
Date de ieșire

Programul va afișa pe ecran numărul S, reprezentând suma cuburilor cifrelor tuturor numerelor din şir .
Restricții și precizări

    1 ≤ n ≤ 2.000.000
    numerele din şir au cel mult 4 cifre


Exemplu

Intrare

3
24 120 51

Ieșire

207

Explicație

Suma cuburilor cifrelor numerelor din şir este S=23+43+13+23+03+53+13=207.
*/
pub fn sumChef() {
    let arr = [24, 120, 51];

    let mut res = 0;

    for i in 0..arr.len() {
        res += calcCub(arr[i]);
    }

    println!("{}", res);
}

fn calcCub(n: i32) -> i32 {
    let mut cp = n;
    let mut res = 0;

    while (cp != 0) {
        let c = cp % 10;

        res += (c *c *c);

        cp = cp / 10;
    }

    return res;
}