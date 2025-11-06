use crate::api::BlsSeries;
use crate::error::{BlsError, Result};

#[derive(Debug, Clone, serde::Serialize)]
pub struct EmploymentChange {
    pub year: String,
    pub month: String,
    pub value: f64,
    pub change: Option<f64>,
    pub percent_change: Option<f64>,
}

pub fn calculate_changes(series: &BlsSeries) -> Result<Vec<EmploymentChange>> {
    if series.data.is_empty() {
        return Err(BlsError::NoData(series.series_id.clone()));
    }

    // Sort data chronologically (oldest first)
    let mut data_points = series.data.clone();
    data_points.sort_by(|a, b| {
        match a.year.cmp(&b.year) {
            std::cmp::Ordering::Equal => a.period.cmp(&b.period),
            other => other,
        }
    });

    let mut results = Vec::new();
    let mut previous_value: Option<f64> = None;

    for point in data_points.iter() {
        // Skip annual data (period "M13")
        if point.period == "M13" {
            continue;
        }

        let value = point.value.parse::<f64>()
            .map_err(|_| BlsError::InvalidInput(format!("Invalid value: {}", point.value)))?;

        let (change, percent_change) = if let Some(prev) = previous_value {
            let change = value - prev;
            let percent_change = if prev != 0.0 {
                (change / prev) * 100.0
            } else {
                0.0
            };
            (Some(change), Some(percent_change))
        } else {
            (None, None)
        };

        results.push(EmploymentChange {
            year: point.year.clone(),
            month: point.period_name.clone(),
            value,
            change,
            percent_change,
        });

        previous_value = Some(value);
    }

    Ok(results)
}

pub fn format_change(change: Option<f64>) -> String {
    match change {
        Some(c) => {
            if c >= 0.0 {
                format!("+{:.1}", c)
            } else {
                format!("{:.1}", c)
            }
        }
        None => "N/A".to_string(),
    }
}

pub fn format_percent(percent: Option<f64>) -> String {
    match percent {
        Some(p) => {
            if p >= 0.0 {
                format!("+{:.2}%", p)
            } else {
                format!("{:.2}%", p)
            }
        }
        None => "N/A".to_string(),
    }
}
