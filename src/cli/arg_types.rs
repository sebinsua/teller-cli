#[derive(Debug)]
pub enum AccountType {
    Current,
    Savings,
    Business,
    Unknown(String),
    None
}

#[derive(Debug)]
pub enum OutputFormat {
    Spark,
    Standard,
}

#[derive(Debug)]
pub enum Interval {
    Monthly,
}

#[derive(Debug)]
pub enum Timeframe {
    Year,
    SixMonths,
    ThreeMonths,
}
