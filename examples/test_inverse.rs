extern crate debug;
extern crate nilpotent;

fn main() {
    let x = [4.0f64, 0.2, 0.5];
    let y = [5.0f64, 0.4, 0.7];
    let mut z = [0.0f64, ..3];
    nilpotent::mul(x.as_slice(), y.as_slice(), z.as_mut_slice());
    println!("{:?}", z);

    let mut x_inv = [0.0f64, ..3];
    nilpotent::inv(x.as_slice(), x_inv.as_mut_slice());
    println!("{:?}", x_inv);

    // Prints [1.0, 0.0, 0.0].
    let mut id = [0.0f64, ..3];
    nilpotent::mul(x.as_slice(), x_inv.as_slice(), id.as_mut_slice());
    println!("{:?}", id);
}
