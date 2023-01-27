use std::sync::{Mutex, Arc};
use std::collections::HashMap;

use wasm_bindgen::{prelude::*, JsCast};
use lazy_static::lazy_static;
use rand::{ Rng, thread_rng };
use int_enum::IntEnum;
use web_sys::{ HtmlDivElement, HtmlParagraphElement };

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

#[repr(u8)]
#[derive(Eq, PartialEq, Hash, Clone, Copy, IntEnum, Debug)]
pub enum Signs {
	Rock = 0,
	Paper = 1,
	Scissors = 2
}

lazy_static!(
	static ref OPPONENT: Arc<Mutex<Signs>> = Arc::new(Mutex::new(Signs::Rock));

	static ref WIN_SIGNS: HashMap<Signs, Signs> = HashMap::from([
		(Signs::Rock, Signs::Scissors),
		(Signs::Scissors, Signs::Paper),
		(Signs::Paper, Signs::Rock)
	]);

	static ref WINS: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
	static ref LOSSES: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

	// static ref DUEL_RESULT: Arc<Mutex<Option<HtmlParagraphElement>>> = Arc::new(Mutex::new(None));
);

pub fn get_random_sign() -> Signs {
	let mut rng = thread_rng();
	Signs::from_int(rng.gen_range(0..3)).unwrap()
}

#[wasm_bindgen]
pub fn randomise_sign() {
	*OPPONENT.lock().unwrap() = get_random_sign(); // Signs::from_int(rng.gen_range(0..3)).unwrap();
}

#[wasm_bindgen]
pub fn check_sign(player: u8) -> i8 {
	let player = Signs::from_int(player).unwrap();
	let opponent = *OPPONENT.lock().unwrap(); // Signs::from_int(opponent).unwrap();

	if player == opponent { 0 }
	else {
		if WIN_SIGNS.get(&player).unwrap() == &opponent {
			1
		} else { -1 }
	}
}

#[wasm_bindgen]
pub fn inc_win() {
	*WINS.lock().unwrap() += 1;
}

#[wasm_bindgen]
pub fn inc_loss() {
	*LOSSES.lock().unwrap() += 1;
}

#[wasm_bindgen]
pub fn get_wins() -> u32 {
	*WINS.lock().unwrap()
}

#[wasm_bindgen]
pub fn get_losses() -> u32 {
	*LOSSES.lock().unwrap()
}

fn get_emoji(sign: Signs) -> char {
	match sign {
		Signs::Rock => '✊',
		Signs::Paper => '🖐',
		Signs::Scissors => '✌'
	}
}

#[wasm_bindgen]
pub fn update_player_sign(sign: u8) {
	let emoji = get_emoji(Signs::from_int(sign).unwrap());

	let document = web_sys::window().unwrap().document().unwrap();

	if let Some(ele) = document.get_element_by_id("player_sign") {
		ele.dyn_into::<HtmlDivElement>().unwrap()
			.set_text_content(Some(format!("{}", emoji).as_str()));
	}
}

#[wasm_bindgen]
pub fn update_ferris_sign() {
	let emoji = get_emoji(*OPPONENT.lock().unwrap());

	let document = web_sys::window().unwrap().document().unwrap();

	if let Some(ele) = document.get_element_by_id("ferris_sign") {
		ele.dyn_into::<HtmlDivElement>().unwrap()
			.set_text_content(Some(format!("{}", emoji).as_str())); // std::str::from_utf8(&[emoji]).unwrap()));
	}
}

#[wasm_bindgen]
pub fn update_duel_result(win_val: i8) {
	let document = web_sys::window().unwrap().document().unwrap();

	if let Some(ele) = document.get_element_by_id("duel_result") {
		ele.dyn_into::<HtmlParagraphElement>().unwrap()
			.set_text_content(Some(
				format!("{}",
					match win_val {
						1 => "You win!",
						0 => "That was a tie!",
						-1 => "You lose!",

						_ => "What ??"
					}).as_str()));
	}
}

#[wasm_bindgen]
pub fn update_wl_counter() {
	let document = web_sys::window().unwrap().document().unwrap();

	if let Some(ele) = document.get_element_by_id("win_lose_counter") {
		ele.dyn_into::<HtmlParagraphElement>().unwrap()
			.set_text_content(Some(
				format!("Wins: {} | Losses: {}",
					*WINS.lock().unwrap(),
					*LOSSES.lock().unwrap()).as_str()));
	}
}

// pub fn new_game() {
// 	let document = web_sys::window().unwrap()
// 		.document().unwrap();
// 	let body = document.body().unwrap();

// 	// set_random_sign();
// }

#[wasm_bindgen(start)]
pub fn main() {
	log("Hello from Rust!");
}
