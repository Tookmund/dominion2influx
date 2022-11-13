use std::io;
use std::error::Error;

use ::serde::Deserialize;
use time::{serde,Date,OffsetDateTime,Duration};


serde::format_description!(us_date, Date, "[month]/[day padding:none]/[year]");

#[derive(Debug,Deserialize)]
struct DomDay {
    account: String,
    recorder: String,
    #[serde(with = "us_date")]
    date: Date,
    kwh: Vec<f32>
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for result in reader.deserialize() {
        let domday: DomDay = result?;
        eprintln!("{:?}", domday);

        let mut datetime = OffsetDateTime::now_local().unwrap()
            .replace_date(domday.date)
            .replace_hour(0).unwrap()
            .replace_minute(0).unwrap()
            .replace_second(0).unwrap();

        for kwh in domday.kwh {
            println!("dominion,account={account},recorder={recorder} kwh={kwh} {datetime}",
                  account = domday.account,
                  recorder = domday.recorder,
                  datetime = datetime.unix_timestamp_nanos(),
                  kwh = kwh
                );
            datetime = datetime.checked_add(Duration::minutes(30))
            .expect(&std::format!("Too many intervals for day {}", domday.date))
        }
    }
    Ok(())
}
