use std::cmp::Ordering;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct StatisticsRecord {
    pub(crate) name: String,
    pub(crate) calls: usize,
    pub(crate) overall_time: Duration
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

#[derive(Debug)]
pub struct StatisticsResult {
    records: Vec<StatisticsRecord>
}

impl StatisticsResult {
    pub(crate) fn from_raw_map(stats_map: &HashMap<String, Vec<Duration>>) -> StatisticsResult {
        let mut fn_stats = vec![];
        stats_map.iter().for_each(|(k, v)| {
            let current_total = v.iter().map(|d| d.as_nanos()).sum::<u128>() as u64;
            fn_stats.push(StatisticsRecord {
                name: k.to_string(),
                calls: v.len(),
                overall_time: Duration::from_nanos(current_total),
            });
        });

        fn_stats.sort();

        return StatisticsResult {
            records: fn_stats
        }
    }
}