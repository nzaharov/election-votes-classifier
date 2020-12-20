use std::collections::hash_map::Entry::{Occupied, Vacant};
#[warn(clippy::all)]
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct VoteAttributeStat {
    republican: u32,
    democrat: u32,
    total: u32,
}

impl VoteAttributeStat {
    fn empty() -> Self {
        Self {
            republican: 0,
            democrat: 0,
            total: 0,
        }
    }

    fn increment_left(&mut self) {
        self.democrat += 1;
        self.total += 1;
    }

    fn increment_right(&mut self) {
        self.republican += 1;
        self.total += 1;
    }
}

type Record = HashMap<String, String>;
type Stats = HashMap<String, HashMap<String, VoteAttributeStat>>;

fn main() -> io::Result<()> {
    let mut reader = csv::Reader::from_path("./data/house-votes-84.csv")?;

    let headers = reader.headers().expect("CSV headers missing!");

    let mut raw_stats: Stats = headers
        .iter()
        .map(|header| (header.into(), HashMap::new()))
        .collect();

    let records = reader.deserialize::<Record>();

    // Populate stats
    for record in records.filter_map(|r| r.ok()) {
        for (attr, val) in record.iter() {
            let attr_raw_stats = raw_stats.get_mut(attr).unwrap();
            match attr_raw_stats.entry(val.into()) {
                Vacant(entry) => {
                    entry.insert(VoteAttributeStat::empty());
                }
                Occupied(entry) => {
                    if record.get("Class Name").unwrap() == "democrat" {
                        entry.into_mut().increment_left();
                    } else {
                        entry.into_mut().increment_right();
                    }
                }
            };
        }
    }

    println!(
        "{}",
        raw_stats
            .iter()
            .fold(String::new(), |acc, stat| acc + &format!("{:?}\n", stat))
    );

    Ok(())
}
