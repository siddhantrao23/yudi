use std::string;

use serde::Deserialize;

use reqwest;

pub struct Weather {
  condition: String,
  icon: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct CurrentWeather {
  temperature: f64,
  windspeed: f64,
  winddirection: i64,
  weathercode: u8,
  is_day: i8,
  time: String,
}

#[derive(Deserialize, Debug)]
struct Units {
  time: String,
  weathercode: String,
  temperature_2m_max: String,
  temperature_2m_min: String,
} 


#[derive(Deserialize, Debug)]
struct ForecastData {
  time: Vec<String>,
  weathercode: Vec<u8>,
  temperature_2m_max: Vec<f64>,
  temperature_2m_min: Vec<f64>,
}

// TODO: switch to daily readings instead
#[derive(Deserialize, Debug)]
#[allow(unused)]
struct WeatherData {
  latitude: f64,
  longitude: f64,
  generationtime_ms: f64,
  utc_offset_seconds: f64,
  timezone: String,
  timezone_abbreviation: String,
  elevation: f64,
  current_weather: CurrentWeather,
  daily_units: Units,
  daily: ForecastData,
}

pub fn convert_code(code: u8) -> Weather {
  match code {
    0 => return Weather {
      condition: "clear skies".to_string(),
      icon: "☀".to_string(),
    },

    1 | 2 | 3 => return Weather {
      condition: "partly cloudy".to_string(),
      icon: "🌤".to_string(),
    },

    45 | 48 => return Weather {
      condition: "fog".to_string(),
      icon: "☁".to_string(),
    },

    51 | 53 | 55 => return Weather {
      condition: "drizzle".to_string(),
      icon: "🌧".to_string(),
    },

    56 | 57 => return Weather {
      condition: "freezing drizzle".to_string(),
      icon: "❆".to_string(),
    },

    61 | 63 | 65 => return Weather {
      condition: "rain".to_string(),
      icon: "🌧".to_string(),
    },

    66 | 67 => return Weather {
      condition: "freezing rain".to_string(),
      icon: "❆".to_string(),
    },

    71 | 73 | 75 | 77 => return Weather {
      condition: "snowfall".to_string(),
      icon: "❆".to_string(),
    },

    80 | 81 | 82 => return Weather {
      condition: "rain showers".to_string(),
      icon: "🌧".to_string(),
    },
    
    85 | 86 => return Weather {
      condition: "snow showers".to_string(),
      icon: "❆".to_string(),
    },

    95 | 96 | 99 => return Weather {
      condition: "thunderstorm".to_string(),
      icon: "🌩".to_string(),
    },

    _ => return Weather {
      condition: "undefined".to_string(),
      icon: "x".to_string(),
    }
  }
}

pub async fn fetch_weather() -> Result<Vec<(String, usize)>, reqwest::Error> {
  // TODO: get user coords
  let lat = 12.97;
  let long = 77.59;

  let url = format!(
    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=weathercode,temperature_2m_max,temperature_2m_min&current_weather=true&timezone=auto&past_days=7&forecast_days=1",
    lat,
    long,
  );

  let weather_data = reqwest::get(url)
    .await?
    .text()
    .await?;

  // println!("{:?}", weather_data);
  let val: WeatherData = serde_json::from_str(&weather_data).unwrap();

  let mut res: Vec<(String, usize)> = vec![];
  for i in 0..val.daily.time.len() {
    let code = convert_code(val.daily.weathercode[i]);
    res.push((
      format!("{}\t\t{}\t{}\t{}{}-{}{}", val.daily.time[i],
        code.icon, code.condition, 
        val.daily.temperature_2m_max[i], val.daily_units.temperature_2m_max, 
        val.daily.temperature_2m_min[i], val.daily_units.temperature_2m_min)
    , i));
  }

  Ok(res)
}