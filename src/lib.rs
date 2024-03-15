use rust_decimal::Decimal;

#[derive(Debug)]
pub struct PointOfSale {
    #[allow(dead_code)]
    sum: Decimal,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IsEmpty,
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::IsEmpty => "empty barcode",
            Error::NotFound => "barcode not found",
        };

        write!(f, "pos error: {msg}")
    }
}

impl std::error::Error for Error {}

impl PointOfSale {
    pub fn new() -> Self {
        Self {
            sum: Decimal::ZERO,
        }
    }

    pub fn scan(&mut self, barcode: &str) -> Result<Decimal, Error> {
        if barcode.is_empty() {
            Err(Error::IsEmpty)
        } else if barcode == "12345" {
            let price = Decimal::new(725, 2);
            self.sum += price;

            Ok(price)
        } else if barcode == "23456" {
            let price = Decimal::new(1250, 2);
            self.sum += price;

            Ok(price)
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn command(&self, _cmd: &str) -> Decimal {
        self.sum
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use crate::{Error, PointOfSale};

    #[test]
    fn given_barcode_12345_then_display_7_25() {
        // Arrange
        let barcode = "12345";
        let mut pos = PointOfSale::new();

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.unwrap(), Decimal::from_str("7.25").unwrap(), "Barcode 1234 price is 7.25");
    }

    #[test]
    fn given_barcode_23456_then_display_12_50() {
        // Arrange
        let barcode = "23456";
        let mut pos = PointOfSale::new();

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.unwrap(), Decimal::from_str("12.50").unwrap(), "Barcode 23456 price is 12.50");
    }

    #[test]
    fn given_barcode_99999_then_display_not_found() {
        // Arrange
        let barcode = "99999";
        let mut pos = PointOfSale::new();

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.err().unwrap(), Error::NotFound, "Barcode 99999: not found");
    }

    #[test]
    fn given_empty_barcode_then_return_empty_barcode() {
        // Arrange
        let barcode = "";
        let mut pos = PointOfSale::new();

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.err().unwrap(), Error::IsEmpty, "Barcode 99999: not found");
    }

    #[test]
    fn given_test_command_then_display_sum_of_prices_of_scanned_products() {
        // Arrange
        let cmd = "total";
        let mut pos = PointOfSale::new();
        let p1 = pos
            .scan("12345")  // 7.25
            .expect("failed to scan barcode '12345'");
        let p2 = pos
            .scan("23456")  // 12.50
            .expect("failed to scan barcode '23456'");
        let pos = pos;

        // Act
        let result = pos.command(cmd);

        // Assert
        assert_eq!(result, Decimal::from_str("19.75").unwrap());
        assert_eq!(result, p1 + p2, "total is sum of individual scans");
    }
}