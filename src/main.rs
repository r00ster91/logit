mod args;

use args::Specifier;
use chrono::{Date, Datelike, Local, NaiveDateTime};
use git2::{Commit, Oid, Repository, Revwalk, Sort};
use regex::Regex;
use std::{
    fmt::{self, Write},
    process,
};

fn error(err: fmt::Arguments) -> ! {
    eprintln!("{}", err);
    process::exit(1);
}

fn get_revwalk<'repo>(repo: &'repo Repository) -> Revwalk<'repo> {
    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(err) => error(format_args!("failed to start commit traversion: {}", &err)),
    };

    revwalk.push_head().unwrap();

    revwalk
}

fn get_commit(id: Result<Oid, git2::Error>, repo: &Repository) -> Commit {
    let id = id.unwrap();
    repo.find_commit(id).unwrap()
}

fn walk_to_date<'repo>(
    repo: &'repo Repository,
    revwalk: &mut Revwalk,
    date: Date<Local>,
) -> Option<Commit<'repo>> {
    revwalk.set_sorting(Sort::TIME).unwrap();

    for commit_id in revwalk {
        let commit = get_commit(commit_id, repo);

        if date_match(date, &commit) {
            return Some(commit);
        }
    }

    None
}

fn get_commit_date(commit: &Commit) -> NaiveDateTime {
    // Incorporate the timezone offset.
    let secs = commit.time().seconds() + commit.time().offset_minutes() as i64 * 60;
    NaiveDateTime::from_timestamp(secs, 0)
}

fn date_match(date: Date<Local>, commit: &Commit) -> bool {
    let commit_date = get_commit_date(commit);

    commit_date.year() == date.year()
        && commit_date.month() == date.month()
        && commit_date.day() == date.day()
}

fn main() {
    let args = match args::get() {
        Ok(args) => args,
        Err(err) => error(format_args!("{}", err)),
    };

    let repo = match Repository::open(&args.repository_path) {
        Ok(repo) => repo,
        Err(err) => error(format_args!("failed to open repository: {}", &err)),
    };

    let mut revwalk = get_revwalk(&repo);

    let mut commits = Vec::<Commit>::new();

    let today = Local::today();

    match args.range.start.specifier {
        Specifier::Today => {
            // Keep walking until we find a commit made today.
            match walk_to_date(&repo, &mut revwalk, today) {
                Some(latest_commit_of_today) => commits.push(latest_commit_of_today),
                None => error(format_args!("no commits made today")),
            }
        }
        Specifier::Latest => {
            // Start at the latest commit.
            revwalk.set_sorting(Sort::TIME).unwrap();
        }
        Specifier::First => {
            // Start at the first commit ever made.
            revwalk.set_sorting(Sort::TIME | Sort::REVERSE).unwrap();
        }
        Specifier::Date(date) => {
            // Keep walking until we find a commit made on that date.
            match walk_to_date(&repo, &mut revwalk, date) {
                Some(latest_commit_of_that_date) => commits.push(latest_commit_of_that_date),
                None => error(format_args!(
                    "no commits made on {}",
                    date.format("%Y-%M-%D")
                )),
            }
        }
    }

    match args.range.end.specifier {
        Specifier::Today => {
            // Record all commits into a buffer until we find a commit made today and then append the buffer.
            let mut buffer = Vec::<Commit>::new();

            for commit_id in revwalk {
                let commit = get_commit(commit_id, &repo);

                if date_match(today, &commit) {
                    buffer.push(commit);
                    commits.append(&mut buffer);
                } else {
                    buffer.push(commit);
                }
            }
        }
        Specifier::Latest => {
            revwalk.set_sorting(Sort::TIME).unwrap();

            if let Some(commit_id) = revwalk.next() {
                let commit = get_commit(commit_id, &repo);
                commits.push(commit);
            }
        }
        Specifier::First => {
            for commit_id in revwalk {
                let commit = get_commit(commit_id, &repo);
                commits.push(commit);
            }
        }
        Specifier::Date(date) => {
            // Record all commits into a buffer until we find a commit made on that date and then append the buffer.
            let mut buffer = Vec::<Commit>::new();

            for commit_id in revwalk {
                let commit = get_commit(commit_id, &repo);

                if date_match(date, &commit) {
                    buffer.push(commit);
                    commits.append(&mut buffer);
                } else {
                    buffer.push(commit);
                }
            }
        }
    }

    let output = match write_changelog(&commits) {
        Ok(output) => output,
        Err(err) => error(format_args!("{}", err.0)),
    };

    println!("{}", output);
}

use std::borrow::Cow;
struct Error(Cow<'static, str>);

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Error("writing to stdout failed".into())
    }
}

fn capitalize(str: &str) -> String {
    let mut chars = str.chars();

    chars.next().unwrap().to_uppercase().chain(chars).collect()
}

fn write_changelog(commits: &[Commit]) -> Result<String, Error> {
    let version_regex = Regex::new(r"^v?([0-9]+\.[0-9]+\.[0-9]+)$").unwrap();
    let mut output = String::from("# Changelog\n");
    let mut current_date_opt: Option<NaiveDateTime> = None;

    for commit in commits {
        let commit_date = get_commit_date(&commit);

        if let Some(current_date) = current_date_opt {
            if commit_date.year() != current_date.year()
                || commit_date.month() != current_date.month()
                || commit_date.day() != current_date.day()
            {
                current_date_opt = Some(commit_date);
            }
        } else {
            current_date_opt = Some(commit_date);
        }

        if let Some(summary) = commit.summary() {
            if let Some(caps) = version_regex.captures(summary) {
                let version = caps.get(1).unwrap().as_str();
                write!(
                    output,
                    "\n\n## {} ({})\n",
                    version,
                    current_date_opt.unwrap().format("%Y-%m-%d")
                )?;

                continue;
            }

            write!(output, "\n* {}", &capitalize(summary).trim_end_matches('.'))?;

            if let Some(message) = commit.message() {
                let description = message.strip_prefix(summary).unwrap().trim();
                if !description.is_empty() {
                    write!(output, "\n\n  {}", description)?;
                }
            } else {
                return Err(Error("commit message is not UTF-8".into()));
            }
        } else {
            return Err(Error("commit summary is not UTF-8".into()));
        }
    }

    Ok(output)
}
