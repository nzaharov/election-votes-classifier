#[warn(clippy::all)]
use std::collections::HashMap;
use std::io;

type Record = HashMap<String, String>;
type Stats = HashMap<String, HashMap<String, i32>>;

fn main() -> io::Result<()> {
    let mut reader = csv::Reader::from_path("./data/house-votes-84.csv")?;

    let headers = reader.headers().expect("CSV headers missing!");

    let mut stats: Stats = headers.into_iter().fold(HashMap::new(), |mut map, header| {
        map.insert(header.into(), HashMap::new());
        map
    });

    let records = reader.deserialize::<Record>();

    for record in records.filter_map(|r| r.ok()) {
        for (attr, val) in record.into_iter() {
            let attr_stats = stats.get_mut(&attr).unwrap();
            *attr_stats.entry(val).or_insert(0) += 1;
        }
    }

    println!(
        "{}",
        stats
            .iter()
            .fold(String::new(), |acc, stat| acc + &format!("{:?}\n", stat))
    );

    Ok(())
}
