//! Used to record a series of [DataPoint]s for a given statistic.

use super::data_point::DataPoint;
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

/// Contains a label for the series description,
/// the unit of measurement, and a series of [DataPoint]s.
#[derive(Clone)]
pub struct TimeSeries {
    pub statistic_label: String,
    pub unit: String,
    pub series: BTreeMap<usize, f64>,
}

impl TimeSeries {
    /// Create a new TimeSeries. Assumes the unit is "Unknown" until a [DataPoint] is added.
    ///
    /// ```
    /// use discrete_event_simulator::statistics::timeseries::TimeSeries;
    ///
    /// let time_series = TimeSeries::new("test".to_string());
    ///
    /// assert_eq!(time_series.statistic_label, "test");
    /// assert_eq!(time_series.series.len(), 0);
    /// assert_eq!(time_series.unit, "Unknown");
    /// ```
    pub fn new(statistic_label: String) -> TimeSeries {
        TimeSeries {
            statistic_label,
            unit: String::from("Unknown"),
            series: BTreeMap::new(),
        }
    }

    /// Add a [DataPoint] to the series. Updates this series' unit if it is the first data point
    /// added to the series.
    /// ```
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    /// use discrete_event_simulator::statistics::timeseries::TimeSeries;
    ///
    /// let mut time_series = TimeSeries::new("test".to_string());
    /// // unit is not known, as the timeseries has no data points
    /// assert_eq!(time_series.unit, "Unknown");
    ///
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// time_series.add_data_point(&data_point);
    ///
    /// assert_eq!(time_series.series.len(), 1);
    /// assert_eq!(time_series.series.get(&0), Some(&1.0));
    /// // unit is now known, as the timeseries has a data point
    /// assert_eq!(time_series.unit, "fake_unit");
    /// ```
    pub fn add_data_point(&mut self, data_point: &DataPoint) {
        if self.unit == "Unknown" {
            self.unit = data_point.get_unit().to_string();
        }
        self.series
            .insert(data_point.get_timestamp(), data_point.get_value());
    }

    /// Get the last value in the series. Useful in scenarios
    /// like a simulation where you are recording values at each timestep,
    /// but you strictly need the final result.
    ///
    /// ```
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    /// use discrete_event_simulator::statistics::timeseries::TimeSeries;
    ///
    /// let mut time_series = TimeSeries::new("test".to_string());
    /// let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
    /// let data_point_new = DataPoint::new(20, 2.0, String::from("fake_unit"));
    /// time_series.add_data_point(&data_point);
    /// time_series.add_data_point(&data_point_new);
    ///
    /// assert_eq!(time_series.get_last_value(), 2.0);
    /// ```
    pub fn get_last_value(&self) -> f64 {
        match self.series.iter().last() {
            Some((_, value)) => *value,
            None => panic!("No data points in series"),
        }
    }
}

impl Display for TimeSeries {
    /// Display the series in a human-readable format for timestamps under 9 digits.
    /// ```
    /// use discrete_event_simulator::statistics::data_point::DataPoint;
    /// use discrete_event_simulator::statistics::timeseries::TimeSeries;
    ///
    /// let mut time_series = TimeSeries::new("test".to_string());
    /// let data_point = DataPoint::new(0, 1.1, String::from("fake_unit"));
    /// time_series.add_data_point(&data_point);
    /// println!("{}", time_series);
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for _ in self.statistic_label.chars() {
            output.push('=');
        }
        output.push('\n');
        output.push_str(&format!("{}\n", self.statistic_label));
        for _ in self.statistic_label.chars() {
            output.push('=');
        }
        output.push('\n');
        output.push_str(&format!("Timestamp | {}\n", self.unit));
        for (timestamp, value) in &self.series {
            output.push_str(&format!("{:<9} | {}\n", timestamp, value));
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::statistics::{data_point::DataPoint, timeseries::TimeSeries};

    #[test]
    fn create_new_timeseries() {
        let time_series = TimeSeries::new("test".to_string());
        assert_eq!(time_series.statistic_label, "test");
        assert_eq!(time_series.series.len(), 0);
    }

    #[test]
    fn add_data_point_to_timeseries() {
        let mut time_series = TimeSeries::new("test".to_string());
        let data_point = DataPoint::new(0, 1.0, String::from("fake_unit"));
        time_series.add_data_point(&data_point);
        assert_eq!(time_series.series.len(), 1);
        assert_eq!(time_series.series.get(&0), Some(&1.0));
    }

    #[test]
    fn display_simple_timeseries() {
        let mut time_series = TimeSeries::new("test".to_string());
        let data_point = DataPoint::new(0, 1.1, String::from("fake_unit"));
        time_series.add_data_point(&data_point);
        let expected_output =
            fs::read_to_string("./test_data/timeseries_simple_expected_display.txt")
                .expect("Failed to read test data for simple timeseries display");
        dbg!(&expected_output);
        assert_eq!(expected_output, format!("{}", time_series));
    }

    #[test]
    fn display_multiple_datapoints() {
        let mut time_series = TimeSeries::new("test".to_string());
        let data_point = DataPoint::new(0, 1.1, String::from("fake_unit"));
        let data_point_new = DataPoint::new(20, 2.2, String::from("fake_unit"));
        time_series.add_data_point(&data_point);
        time_series.add_data_point(&data_point_new);
        let expected_output =
            fs::read_to_string("./test_data/timeseries_multiple_data_expected_display.txt")
                .expect("Failed to read test data for multiple datapoints timeseries display");
        dbg!(&expected_output);
        assert_eq!(expected_output, format!("{}", time_series));
    }
}
