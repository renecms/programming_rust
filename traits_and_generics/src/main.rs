use std::{
    fs::File,
    io::{self, Write},
};
fn main() -> Result<(), io::Error> {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;
    writer.write_all(b"hello")?; // ok

    let mut local_file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open("hello.txt")?;

    say_hello_generics(&mut local_file)?;
    say_hello_trait(&mut local_file)?;

    // when to specify type using turbofish
    say_hello_generics::<File>(&mut local_file)?;

    // let v1 = (0..1000).collect(); // error: can't infer type
    // let v1: Vec<i32> = (0..1000).collect(); // error: can't infer type
    // let v2 = (0..1000).collect::<Vec<i32>>(); // ok

    // Explicitly provide `3` as the value for `N`.
    println!("{}", dot_product::<3>([0.2, 0.4, 0.6], [0., 0., 1.]));

    // Let Rust infer that `N` must be `2`.
    println!("{}", dot_product([3., 4.], [-5., 1.]));
    
    Ok(())
}

fn say_hello_generics<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world generics\n")?;
    out.flush()
}

fn say_hello_trait(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world trait objects\n")?;
    out.flush()
}

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}


trait Vegetable {

}

struct Salad<V: Vegetable> {
    // Only the same type will be present in the Vec
    veggies: Vec<V>
}

struct BetterSalad {
    // any struct that implements Vegetable can be used here
    veggies: Vec<Box<dyn Vegetable>>
}


pub struct Sink;
impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Claim to have successfully written the whole buffer.
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
