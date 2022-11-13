use std::fmt;
use ::serde::Deserialize;
use time::{serde, Date, Duration, OffsetDateTime};

serde::format_description!(us_date, Date, "[month]/[day padding:none]/[year]");

#[derive(Debug, Deserialize)]
pub struct DomDay {
    account: String,
    recorder: String,
    #[serde(with = "us_date")]
    date: Date,
    kwh: Vec<f32>,
}

impl fmt::Display for DomDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut datetime = OffsetDateTime::now_local()
            .unwrap()
            .replace_date(self.date)
            .replace_hour(0)
            .unwrap()
            .replace_minute(0)
            .unwrap()
            .replace_second(0)
            .unwrap();

        for kwh in &self.kwh {
            writeln!(f,
                "dominion,account={account},recorder={recorder} kwh={kwh} {datetime}",
                account = self.account,
                recorder = self.recorder,
                datetime = datetime.unix_timestamp_nanos(),
                kwh = kwh
            )?;
            datetime = datetime
                .checked_add(Duration::minutes(30))
                .expect(&std::format!("Too many intervals for day {}", self.date))
        }
        Ok(())
    }
}

