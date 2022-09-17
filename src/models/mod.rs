use chrono::{offset::Utc, Date};
use egui_datepicker::DatePicker;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// An enum to track the level of Drunkness (0 - 5)
pub enum User {
	Lostsaka,
	Gkasma,
}

impl Default for User {
	fn default() -> Self {
		Self::Lostsaka
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// An enum to track the level of Drunkness (0 - 5)
pub enum Drunkness {
	Cool,
	LittleHead,
	Bream,
	Gnat,
	Ant,
	ImOk,
}

impl Default for Drunkness {
	fn default() -> Self {
		Self::Cool
	}
}

/// A struct to track the result of the night
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Craziness {
	pub user: User,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	//pub date: Date<Utc>,
	pub location: String,
	pub night_description: String,
}

impl Default for Craziness {
	fn default() -> Self {
		Self {
			user: User::default(),
			drunkness: Drunkness::default(),
			coitus: false,
			drive: false,
			talked_2x: false,
			location: "Athens".to_string(),
			night_description: "Kala htan".to_string(),
		}
	}
}
