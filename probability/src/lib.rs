/*
DIY project no. 25002
Write a trait for a probability model.

DIY project no. 25005
Write a rust crate to implement a trait for a probability model.
Use the environmental data to create data models and help users to make informed decisions.


*/

use rand::Rng;
use std::collections::HashSet;

pub const SECONDS: f32 = 1.0;
pub const MINUTES: f32 = 60.0 * SECONDS;
pub const HOURS: f32 = 60.0 * MINUTES;


pub trait Probability {
    fn simple_random_sampling(&self, data_x: &Vec<EnvinmentalData>, sample: i64) -> Vec<EnvinmentalData>;
    fn stratified_random_sampling(&self, sample: usize, month_a: String, month_b: String, month_c: String) -> (Vec<EnvinmentalData>, Vec<EnvinmentalData>, Vec<EnvinmentalData>);
    fn mean(&self, option: &str) -> f32;
    fn median(&self, option: &str) -> f32;
    fn mode(&self, option: &str) -> f32;
    fn outcome_likelihood(&self, event: &str) -> (f32, String);
}

#[derive(Clone, Debug)]
pub struct EnvinmentalData {
    pub day: i32,
    pub date: String,
    pub month: String,
    pub day_of_the_week: String,
    pub time: String,
    pub location: String,
    pub temperature: f32,
    pub smoke_detected: bool,
    pub smoke_duration: f32,
    pub comments: String,
    pub raining: bool,
    pub rain_duration: f32,
    pub toxic_detected: bool,
    pub toxic_smell_duration: f32,
    pub earthquake: bool,
    pub earthquake_duration: f32,
}

#[derive(Clone, Debug)]
pub struct EnvinmentalModel {
    pub population: Vec<EnvinmentalData>,
    pub sample: Vec<EnvinmentalData>,
}

#[derive(Clone)]
pub struct FrequencyMap {
    pub key: f32,
    pub value: i32,
}

impl Probability for EnvinmentalModel {
    fn simple_random_sampling(&self, data_x: &Vec<EnvinmentalData>, sample: i64) -> Vec<EnvinmentalData> {
        let len = data_x.len();
        assert!(len >= sample.try_into().unwrap());
        let mut i = 0;
        let mut sample_list: HashSet<i64> = HashSet::new();
        let mut x = get_random(0, len - 1);
        let mut xy: i64 = x.try_into().unwrap();
        let mut added: bool = sample_list.insert(xy);
        println!("sample: {}, len: {}, xy: {}, added: {}", sample, len, xy, added);
        let mut sample_data: Vec<EnvinmentalData> = Vec::new();
        while i < sample {
            let data = data_x[x].clone();
            sample_data.push(data);
            x = get_random(0, len - 1);
            xy = x.try_into().unwrap();
            added = sample_list.insert(xy);
            println!("xy: {}, added: {}", xy, added);
            while !added {
                added = sample_list.insert(xy);
                x = get_random(0, len - 1);
                xy = x.try_into().unwrap();
            }
            i += 1;
        }
        sample_data
    }

    fn stratified_random_sampling(&self, sample: usize, month_a: String, month_b: String, month_c: String) -> (Vec<EnvinmentalData>, Vec<EnvinmentalData>, Vec<EnvinmentalData>) {
        let mut group_data_a: Vec<EnvinmentalData> = Vec::new();
        let mut group_data_b: Vec<EnvinmentalData> = Vec::new();
        let mut group_data_c: Vec<EnvinmentalData> = Vec::new();
        let len = self.population.len();
        assert!(len >= sample);
        for x in &self.population {
            let month_x = x.month.clone();
            if month_x == month_a {
                group_data_a.push(x.clone());
            } else if month_x == month_b {
                group_data_b.push(x.clone());
            } else if month_x == month_c {
               group_data_c.push(x.clone());
            }
        }
        let frequency_data_a = group_data_a.len();
        let frequency_data_b = group_data_b.len();
        let frequency_data_c = group_data_c.len();
        let total_frequency = frequency_data_a + frequency_data_b + frequency_data_c;
        let mut sample_x = sample;
        if sample_x > total_frequency {
            sample_x = total_frequency;
        }
        let element_a = (frequency_data_a * sample_x) / total_frequency;
        let element_b = (frequency_data_b * sample_x) / total_frequency;
        let element_c = (frequency_data_c * sample_x) / total_frequency;
        let sample_a = self.simple_random_sampling(&group_data_a, element_a.try_into().unwrap());
        let sample_b = self.simple_random_sampling(&group_data_b, element_b.try_into().unwrap());
        let sample_c = self.simple_random_sampling(&group_data_c, element_c.try_into().unwrap());

        (sample_a, sample_b, sample_c)
    }

    fn mean(&self, option: &str) -> f32 {
        let len = self.sample.len();
        assert!(len > 0);
        let mut observation = 0.00;
        for x in &self.sample {
            if option == "temperature" {
                observation += x.temperature;
            } else if option == "smoke" {
                observation += x.smoke_duration;
            } else if option == "rain" {
                observation += x.rain_duration;
            } else if option == "toxic" {
                observation += x.toxic_smell_duration;
            } else if option == "earthquake" {
                observation += x.earthquake_duration;
            } else {
                assert!(option == "temperature");
            }
        }
        let len_x: f32 = len as f32;
        observation / len_x
    }

    fn median(&self, option: &str) -> f32 {
        let len = self.sample.len();
        assert!(len > 0);
        let mut duration_sample = local_sample(&self.sample, option);
        duration_sample.sort_by(|a, b| a.partial_cmp(b).unwrap());

        sample_median(&duration_sample)
    }

    fn mode(&self, option: &str) -> f32 {
        let mut frequency_table: Vec<FrequencyMap> = Vec::new();
        let mut mode_sample = local_sample(&self.sample, option);
        mode_sample.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut x = mode_sample[0];
        let mut i = 0;
        for key in mode_sample {
            if key == x {
                i += 1;
            } else {
                frequency_table.push(FrequencyMap{key: x , value: i,});
                x = key;
                i = 0;
            }
        }

        if i != 0 {
            frequency_table.push(FrequencyMap{key: x , value: i,});
        }

        let mut max_frequency = frequency_table[0].clone();
        for y in frequency_table {
            if max_frequency.value < y.value {
                max_frequency = y;
            }
        }

        max_frequency.key
    }

    fn outcome_likelihood(&self, event: &str) -> (f32, String) {
        let mut event_x: f32 = 0.00;
        let mut total_events_x: f32 = 0.00;
        for x in &self.sample {
            total_events_x += 1.00;
            if (event == "smoke") & x.smoke_detected {
                event_x += 1.00;
            } else if (event == "rain") & x.raining {
                event_x += 1.00;
            } else if (event == "toxic") & x.toxic_detected {
                event_x += 1.00;
            } else if (event == "earthquake") & x.earthquake {
                event_x += 1.00;
            } else {
                assert!(event == "rain");
            }
        }
        assert!(total_events_x > 0.00);
        let likelihood: f32 = event_x / total_events_x;
        (likelihood, String::from(event))
    }
}

pub fn get_random(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..=max)
}

pub fn local_sample(sample: &Vec<EnvinmentalData>, option: &str) -> Vec<f32> {
    let len = sample.len();
    assert!(len > 0);
    let mut duration_sample: Vec<f32> = Vec::new();
    for x in sample {
        if option == "temperature" {
            duration_sample.push(x.temperature);
        } else if option == "smoke" {
            duration_sample.push(x.smoke_duration);
        } else if option == "rain" {
            duration_sample.push(x.rain_duration);
        } else if option == "toxic" {
            duration_sample.push(x.toxic_smell_duration);
        } else if option == "earthquake" {
            duration_sample.push(x.earthquake_duration);
        } else {
            assert!(option == "temperature");
        }
    }
    duration_sample
}

pub fn sample_median(sample: &Vec<f32>) -> f32 {
    let len = sample.len();
    let median_x = (len + 1) / 2;
    if (len + 1 ) % 2 != 0 {
        let x1 = len / 2;
        let x2 = x1 + 1;
        let median_x1 = sample[x1];
        let median_x2 = sample[x2];
        let median = (median_x1 + median_x2) / 2.00;
        median
    } else {
        let median = sample[median_x];
        median
    }
}

impl EnvinmentalModel {
    pub fn simple_sampling(&self, data_x: &Vec<EnvinmentalData>, sample: i64) -> Vec<EnvinmentalData> {
        self.simple_random_sampling(data_x, sample)
    }

    pub fn get_mean(&self, option: &str) -> f32 {
        self.mean(option)
    }

    pub fn get_outcome_likelihood(&self, event: &str) -> (f32, String) {
        self.outcome_likelihood(event)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_simple_random_sampling() {
        let data_x1 = EnvinmentalData {
            day: 100,
            date: "2024/05/15".to_string(),
            month: "May".to_string(),
            day_of_the_week: "Wednesday".to_string(),
            time: "14:00:00".to_string(),
            location: "Road".to_string(),
            temperature: 19.67,
            smoke_detected: true,
            smoke_duration: 6.0 * HOURS,
            comments: String::new(),
            raining: false,
            rain_duration: 0.00,
            toxic_detected: true,
            toxic_smell_duration: 300.00,
            earthquake: false,
            earthquake_duration: 0.00,
        };
        let data_x2 = EnvinmentalData {
            day: 99,
            date: "2024/05/14".to_string(),
            month: "May".to_string(),
            day_of_the_week: "Tuesday".to_string(),
            time: "15:00:00".to_string(),
            location: "Road".to_string(),
            temperature: 15.67,
            smoke_detected: true,
            smoke_duration: 600.00,
            comments: String::new(),
            raining: true,
            rain_duration: 25.00 * MINUTES,
            toxic_detected: true,
            toxic_smell_duration: 300.00,
            earthquake: false,
            earthquake_duration: 0.00,
        };
        let data_x3 = EnvinmentalData {
            day: 98,
            date: "2024/05/13".to_string(),
            month: "May".to_string(),
            day_of_the_week: "Monday".to_string(),
            time: "09:00:00".to_string(),
            location: "Road".to_string(),
            temperature: 23.67,
            smoke_detected: true,
            smoke_duration: 6000.00,
            comments: String::new(),
            raining: false,
            rain_duration: 0.00,
            toxic_detected: true,
            toxic_smell_duration: 35.0 * SECONDS,
            earthquake: false,
            earthquake_duration: 0.00,
        };
        let mut data_x: Vec<EnvinmentalData> = Vec::new();
        data_x.push(data_x1);
        data_x.push(data_x2);
        data_x.push(data_x3);
        let mut model = EnvinmentalModel {
            population: data_x,
            sample: Vec::new(),
        };
        model.sample = model.simple_random_sampling(&model.population, 2);
        let mean_x = model.mean("temperature");
        assert!((mean_x == 17.67) | (mean_x == 19.67) | (mean_x == 21.67) );
        let likelihood = model.get_outcome_likelihood("rain");
        assert!((likelihood.0 == 0.00) | (likelihood.0 == 0.50));

    }
}
