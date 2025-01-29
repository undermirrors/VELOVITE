/**
 * Interfaces for the data fetched from the API getWeatherForecast
 *
 * @interface WeatherForecast
 */
interface WeatherForecast {
    temperature_2m: number;
    precipitation: number;
    wind_speed_10m: number;
    precipitation_probability: number;
    weather_code: number;
}

/**
 * Interfaces for the data fetched from the API getPredict and getAllPredictions
 *
 * @interface Station
 */
interface Prediction {
    id: number;
    free_stands: number;
    available_bikes: number;
}

/**
 * Interfaces for the data fetched from the API getStations and getMarkersFromSearch
 *
 * @interface Station
 */
interface Station {
    id: number;
    latitude: number;
    longitude: number;
}

/**
 * Interfaces for the data fetched from the API getDetails and getDetailsById
 *
 * @interface Details
 */
interface Details {
    id: number;
    name: string;
    latitude: number;
    longitude: number;
    address: string;
    area: string;
    capacity: number;
}