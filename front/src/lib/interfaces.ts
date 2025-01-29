interface WeatherForecast {
    temperature_2m: number;
    precipitation: number;
    wind_speed_10m: number;
    precipitation_probability: number;
    weather_code: number;
}

interface Prediction {
    id: number;
    free_stands: number;
    available_bikes: number;
}

interface Table {
    id: number;
    latitude: number;
    longitude: number;
}

interface Details {
    id: number;
    name: string;
    latitude: number;
    longitude: number;
    address: string;
    area: string;
    capacity: number;
}