pub fn pgcd(a:i32, b:i32)
{

    let mut b: i32 = b;
    let mut a: i32 = a;

    while b != a
    {
        if a > b
        {
            a = a - b
        }
        else
        {
            b =  b - a
        }
    }
    println!("{}",b);
}
fn primaliter(nb:i32) {

    for i in 2..nb - 1
    {

        if nb % i == 0
        {
            println!("{} n'est pas premier", nb);
            break;
        }
        else
        {
            println!("{} est premier", nb);
            break;
        }
    }
}