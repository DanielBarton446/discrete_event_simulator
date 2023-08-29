pub struct DataPoint {
    pub timestamp: usize,
    pub value: f64,
}

impl DataPoint {
    pub fn new(timestamp: usize, value: f64) -> DataPoint {
        DataPoint { timestamp, value }
    }
}

#[test]
fn create_datapoint() {
    let data_point = DataPoint::new(0, 1.0);
    assert_eq!(data_point.timestamp, 0);
    assert_eq!(data_point.value, 1.0);
}
