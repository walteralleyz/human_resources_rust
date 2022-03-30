use rocket::serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    pub fn parse_string(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }

    pub fn from_string(date: String) -> Date {
        let (mut year, mut month, mut day): (u16, u8, u8) = (0, 0, 0);
        let date_split = date.split("-").collect::<Vec<&str>>();

        year = date_split[0].trim().parse().unwrap();
        month = date_split[1].trim().parse().unwrap();
        day = date_split[2].trim().parse().unwrap();

        Date { day, month, year }
    }
}
