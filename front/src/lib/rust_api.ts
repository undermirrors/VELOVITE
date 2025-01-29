import type {CustomMarkers} from "$lib/CustomMarkers";
import {date, markers} from "$lib/store";

let url = 'http://localhost:8000/';
let url_mock = 'http://localhost:8000/mock/';

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

export async function getDetails(): Promise<Details[]> {
    const response = await fetch(url + 'detailed_stations/');
    return await response.json();
}

export async function getDetailsById(id: number): Promise<Details> {
    const response = await fetch(url + 'station/' + id)
    return await response.json();
}

export async function getAllPredictions(date: string): Promise<Prediction[] | null> {
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
    return await response.json();
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

    let predictions = await getAllPredictions(date_value);
    if (predictions === null) {
        for (const marker of markersList) {
            marker.changeColor('black');
        }
        return markersList;
    }

    for (const marker of markersList) {
        if (date_value >= new Date().toISOString()) {
            id = marker.getId();
            let val = await getRatio(id, predictions);
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

async function getRatio(id: number, predictions: Prediction[]): Promise<number> {
    let data1 = predictions.find(element => element.id === id);
    if (data1 === undefined) {
        return -1;
    }
    let data2 = await getDetailsById(id);
    if (data2.capacity === 0) {
        return 0;
    }
    return data1.available_bikes / data2.capacity;
}