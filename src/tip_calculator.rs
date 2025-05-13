use std::error::Error;

pub struct TipCalculator {
    bill: f64,
    tip_percent: f64,
}

impl TipCalculator {
    pub fn new(bill: String, tip_percent: String) -> Result<Self, Box<dyn Error>> {
        let bill: f64 = bill.trim().parse()?;
        let tip_percent: f64 = tip_percent.trim().parse()?;

        if bill.is_sign_negative() || tip_percent.is_sign_negative() {
            return Err("Error: Only positive values are accepted".into());
        }

        Ok(Self { bill, tip_percent })
    }

    pub fn calculate_tip(&self) -> f64 {
        let result = self.bill * (self.tip_percent / 100.0);
        let result = format!("{:.2}", result).parse::<f64>().unwrap();
        result
    }

    pub fn calculate_total(&self) -> f64 {
        self.bill + self.calculate_tip()
    }
}
