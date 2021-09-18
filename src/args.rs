use chrono::{Date, Local, TimeZone};
use regex::Regex;
use std::{
    borrow::Cow,
    env,
    num::{IntErrorKind, NonZeroU32},
};

#[derive(Debug)]
pub enum Specifier {
    // The latest commit.
    Latest,
    // The first commit ever made.
    First,
    // The latest commit of today (if any).
    Today,
    // The latest commit made on that date (if any).
    Date(Date<Local>),
}

impl Specifier {
    fn parse(str: &str) -> Result<Self, Error> {
        match str {
            "latest" => Ok(Self::Latest),
            "first" => Ok(Self::First),
            "today" => Ok(Self::Today),
            _ => {
                let year;
                let month;
                let day;

                // TODO: put this in a const once possible
                let date_regex: Regex =
                    Regex::new(r"([0-9]{4})/([0-9]{1,2})/([0-9]{1,2})").unwrap();

                if let Some(caps) = date_regex.captures(str) {
                    year = caps.get(1).unwrap().as_str();
                    month = caps.get(2).unwrap().as_str();
                    day = caps.get(3).unwrap().as_str();

                    match (
                        year.parse::<NonZeroU32>(),
                        month.parse::<NonZeroU32>(),
                        day.parse::<NonZeroU32>(),
                    ) {
                        (Ok(year), Ok(month), Ok(day)) => {
                            let date = Local.ymd(u32::from(year) as i32, month.into(), day.into());

                            Ok(Self::Date(date))
                        }
                        _ => Err("invalid date. TODO: add date format help here".into()),
                    }
                } else {
                    Err("unexpected something".into())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Selector {
    pub specifier: Specifier,
    pub offset: Option<i16>, // TODO use it
}

// A range of selected commits.
#[derive(Debug)]
pub struct Range {
    pub start: Selector,
    pub end: Selector,
}

#[derive(Debug)]
pub struct Args {
    pub repository_path: String,
    pub range: Range,
}

type Error = Cow<'static, str>;

const RANGE_HELP: &str = concat!(
    "range format: `start..end`\n",
    "`start` and `end` can be either of the following:\n",
);

const HELP: &str = concat!(
    "generate changelogs with git commits.\n",
    "usage: logit (path to local repository) (range)\n",
    "example: `logit logit/ today..first` to generate a changelog using commits ranging from commits made today to the first commits ever\n",
    "the range is always inclusive"
);

fn warn(msg: &str) {
    eprintln!("warning: {}", msg);
}

fn parse_selector(str: &str) -> Result<Selector, Error> {
    match str.split_once(&['-', '+'][..]) {
        Some((specifier, offset)) => {
            let specifier = Specifier::parse(specifier)?;

            match offset.parse::<u16>() {
                Ok(offset) => {
                    if str.contains('-') {
                        Ok(Selector {
                            specifier,
                            offset: Some(-(offset as i16)),
                        })
                    } else if str.contains('+') {
                        Ok(Selector {
                            specifier,
                            offset: Some(offset as i16),
                        })
                    } else {
                        unreachable!();
                    }
                }
                Err(err) => match err.kind() {
                    IntErrorKind::PosOverflow => {
                        return Err(
                            format!("offset must be in range {}..{}", i32::MIN, i32::MAX).into(),
                        );
                    }
                    _ => todo!(),
                },
            }
        }
        None => {
            let specifier = Specifier::parse(str)?;

            Ok(Selector {
                specifier,
                offset: None,
            })
        }
    }
}

fn parse_range(str: &str) -> Result<Range, Error> {
    match str.split_once("..") {
        Some((start, end)) => {
            let start = if start.is_empty() {
                Selector {
                    specifier: Specifier::Latest,
                    offset: None,
                }
            } else {
                parse_selector(start)?
            };

            let end = if end.is_empty() {
                Selector {
                    specifier: Specifier::First,
                    offset: None,
                }
            } else {
                parse_selector(end)?
            };

            return Ok(Range { start, end });
        }
        None => Err(format!("invalid range\n{}", RANGE_HELP).into()),
    }
}

pub fn get() -> Result<Args, Error> {
    let repository_path;
    let range;

    let mut args = env::args_os();
    let _program_name = args.next();

    if let Some(first_arg) = args.next() {
        repository_path = match first_arg.to_str() {
            Some(arg) => arg.to_string(),
            None => return Err("first argument is not UTF-8".into()),
        };

        range = if let Some(second_arg) = args.next() {
            match second_arg.to_str() {
                Some(arg) => parse_range(arg),
                None => return Err("second argument is not UTF-8".into()),
            }?
        } else {
            warn("no second argument given. defaulting to range `today..`");
            Range {
                start: Selector {
                    specifier: Specifier::Today,
                    offset: None,
                },
                end: Selector {
                    specifier: Specifier::Latest,
                    offset: None,
                },
            }
        };

        Ok(Args {
            repository_path,
            range,
        })
    } else {
        Err(format!("no arguments given\n{}", HELP).into())
    }
}
