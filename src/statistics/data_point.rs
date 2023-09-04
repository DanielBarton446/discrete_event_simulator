pub struct DataPoint {
    pub timestamp: usize,
    pub value: f64,
    pub unit: String,
}

impl DataPoint {
    pub fn new(timestamp: usize, value: f64, unit: String) -> DataPoint {
        DataPoint {
            timestamp,
            value,
            unit,
        }
    }
}

#[test]
fn create_datapoint() {
    let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    assert_eq!(data_point.timestamp, 0);
    assert_eq!(data_point.value, 1.0);
    assert_eq!(data_point.unit, "fake_unit");
}
