/*
Problema Educatie fizica
Iconita bookmark
La ora de educație fizică, profesorul vrea să îi aranjeze pe cei n elevi ai unei clase, astfel: se așază fetele înaintea băieților, iar fiecare dintre cele două grupe (fete și băieți) se ordonează separat descrescător (cei mai înalți sunt primii).

Cerință
Dându-se informațiile celor n elevi, să se afișeze înălțimea lor după ordonare.

Date de intrare
Programul citește de la tastatură, de pe prima linie, numărul natural n, reprezentând numărul de elevi, iar de pe următoarele două linii, două valori naturale x y, cu următoarea semnificație: dacă pentru un elev x = 1, atunci este fată, iar dacă x = 2, atunci este băiat; y reprezintă înălțimea elevului.

Date de ieșire
Programul afișează pe ecran, pe prima linie, n valori, reprezentând ordinea înălțimilor elevilor după ordonare.

Restricții și precizări
1 ≤ n ≤ 1000
Pentru fiecare elev, 100 ≤ y ≤ 200
Exemplu
5
1 170
1 156
2 159
1 165
2 178
170 165 156 178 159
Explicația exemplului
Sunt trei fete și doi băieți în clasă. Fetele sunt așezate primele și sunt ordonate descrescător, astfel că înălțimile lor sunt, în ordine, 170 165 156. După aceea urmează băieții, care sunt, la fel, ordonați descrescător, ordinea lor fiind 178 159. Ordinea finală a șirului este 170 165 156 178 159.

*/
pub fn educatieFizica() {
    let mut fete = [170, 156, 165];
    let mut baieti = [159, 178];

    sort(&mut fete);
    sort(&mut baieti);

    println!("fete {:?}", fete);
    println!("baieti {:?}", baieti);

    let mut total: [i32; 5] = [-1, -1, -1, -1, -1];
    let mut totalIndex: usize = 0;
    
    for i in 0..fete.len() {
        total[totalIndex] = fete[i];
        totalIndex += 1;
    }

    for i in 0..baieti.len() {
        total[totalIndex] = baieti[i];
        totalIndex += 1;
    }

    println!("baieti {:?}", total);
}

fn sort(arr: &mut [i32]) {
    let mut index = 1;

    while (index < arr.len()) {
        let mut j = index;

        while (j > 0 && arr[j-1] < arr[j]) {
            let temp = arr[j-1];
            arr[j-1] = arr[j];
            arr[j] = temp;

            j -= 1;
        }

        index += 1;
    }
}