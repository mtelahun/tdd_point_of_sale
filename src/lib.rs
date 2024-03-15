use rust_decimal::Decimal;

#[derive(Debug)]
pub struct PointOfSale {}

impl PointOfSale {
    pub fn scan(&self, barcode: &str) -> Decimal {
        todo!()
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
}