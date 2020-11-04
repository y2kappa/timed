use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Duration;
use prettytable::{Table, Row, Cell};
use crate::{Phase, RecordBuffer};

#[derive(Clone, Debug)]
struct StatisticsRecord {
    name: String,
    calls: Vec<Duration>,
    overall_time: Duration,
    is_sorted: bool,
}

impl StatisticsRecord {
    fn new(name: String) -> StatisticsRecord {
        StatisticsRecord {
            name,
            calls: vec![],
            overall_time: Duration::from_nanos(0),
            is_sorted: false,
        }
    }

    pub fn nth_percentile_time(&mut self, percentile: f32) -> Option<&Duration> {
        if !self.is_sorted {
            self.calls.sort();
            self.is_sorted = true;
        }

        let mut i = (percentile * self.calls.len() as f32).round() as usize;
        if i > 0 {
            i -= 1;
        }

        return self.calls.get(i);
    }

    pub fn avg(&self) -> Duration {
        Duration::from_nanos(self.calls.iter()
            .sum::<Duration>().as_nanos() as u64 / self.calls.len() as u64)
    }
}

impl Ord for StatisticsRecord {
    // This is reversed so that longer time will be above shorter time after sorting.
    fn cmp(&self, other: &Self) -> Ordering {
        other.overall_time.cmp(&self.overall_time)
    }
}

impl PartialOrd for StatisticsRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for StatisticsRecord {
    fn eq(&self, other: &Self) -> bool {
        self.overall_time == other.overall_time
    }
}

impl Eq for StatisticsRecord {}

pub trait StatisticsExt {
    fn print_statistics(&mut self);
}

impl StatisticsExt for RecordBuffer {
    fn print_statistics(&mut self) {
        let mut stats = HashMap::new();

        self.drain()
            .iter()
            .for_each(|record| {
                if let Phase::Finish(d) = record.phase {
                    let mut entry = stats.entry(record.function_name.clone())
                        .or_insert(StatisticsRecord::new(record.function_name.clone()));
                    entry.calls.push(d);
                    entry.overall_time += d;
                }
            });

        let mut table = Table::new();
        table.add_row(row!["function name", "calls", "overall time", "avg time", "max time", "p90 time", "p50 time", "p10 time"]);
        stats.iter_mut().for_each(|(_, sr)| {
            table.add_row(row![
               sr.name,
               sr.calls.len(),
               format!("{:?}", sr.overall_time),
               format!("{:?}", sr.avg()),
               format!("{:?}", sr.nth_percentile_time(1.0).unwrap()),
               format!("{:?}", sr.nth_percentile_time(0.9).unwrap()),
               format!("{:?}", sr.nth_percentile_time(0.5).unwrap()),
               format!("{:?}", sr.nth_percentile_time(0.1).unwrap())
           ]);
        });

        table.printstd();
    }
}