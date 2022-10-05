/*
Într-o clasă sunt F fete și B băieți. Fiecare fată citește 3 pagini pe zi și
fiecare băiat citește 2 pagini pe zi.Câte pagini vor citi copiii în n zile?
*/
pub fn copii2(f: i32, b: i32, zile: i32) {
    let fete = f * 3;
    let baieti = b * 2;
    let total = (fete + baieti) * zile;
    println!("Fetele au citit {} si baietii au citit {} pe zi. In {} zile s-au citit {} pagini.", fete, baieti, zile, total);
}