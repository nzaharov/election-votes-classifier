#[warn(clippy::all)]
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::hash_map::Entry::{Occupied, Vacant};
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

const EPOCHS: usize = 10;

fn main() -> io::Result<()> {
    let mut reader = csv::Reader::from_path("./data/house-votes-84.csv")?;

    let headers = reader.headers().expect("CSV headers missing!").clone();

    let mut records: Vec<Record> = reader
        .deserialize::<Record>()
        .filter_map(|r| r.ok())
        .collect();

    let mut rng = thread_rng();
    records.shuffle(&mut rng);

    let chunk_size = records.len() / EPOCHS;

    let mut average_hit_rate = 0.0;

    for attempt in 0..EPOCHS {
        let mut raw_stats: Stats = headers
            .iter()
            .map(|header| (header.into(), HashMap::new()))
            .collect();

        let mut test_chunk: Vec<Record> = vec![];

        // Populate stats
        for (i, chunk) in records.chunks(chunk_size).enumerate() {
            if i == attempt {
                test_chunk = chunk.to_owned();
                continue;
            }
            for record in chunk {
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
        }

        let mut local_hits = 0;
        let mut local_misses = 0;

        for mut record in test_chunk {
            let actual_vote = record.remove("Class Name").unwrap();
            let classes = raw_stats.get("Class Name").unwrap();

            let mut democrat_chance = record.iter().fold(1.0, |acc, (key, value)| {
                let vote_stat = raw_stats.get(key).unwrap().get(value).unwrap();
                acc * vote_stat.democrat as f32 / vote_stat.total as f32
            });
            democrat_chance *= classes["democrat"].total as f32;

            let mut republican_chance = record.iter().fold(1.0, |acc, (key, value)| {
                let vote_stat = raw_stats.get(key).unwrap().get(value).unwrap();
                acc * vote_stat.republican as f32 / vote_stat.total as f32
            });
            republican_chance *= classes["republican"].total as f32;

            let vote_prediciton = match democrat_chance < republican_chance {
                true => "republican",
                false => "democrat",
            };

            if vote_prediciton == actual_vote {
                local_hits += 1;
            } else {
                local_misses += 1;
            }
        }

        let probability = local_hits as f32 / (local_hits + local_misses) as f32;
        average_hit_rate += probability / 10.0;

        println!("Epoch {}: {} {}", attempt, local_hits, local_misses);
        println!("{}", probability);
    }

    println!("Average: {}", average_hit_rate);

    Ok(())
}
