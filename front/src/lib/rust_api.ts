import type {CustomMarkers} from "$lib/CustomMarkers";
import {date, markers} from "$lib/store";

const url = 'http://localhost:8000/';

export async function getWeatherForecast(): Promise<Map<string, WeatherForecast> | null> {
    console.log('Fetching weather forecast');
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

export async function getTables(): Promise<Table[]> {
    const response = await fetch(url + 'stations');
    return await response.json();
}

export async function getDetails(): Promise<Map<number,Details>> {
    const response = await fetch(url + 'detailed_stations');
    const entries: [number, Details][] = Object.entries(await response.json()).map(([key, value]) => [Number(key), value as Details]);
    return new Map<number, Details>(entries);
}

export async function getDetailsById(id: number): Promise<Details> {
    const response = await fetch(url + 'station/' + id)
    return await response.json();
}

export async function getAllPredictions(date: string): Promise<Map<number, Prediction> | null> {
    date = date.replaceAll(':', '%3A');
    date = date.replaceAll('Z', '');
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

export async function getPredict(id: number, date: string): Promise<Prediction | null> {
    date = date.replaceAll(':', '%3A');
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

export async function setMarkerColor(): Promise<CustomMarkers[]> {
    let id: number = 0;
    let date_id: string = '';
    date.subscribe(value => date_id = value)();
    let date_value = new Date(
        new Date(date_id).setHours(
            Number(date_id.split('T')[1].split(':')[0]) + 1, 0, 0, 0)
    ).toISOString();
    date_value = date_value.replaceAll(".000Z", "Z");

    let color: string = 'black';
    let markersList: CustomMarkers[] = [];
    markers.subscribe(value => markersList = value)();

    const predictions = await getAllPredictions(date_value);
    const details = await getDetails();
    if (predictions === null) {
        for (const marker of markersList) {
            marker.changeColor('black');
        }
        return markersList;
    }

    for (const marker of markersList) {
        if (date_value >= new Date().toISOString()) {
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