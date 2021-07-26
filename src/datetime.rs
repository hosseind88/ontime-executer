use chrono::TimeZone;

pub struct DateTime<'a> {
    datetime: &'a str,
}

impl<'a> DateTime<'a> {
    pub fn new(datetime: &'a str) -> Self {
        return DateTime { datetime };
    }

    pub fn get_time_duration_to_run(&self) -> chrono::Duration {
        let mut splited_datetime = self.datetime.split(" ");
        let date = splited_datetime.next().unwrap_or_default();
        let time = splited_datetime.next().unwrap_or_default();
        let mut splited_date = date.split("-");
        let day = splited_date.next().unwrap_or_default();
        let month = splited_date.next().unwrap_or_default();
        let year = splited_date.next().unwrap_or_default();
        let mut splited_time = time.split(":");
        let hour = splited_time.next().unwrap_or_default();
        let minute = splited_time.next().unwrap_or_default();
        let seconds = splited_time.next().unwrap_or_default();

        let future_time = chrono::Local
            .ymd(
                year.parse::<i32>().unwrap(),
                month.parse::<u32>().unwrap(),
                day.parse::<u32>().unwrap(),
            )
            .and_hms(
                hour.parse::<u32>().unwrap(),
                minute.parse::<u32>().unwrap(),
                seconds.parse::<u32>().unwrap(),
            );
        let now = chrono::Local::now();

        return future_time.signed_duration_since(now);
    }
}
