pub fn format(result: &str) -> String {
    let temperature = result.split('=').collect::<Vec<&str>>()[1];
    let temperature = temperature.split('\'').collect::<Vec<&str>>()[0];

    let celcius = temperature.parse::<f32>().unwrap();

    let farenheit = (celcius * 9.0 / 5.0) + 32.0;

    format!("{} °C / {} °F", celcius, farenheit)
}
