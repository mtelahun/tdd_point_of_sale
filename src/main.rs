use std::io::{self, BufRead, Write};

use tdd_point_of_sale::PointOfSale;

fn main() {
    let mut pos = PointOfSale::new();

    for _ in 0..2 {
        write!(io::stdout(), "Please scan item: ").expect("failed to write prompt");
        io::stdout().flush().expect("failed to flush prompt");
        let mut barcode = String::new();
        io::stdin()
            .lock()
            .read_line(&mut barcode)
            .expect("failed to read barcode");
        match pos.scan(barcode.trim()) {
            Ok(_) => continue,
            Err(e) => println!("Oops! {e}"),
        }
    }

    writeln!(io::stdout(), "Your total is: {}", pos.command("total"))
        .expect("failed to write prompt");
}
