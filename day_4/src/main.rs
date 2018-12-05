#![feature(extern_prelude)]

extern crate regex;
extern crate failure;

mod common;

pub use failure::Error;
pub type AOCResult<T> = Result<T, Error>;

use common::*;

use std::iter;
use regex::Regex;


fn main() -> AOCResult<()> {
	let input = include_str!("../input.txt");
	let re = Regex::new(r"\[\d+-(\d+)-(\d+) (\d+):(\d+)\] (.*)")?;

	let mut events: Vec<_> = input.lines()
		.map(|l| re.captures(l).unwrap())
		.map(Event::from_captures)
		.map(Result::unwrap)
		.collect();

	events.sort_unstable_by_key(|e| e.time);

	let mut sleepy_times = derive_sleepy_times_from_events(events);
	sleepy_times.sort_unstable_by_key(|st| st.id);

	solution_1(&sleepy_times);

	Ok(())
}


fn solution_1(sleepy_times: &[SleepyTime]) {
	let empty_st = SleepyTime{id: 0, start: NULL_TP, duration: 0};

	let sleepiest_guard = sleepy_times.iter().chain(iter::once(&empty_st))
		.scan((0, 0), |(id, duration), st| {
			if *id != st.id {
				*id = st.id;
				*duration = 0;
				Some(None)
			} else {
				*duration += st.duration;
				Some(Some((*id, *duration)))
			}
		})
		.filter_map(identity)
		.max_by_key(|(_, d)| *d)
		.unwrap().0;

	println!("sleepiest guard #{}", sleepiest_guard);

	// Get all minutes spent asleep
	let mut sleepiest_guard_minutes: Vec<_> = sleepy_times.iter()
		.filter(|st| st.id == sleepiest_guard)
		.flat_map(|st| st.iter_minutes())
		.collect();

	sleepiest_guard_minutes.sort_unstable();

	// Find mode
	let most_common_sleepy_minute = sleepiest_guard_minutes.iter()
		.cloned()
		.chain(iter::once(10000)) // terminator
		.scan((0, 0), |(prev, count), min| {
			if *prev != min {
				*prev = min;
				*count = 0;
				Some((min, 0))
			} else {
				*count += 1;
				Some((min, *count))
			}
		})
		.max_by_key(|(_, cnt)| *cnt)
		.map(|(min, _)| min)
		.unwrap() % 60;

	println!("#{} x {} = {}", sleepiest_guard, most_common_sleepy_minute,
		sleepiest_guard * most_common_sleepy_minute);
}