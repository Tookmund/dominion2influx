use std::io;
use std::error::Error;

use ::serde::Deserialize;
use time::{serde,Date,OffsetDateTime,Duration,format_description::well_known::Rfc3339};


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
    println!("#datatype,_measurement,string,dateTime:RFC3339,double
#group,true,false,false,false
#default,,,,
,_measurement,_field,_time,_value");
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
            println!(",{account},{recorder},{datetime},{kwh}",
                  account = domday.account,
                  recorder = domday.recorder,
                  datetime = datetime.format(&Rfc3339)
                  .expect(&std::format!("Invalid datetime: {}", datetime)),
                  kwh = kwh
                );
            datetime = datetime.checked_add(Duration::minutes(30))
            .expect(&std::format!("Too many intervals for day {}", domday.date))
        }
    }
    Ok(())
}
