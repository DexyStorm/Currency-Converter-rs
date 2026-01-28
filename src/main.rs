use std::{env, io};
use serde_json;

#[allow(nonstandard_style)]
#[allow(unused_parens)]
fn main()
{
	println!();
	
	// Load environment variables from .env file.
	// Fails if .env file not found, not readable or invalid.
	dotenvy::dotenv().expect("Could not load .env file");
	let baseUrl: &str = "https://api.freecurrencyapi.com/";
	let baseUrlAppend: &str = "v1/latest?apikey=";
	
	//loads the "API_KEY" environment variable
	let API_KEY = env::var("API_KEY").expect("Could not find API_KEY env var");

	//send API request
	let response = reqwest::blocking::get(format!("{baseUrl}{baseUrlAppend}{API_KEY}")).unwrap().text().unwrap();
	
	//convert response from server into serde_json so i can work with it more easily
	let json : serde_json::Value = serde_json::from_str(&response).unwrap();
	
	
	println!("Available Currencies:");
	if let Some(data) = json["data"].as_object()
	{
		for (eachCurrency, _value) in data
		{
			print!("{eachCurrency} ");
		}
	}
	println!();
	
	println!("What currency do you want to convert?");
	let mut rateOfSelectedCurrency: f64 = 0.0;
	let mut selectedCurrency: String = String::from("");
	io::stdin().read_line(&mut selectedCurrency).expect("Failed to read line");
	selectedCurrency = String::from(selectedCurrency.trim().to_uppercase());
	
	let mut foundSelectedCurrency: bool = false;
	if let Some(data) = json["data"].as_object()
	{
		for (eachCurrency, value) in data
		{
			if(selectedCurrency == String::from(eachCurrency))
			{
				foundSelectedCurrency = true;
				if let Some(eachRate) = value.as_f64()
				{
					rateOfSelectedCurrency = eachRate;
				}
				
			}
		}
	}
	if(foundSelectedCurrency == false)
	{
		panic!("Could not find currency");
	}
	
	println!("How much of that currency do you want to convert?");
	let mut amountToConvert: String = String::from("");
	io::stdin().read_line(&mut amountToConvert).expect("Failed to read line");
	amountToConvert = String::from(amountToConvert.trim());
	let amountToConvert: f64 = amountToConvert.parse::<f64>().unwrap();
	
	println!();
	if let Some(data) = json["data"].as_object()
	{
		for (eachCurrency, value) in data
		{
			if(String::from(eachCurrency) != selectedCurrency)
			{
				if let Some(eachRate) = value.as_f64()
				{
					let convertedAmount: f64 = amountToConvert * (eachRate/rateOfSelectedCurrency);
					
					println!("{}: {}", eachCurrency, convertedAmount);
				}
				
			}
		}
	}
	
	//just here cuz the output is weird on zsh
	println!();
 
}
