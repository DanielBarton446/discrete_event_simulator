pub struct DataPoint {
    pub timestamp: usize,
    pub statistic_label: String,
    pub value: f64,
}

impl DataPoint {
    pub fn new(timestamp: usize, statistic_label: String, value: f64) -> DataPoint {
        DataPoint {
            timestamp,
            statistic_label,
            value,
        }
    }
}

#[test]
fn create_datapoint() {
    let data_point = DataPoint::new(0, "test".to_string(), 1.0);
    assert_eq!(data_point.timestamp, 0);
    assert_eq!(data_point.statistic_label, "test");
    assert_eq!(data_point.value, 1.0);
}
