use std::collections::HashMap;
use csv::StringRecord;

pub struct ParticipantAverage {
    pub pss: f64,
    pub psqi: f64,
}

pub struct Dataset {
    data: HashMap<u32, Vec<(f64, f64)>>,
}

impl Dataset {
    pub fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = csv::Reader::from_path(file_path)?;
        let mut data = HashMap::new();

        for result in reader.records() {
            let record: StringRecord = result?;
            let participant_id: u32 = record.get(0).unwrap().parse()?;
            let pss: f64 = record.get(2).unwrap().parse()?;
            let psqi: f64 = record.get(11).unwrap().parse()?;

            data.entry(participant_id)
                .or_insert_with(Vec::new)
                .push((pss, psqi));
        }

        Ok(Self { data })
    }

    pub fn compute_averages(&self) -> Vec<ParticipantAverage> {
        self.data
            .iter()
            .map(|(_, scores)| {
                let (total_pss, total_psqi) = scores.iter().fold((0.0, 0.0), |(acc_pss, acc_psqi), (pss, psqi)| {
                    (acc_pss + pss, acc_psqi + psqi)
                });
                let count = scores.len() as f64;
                ParticipantAverage {
                    pss: total_pss / count,
                    psqi: total_psqi / count,
                }
            })
            .collect()
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_averages() {
        let mut data = Dataset {
            data: std::collections::HashMap::new(),
        };

        data.data.insert(1, vec![(10.0, 2.0), (20.0, 4.0)]);
        let averages = data.compute_averages();
        assert_eq!(averages.len(), 1);
        assert!((averages[0].pss - 15.0).abs() < 1e-6, "Expected average PSS of 15.0");
        assert!((averages[0].psqi - 3.0).abs() < 1e-6, "Expected average PSQI of 3.0");
    }
}
