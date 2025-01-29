import type {CustomMarkers} from "$lib/CustomMarkers";
import {date, markers} from "$lib/store";

const url = 'http://localhost:8000/';

/**
 * Get the weather forecast for each station from the API at http://localhost:8000/weather_forecast
 *
 * @returns a map of the weather forecast for each station if found, null otherwise
 */
export async function getWeatherForecast(): Promise<Map<string, WeatherForecast> | null> {
    try {
        const response = await fetch(url + 'weather_forecast');
        if (!response.ok) {
            console.log('Error fetching weather forecast:', response.statusText);
            return null;
        }
        return new Map(Object.entries(await response.json()));
    } catch (error) {
        console.error('Error fetching weather forecast:', error);
        return null;
    }
}

/**
 * Get the markers from the search query from the API at http://localhost:8000/search/{search}
 *
 * @param search
 * @returns a list of stations if found, null otherwise
 */
export async function getMarkersFromSearch(search: string): Promise<Station[]> {
    const response = await fetch(url + 'search/' + search);
    return await response.json();
}

/**
 * Get the stations from the API at http://localhost:8000/stations
 *
 * @returns a list of stations if found, null otherwise
 */
export async function getStations(): Promise<Station[]> {
    const response = await fetch(url + 'stations');
    return await response.json();
}

/**
 * Get the details for each station from the API at http://localhost:8000/detailed_stations
 *
 * @returns a map of the details for each station if found, null otherwise
 */
export async function getDetails(): Promise<Map<number, Details>> {
    const response = await fetch(url + 'detailed_stations');
    const entries: [number, Details][] = Object.entries(await response.json()).map(([key, value]) => [Number(key), value as Details]);
    return new Map<number, Details>(entries);
}

/**
 * Get the details for a specific station from the API at http://localhost:8000/station/{id}
 *
 * @param id
 * @returns the details for the station if found, null otherwise
 */
export async function getDetailsById(id: number): Promise<Details> {
    const response = await fetch(url + 'station/' + id)
    return await response.json();
}

/**
 * Get the predictions for each station from the API at http://localhost:8000/predictions?date={date}
 *
 * @param date
 * @returns a map of the predictions for each station if found, null otherwise
 */
export async function getAllPredictions(date: string): Promise<Map<number, Prediction> | null> {
    console.log(date);
    date = date.replaceAll(':', '%3A');

    let response;
    try {
        response = await fetch(url + 'predictions?date=' + date);
        if (!response.ok) {
            console.log('Error fetching prediction : ', response.statusText);
            return null;
        }
    } catch (error) {
        console.error('Error fetching prediction:', error);
        return null;
    }
    const entries: [number, Prediction][] = Object.entries(await response.json()).map(([key, value]) => [Number(key), value as Prediction]);
    return new Map<number, Prediction>(entries);
}

/**
 * Get the prediction for a specific station from the API at http://localhost:8000/predict?id={id}&date={date}
 *
 * @param id
 * @param date
 * @returns the prediction for the station if found, null otherwise
 */
export async function getPredict(id: number, date: string): Promise<Prediction | null> {
    date = date.replaceAll(':', '%3A');
    // We don't need the Z at the end of the date
    date = date.replaceAll('Z', '');

    let response;
    try {
        response = await fetch(url + 'predict?id=' + id + '&date=' + date);
        if (!response.ok) {
            console.log('Error fetching prediction : ', response.statusText);
            return null;
        }
    } catch (error) {
        console.error('Error fetching prediction:', error);
        return null;
    }
    return await response.json();
}

/**
 * Get the ratio for each station with previous data
 *
 * @returns a map of the ratio for each station if found, null otherwise
 */
export async function setMarkerColor(): Promise<CustomMarkers[]> {
    let id: number = 0;
    let date_value: Date = new Date(0, 1, 1); // to avoid confusion with the date type
    date.subscribe(value => date_value = value)();

    // to set the minutes and seconds to 0
    date_value.setMinutes(0);
    date_value.setSeconds(0);

    // to format the date in the correct format
    const dateStr = date_value.toLocaleString('sv-SE', { year: 'numeric', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', second: '2-digit' }).replace(' ', 'T');

    let color: string = 'black';
    let markersList: CustomMarkers[] = [];
    markers.subscribe(value => markersList = value)();

    const predictions = await getAllPredictions(dateStr);
    const details = await getDetails();
    if (predictions === null) {
        for (const marker of markersList) {
            marker.changeColor('black');
        }
        return markersList;
    }

    for (const marker of markersList) {
        let basicDate = new Date();
        basicDate.setHours(0, 0, 0, 0);
        if (date_value >= basicDate) {
            id = marker.getId();
            const val = await getRatio(id, predictions, details);
            if (val === -1) {
                color = 'black';
            } else {
                const r = Math.round(255 * (1 - val));
                const g = Math.round(255 * val);
                color = `rgb(${r}, ${g}, 0)`;
            }
        } else {
            color = 'black';

        }
        marker.changeColor(color);
    }
    return markersList;
}

/**
 * Get the ratio for a specific station with previous data
 *
 * @param id
 * @param predictions
 * @param details
 * @returns the ratio between 0 and 1 for the station if found, -1 otherwise
 */
async function getRatio(id: number, predictions: Map<number, Prediction>, details: Map<number, Details>): Promise<number> {
    const data = predictions.get(id);
    if (data === undefined) {
        return -1;
    }
    const data2 = details.get(id);
    if (data2 === undefined) {
        return -1;
    }
    if (data2.capacity === 0) {
        return 0;
    }
    return data.available_bikes / data2.capacity;
}