let url = 'http://localhost:8000/';
let url_mock = 'http://localhost:8000/mock/';

export async function getWeatherForecast(): Promise<Map<string, WeatherForecast>> {
    const response = await fetch(url + 'weather_forecast');
    return await response.json();
}

export async function getTables(): Promise<Table[]> {
    const response = await fetch(url + 'stations');
    return await response.json();
}

export async function getDetails(): Promise<Details[]> {
    const response = await fetch(url + 'detailed_stations/');
    return await response.json();
}

export async function getDetailsById(id: number): Promise<Details> {
    const response = await fetch(url + 'station/' + id)
    return await response.json();
}