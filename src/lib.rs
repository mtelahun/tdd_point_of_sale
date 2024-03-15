use rust_decimal::Decimal;

#[derive(Debug)]
pub struct PointOfSale {}

impl PointOfSale {
    pub fn scan(&self, barcode: &str) -> Decimal {
        if barcode == "12345" {
            Decimal::from(7_25)
        } else if barcode == "23456" {
            Decimal::from(12_50)
        } else {
            Decimal::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::PointOfSale;

    #[test]
    fn given_barcode_12345_then_display_7_25() {
        // Arrange
        let barcode = "12345";
        let pos = PointOfSale {};

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price, Decimal::from(7_25), "Barcode 1234 price is 7.25");
    }

    #[test]
    fn given_barcode_23456_then_display_12_50() {
        // Arrange
        let barcode = "23456";
        let pos = PointOfSale {};

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price, Decimal::from(12_50), "Barcode 23456 price is 12.50");
    }

    #[test]
    fn given_barcode_99999_then_display_not_found() {
        // Arrange
        let barcode = "99999";
        let pos = PointOfSale {};

        // Act
        let price = pos.scan(barcode);

        // Assert
        assert_eq!(price.err(), Error::NotFound, "Barcode 99999: not found");
    }
}