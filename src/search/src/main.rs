extern crate http;
extern crate url;

use std::env;
use std::path::Path;
use std::fs;
use url::Url;
use std::io::Error;
use regex::Regex;
use dict::*;
use json;
use json::JsonValue;

#[derive (PartialEq, Copy, Clone)]
enum EmojiType {
	GIF,
	NonGIF,
	Any
}

fn main() {
	println!("Content-Type: application/json\n");
	
	let emoji_root = Path::new("../emojis");
	
	match env::set_current_dir(&emoji_root) {
		Err(e) => {println!("Error: {:?}", e); return},
		_ => {}
	}

	let my_url: String;

	match env::var("REQUEST_URI") {
		Ok(s) => my_url = s,
		Err(e) => {println!("Error: {:?}", e); return},
	}
	
	let query = get_query_from_url(&my_url, "query");
	if query == ""
	{
		println!("{:?}", "{}");
	}

	let gif = get_query_from_url(&my_url, "gif");
	let emoji_type = str_to_emoji_type(&gif);

	let emojis = get_emoji_dict(query.as_str(), emoji_root, emoji_type);
	let mut json_str = json::JsonValue::new_object();

	for show in emojis.unwrap() {
		let mut json_emojis = vec![];
		for emoji in &show.val {
			json_emojis.push(JsonValue::String(emoji.clone()));
		}
		if show.val.len() > 0 {
			json_str[show.key.clone()] = JsonValue::Array(json_emojis);
		}
	}
	
	println!("{}", json_str.dump());
}

fn get_query_from_url(url_str: &str, key: &str) -> String{
	let mut url_obj: Url = Url::parse("http://dummy").unwrap();
	
	match url_obj.join(url_str) {
		Ok(u) => url_obj = u,
		Err(e) => {
			println!("{:?}",e); 
			return String::new()} 
	}

	match url_obj.query_pairs().find(|x| {x.0 == key}) {
		Some(s) => s.1.to_string(),
		None => {
			String::new()}
	}
}

fn get_emoji_dict(search: &str, path: &Path, emoji_type: EmojiType) -> Result<Dict<Vec<String>>, Error> {
	let dir_listing = fs::read_dir(&path)?;
	let mut dict = Dict::<Vec<String>>::new();

	for item in dir_listing {
		let item = item.unwrap();
		let item_path_buf = item.path();
		
		if !item_path_buf.is_dir() {
			continue;
		}

		let folder_name = item_path_buf.as_path().file_name().unwrap().
							to_str().unwrap();
		let emojis_in_show = get_emojis_in_show(search, item_path_buf.as_path(), emoji_type)?;
		
		dict.add(folder_name.to_string(), emojis_in_show);
	}
	Ok(dict)
}

fn get_emojis_in_show(search: &str, path: &Path, emoji_type: EmojiType) -> Result<Vec<String>, Error> {
	let dir_listing = fs::read_dir(&path)?;
	let mut emojis: Vec<String> = vec![];
	let folder_name = path.file_name().unwrap().to_str().unwrap();

	if emoji_type != EmojiType::GIF
	{
		for item in dir_listing {
			check_dir_item(search, folder_name, &item.unwrap(), &mut emojis);
		}
	}
	
	if emoji_type == EmojiType::NonGIF
	{
		return Ok(emojis);
	}

	let gif_listing = fs::read_dir(path.join("gifs"));
	
	match gif_listing{
		Ok(gif_listing) => {
			for item in gif_listing {
				check_dir_item(search, folder_name, &item.unwrap(), &mut emojis);
			}
		}
		_ => {}
	}
	
	Ok(emojis)
}

fn check_dir_item(search: &str, folder_name: &str, 
					item: &fs::DirEntry, emojis: &mut Vec<String>) {
		let item_path_buf = item.path();
		
		if !item_path_buf.is_file() {
			return
		}
		
		let file_name_os = item.file_name();
		let file_name = file_name_os.to_str().unwrap();
		
		if does_emoji_match(search, folder_name, file_name)
		{
			emojis.push(file_name.to_string());
		}
}

fn does_emoji_match(search: &str, folder: &str, file: &str) -> bool {
	let lcase_folder = folder.to_string().to_lowercase();
	let lcase_file = file.to_string().to_lowercase();
	let lcase_search = search.to_string().to_lowercase();
	let symbol_filter = Regex::new(r"[_+\- ]").unwrap();
	
	let mut folder_words = symbol_filter.split(lcase_folder.as_str());
	let mut file_words = symbol_filter.split(lcase_file.as_str());
	let mut search_words = symbol_filter.split(lcase_search.as_str());
	
	search_words.all(|x: &str|
		folder_words.any(|y: &str| x == y) || file_words.any(|y: &str| x == y)) 
}

fn str_to_emoji_type(str_to_convert: &str) -> EmojiType {
	match str_to_convert {
		"yes" => EmojiType::GIF,
		"no" => EmojiType::NonGIF,
		_ => EmojiType::Any
	}
}
