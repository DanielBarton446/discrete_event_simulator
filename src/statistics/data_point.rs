//! Retains a timestamp, value, and unit for a single data point.

/// # DataPoint
/// follows WORM(write once, read many) pattern. Immutable.
/// Contains a timestamp, value, and unit for a single data point.
/// Must be created with `DataPoint::new()`.
pub struct DataPoint {
    timestamp: usize,
    value: f64,
    unit: String,
}

impl DataPoint {
    /// Create a new DataPoint.
    ///
    /// ```
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// assert_eq!(data_point.get_timestamp(), 0);
    /// assert_eq!(data_point.get_value(), 1.0);
    /// assert_eq!(data_point.get_unit(), "fake_unit");
    /// ```
    pub fn new(timestamp: usize, value: f64, unit: String) -> DataPoint {
        DataPoint {
            timestamp,
            value,
            unit,
        }
    }

    /// Get the timestamp of the data point.
    /// ```
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// assert_eq!(data_point.get_timestamp(), 0);
    /// ```
    pub fn get_timestamp(&self) -> usize {
        self.timestamp
    }

    /// Get the value of the data point.
    /// ```
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// assert_eq!(data_point.get_value(), 1.0);
    /// ```
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Get the unit of the data point.
    /// ```
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    ///
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// assert_eq!(data_point.get_unit(), "fake_unit");
    /// ```
    pub fn get_unit(&self) -> &str {
        &self.unit
    }
}

#[test]
fn create_datapoint() {
    let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    assert_eq!(data_point.timestamp, 0);
    assert_eq!(data_point.value, 1.0);
    assert_eq!(data_point.unit, "fake_unit");
}
