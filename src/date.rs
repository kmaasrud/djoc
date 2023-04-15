use std::fmt::Display;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use toml::value::Datetime;

/// A naïve (no timezone) datetime for use in a document.
///
/// Both the date and time are optional, meaning this struct is not necessarily
/// specified at all (for instance when it is constructed with `Default`.)
#[derive(Default, PartialEq, Eq)]
pub struct Date {
    date: Option<NaiveDate>,
    time: Option<NaiveTime>,
}

impl Date {
    /// Create a new date from a year, month and day.
    ///
    /// The values will be clamped to the valid ranges (i.e. 31 for day and 12
    /// for month.)
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        Self {
            date: NaiveDate::from_ymd_opt(year, month.min(12), day.min(31)),
            time: None,
        }
    }

    /// Create a new date from a year, month, day, hour, minute and second.
    ///
    /// The values will be clamped to the valid ranges (i.e. 31 for day and 12
    /// for month.)
    pub fn from_ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Self {
        Self {
            date: NaiveDate::from_ymd_opt(year, month.min(12), day.min(31)),
            time: NaiveTime::from_hms_opt(hour.min(23), minute.min(59), second.min(59)),
        }
    }

    /// Create a new date from hour, minute and second.
    ///
    /// The values will be clamped to the valid ranges (i.e. 23 for hour, 59
    /// for minute and second.)
    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            date: None,
            time: NaiveTime::from_hms_opt(hour.min(23), minute.min(59), second.min(59)),
        }
    }

    /// Set the time.
    ///
    /// The values will be clamped to the valid ranges (i.e. 23 for hour, 59
    /// for minute and second.)
    pub fn time(&mut self, hour: u32, minute: u32, second: u32) -> &mut Self {
        self.time = NaiveTime::from_hms_opt(hour.min(23), minute.min(59), second.min(59));
        self
    }

    /// Set the date.
    ///
    /// The values will be clamped to the valid ranges (i.e. 31 for day and 12
    /// for month.)
    pub fn date(&mut self, year: i32, month: u32, day: u32) -> &mut Self {
        self.date = NaiveDate::from_ymd_opt(year, month.min(12), day.min(31));
        self
    }

    /// Format the date using a locale. Will return `None` if neither the date
    /// nor the time are specified.
    ///
    /// The locale must be a valid locale as listed in the
    /// [`pure-rust-locales`] crate. In general, most [BCP 47] language tags are
    /// valid.
    ///
    /// [`pure-rust-locales`]: https://docs.rs/pure-rust-locales
    /// [BCP 47]: https://tools.ietf.org/html/bcp47
    pub fn format_with_locale(&self, locale: &str) -> Option<String> {
        locale
            .try_into()
            .ok()
            .and_then(|locale| match (self.date, self.time) {
                (Some(date), Some(time)) => Some(format!(
                    "{} {}",
                    date.format_localized("%e %B %Y", locale),
                    time.format("%H:%M")
                )),
                (Some(date), None) => Some(date.format_localized("%e %B %Y", locale).to_string()),
                (None, Some(time)) => Some(time.format("%H:%M").to_string()),
                _ => None,
            })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.date, self.time) {
            (Some(date), Some(time)) => write!(f, "{} {}", date, time),
            (Some(date), None) => write!(f, "{}", date),
            (None, Some(time)) => write!(f, "{}", time),
            _ => write!(f, ""),
        }
    }
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Self {
            date: Some(date),
            time: None,
        }
    }
}

impl From<NaiveTime> for Date {
    fn from(time: NaiveTime) -> Self {
        Self {
            date: None,
            time: Some(time),
        }
    }
}

impl From<NaiveDateTime> for Date {
    fn from(dt: NaiveDateTime) -> Self {
        Self {
            date: Some(dt.date()),
            time: Some(dt.time()),
        }
    }
}

impl From<Datetime> for Date {
    fn from(dt: Datetime) -> Self {
        Self {
            date: dt.date.and_then(|date| {
                NaiveDate::from_ymd_opt(date.year.into(), date.month.into(), date.day.into())
            }),
            time: dt.time.and_then(|time| {
                NaiveTime::from_hms_opt(time.hour.into(), time.minute.into(), time.second.into())
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date() {
        let date = Date::from_ymd(2020, 1, 1);
        assert_eq!(date.to_string(), "2020-01-01");
        assert_eq!(
            date.format_with_locale("en_US"),
            Some(" 1 January 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("en_GB"),
            Some(" 1 January 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("de_DE"),
            Some(" 1 Januar 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("fr_FR"),
            Some(" 1 janvier 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("es_ES"),
            Some(" 1 enero 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("zh_CN"),
            Some(" 1 一月 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("ja_JP"),
            Some(" 1 1月 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("ko_KR"),
            Some(" 1 1월 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("ru_RU"),
            Some(" 1 января 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("ar_SA"),
            Some(" 1 يناير 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("fa_IR"),
            Some(" 1 ژانویه 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("he_IL"),
            Some(" 1 ינואר 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("th_TH"),
            Some(" 1 มกราคม 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("vi_VN"),
            Some(" 1 Tháng 1 2020".to_string())
        );
        assert_eq!(
            date.format_with_locale("hi_IN"),
            Some(" 1 जनवरी 2020".to_string())
        );
    }

    #[test]
    fn test_time() {
        let time = Date::from_hms(12, 34, 56);
        assert_eq!(time.to_string(), "12:34:56");
        assert_eq!(time.format_with_locale("en_US"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("en_GB"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("de_DE"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("fr_FR"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("es_ES"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("zh_CN"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("ja_JP"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("ko_KR"), Some("12:34".to_string()));
        assert_eq!(time.format_with_locale("ru_RU"), Some("12:34".to_string()));
    }
}
