
use AOCResult;

use std::cmp::Ordering;
use std::str::FromStr;


pub fn identity<T>(t: T) -> T { t }

#[derive(Debug, Clone, Copy)]
pub enum EventType {
	Sleep,
	Wake,
	Change(u32),
}

impl FromStr for EventType {
	type Err = failure::Error;

	fn from_str(s: &str) -> AOCResult<Self> {
		match s {
			"falls asleep" => Ok(EventType::Sleep),
			"wakes up" => Ok(EventType::Wake),
			guard_change => {
				let guard_id = guard_change
					.trim_matches(|c: char| !c.is_numeric())
					.parse()?;

				Ok(EventType::Change(guard_id))
			}
		}
	}
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Timepoint{ pub month: u32, pub day: u32, pub minute: u32 }

pub const NULL_TP: Timepoint = Timepoint{ month: 0, day: 0, minute: 0 };


impl PartialOrd for Timepoint {
    fn partial_cmp(&self, other: &Timepoint) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for Timepoint {
	fn cmp(&self, o: &Timepoint) -> Ordering {
		if self.month != o.month {
			return self.month.cmp(&o.month);
		}

		if self.day != o.day {
			return self.day.cmp(&o.day);
		}

		self.minute.cmp(&o.minute)
	}
}


#[derive(Debug, Clone)]
pub struct Event {
	pub time: Timepoint,
	pub ty: EventType,
}

impl Event {
	pub fn from_captures<'t>(caps: regex::Captures<'t>) -> AOCResult<Event> {
		let ty = caps[5].parse()?;

		let hour: u32 = caps[3].parse()?;
		let minute: u32 = caps[4].parse()?;

		let time = Timepoint {
			month: caps[1].parse()?,
			day: caps[2].parse()?,
			minute: minute + hour * 60
		};
		Ok(Event { time, ty })
	}
}


#[derive(Debug, Copy, Clone)]
pub struct SleepyTime {
	pub id: u32,
	pub start: Timepoint,
	pub duration: u32,
}

impl SleepyTime {
	// Yields each minute spent asleep
	pub fn iter_minutes(&self) -> impl Iterator<Item=u32> {
		const HOURS_IN_DAY: u32 = 60 * 24;

		assert!(self.duration < HOURS_IN_DAY);

		let start = self.start.minute;
		let end = self.start.minute + self.duration;

		(start..end).map(|m| m % HOURS_IN_DAY)
	}
}


pub fn derive_sleepy_times_from_events(events: Vec<Event>) -> Vec<SleepyTime> {
	// Map Sleep/Wake pairs to SleepyTimes
	events.into_iter()
		.scan((0, NULL_TP), |(guard_id, start), e| {
			use EventType::*;

			match e.ty {
				Change(id) => {
					*guard_id = id;
					Some(None)
				}

				Sleep => {
					*start = e.time;
					Some(None)
				}

				Wake => {
					assert!(e.time.month == start.month);

					let diff_day = e.time.day - start.day;
					let diff_minute = e.time.minute - start.minute;

					let st = SleepyTime{
						id: *guard_id,
						start: *start,
						duration: diff_minute + diff_day*60*24
					};

					Some(Some(st))
				}
			}
		})
		.filter_map(identity)
		.collect()
}