use rust_decimal::Decimal;

#[derive(Debug)]
pub struct PointOfSale {}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos error: barcode not found")
    }
}

impl std::error::Error for Error {}

impl PointOfSale {
    pub fn scan(&self, barcode: &str) -> Result<Decimal, Error> {
        if barcode == "12345" {
            Ok(Decimal::from(7_25))
        } else if barcode == "23456" {
            Ok(Decimal::from(12_50))
        } else {
            Err(Error::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::{Error, PointOfSale};

    #[test]
    fn given_barcode_12345_then_display_7_25() {
        // Arrange
        let barcode = "12345";
        let pos = PointOfSale {};

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.unwrap(), Decimal::from(7_25), "Barcode 1234 price is 7.25");
    }

    #[test]
    fn given_barcode_23456_then_display_12_50() {
        // Arrange
        let barcode = "23456";
        let pos = PointOfSale {};

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.unwrap(), Decimal::from(12_50), "Barcode 23456 price is 12.50");
    }

    #[test]
    fn given_barcode_99999_then_display_not_found() {
        // Arrange
        let barcode = "99999";
        let pos = PointOfSale {};

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.err().unwrap(), Error::NotFound, "Barcode 99999: not found");
    }

    #[test]
    fn given_empty_barcode_then_return_empty_barcode() {
        // Arrange
        let barcode = "";
        let pos = PointOfSale {};

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.err().unwrap(), Error::Empty, "Barcode 99999: not found");
    }
}