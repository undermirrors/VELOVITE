<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import { onMount } from 'svelte';
	import {
		getMarkersFromSearch,
		getStations,
		getWeatherForecast,
		setMarkerColor
	} from '$lib/rust_api';
	import { date, mapContainerStored, markers, research } from '$lib/store';
	import type { CustomMarkers } from '$lib/CustomMarkers';
	import type L from 'leaflet';

	let mapContainer: HTMLElement | null = null;
	let map: L.Map | null = null;

	/**
	 * Update date and time selected by user,
	 * and reload the map in order to update the color of the markers
	 *
	 * @param selectedDate
	 */
	async function updateDate(selectedDate: Date) {
		date.set(selectedDate);
		await updateMapMarkers();
	}

	/**
	 * Update the date of the selected day only, and keep the time,
	 * and reload the map in order to update the color of the markers
	 *
	 * @param selectedJour : date chose by the user
	 */
	async function updateJour(selectedJour: Date) {
		// get the previous selected date
		let newDate: Date = new Date(selectedJour);
		date.subscribe((value) => (newDate = value))();

		// modify the year, month and day of the new date
		newDate.setFullYear(
			selectedJour.getFullYear(),
			selectedJour.getMonth(),
			selectedJour.getDate()
		);
		date.set(newDate);

		await updateMapMarkers();
	}

	/**
	 * Update the hour of the selected day only, and keep the date,
	 * and reload the map in order to update the color of the markers
	 *
	 * @param selectedHour : hour chose by the user
	 */
	async function updateHour(selectedHour: number) {
		// get the previous selected date
		let newDate: Date = new Date(selectedHour);
		date.subscribe((value) => (newDate = value))();

		// modify the hour of the new date
		newDate.setHours(selectedHour);
		date.set(newDate);

		// reload the map
		await updateMapMarkers();
	}

	/**
	 * Update the research entered by the user,
	 * and reload the map in order to update number of markers
	 *
	 * @param value : address or name of the station entered by the user
	 */
	async function updateEntries(value: string) {
		research.set(value);
		await updateMapMarkers();
	}

	/**
	 * Get the date stored
	 *
	 * @returns {Date} : the date selected by the user
	 */
	function getJour() {
		let dateJour: Date = new Date(0, 1, 1); // 01/01/0000 to avoid confusion in the code if the date is not set
		date.subscribe((value) => (dateJour = value))();

		return dateJour;
	}

	updateDate(new Date(new Date().getTime())); // in order to predict directly Velo'V availability
	let dateJour = getJour();

	// Get the current time in the format HH:MM
	// We use the ISO format to get the time in the same format as the API
	let hoursAndMinutes = `${dateJour.toISOString().split('T')[1].split(':')[0]}:${dateJour.toISOString().split(':')[1].split('.')[0]}`;

	// Get the current time in the format HH:MM in local time
	let localHoursAndMinutes = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

	// Initialize the variables used for weather icon and temperature
	let temp = '';
	let weather = -1;
	let src_img = '';

	/**
	 * Update the weather icon and temperature displayed on the page
	 * The weather icon is chosen according to the weather code returned by the API
	 */
	async function updateWeather() {
		let global_weather = await getWeatherForecast();
		if (global_weather === null) {
			temp = '?';
			weather = -1;
		} else {
			let date2 = new Date();
			date.subscribe((value) => (date2 = value))();
			let dateStr = `${date2.getFullYear()}-${String(date2.getMonth() + 1).padStart(2, '0')}-${String(date2.getDate()).padStart(2, '0')}T${String(date2.getHours()).padStart(2, '0')}:00:00Z`;
			temp = String(global_weather.get(dateStr)?.temperature_2m);
			weather = Number(global_weather.get(dateStr)?.weather_code);
		}

		let soleil = new Set([0, 1]);
		let soleil_nuage = new Set([2]);
		let nuage = new Set([3]);
		let pluie = new Set([51, 53, 55, 61, 63, 65, 80, 81, 82]);
		let verglas = new Set([56, 57, 66, 67]);
		let brouillard = new Set([45, 48]);
		let neige = new Set([71, 73, 75, 77, 85, 86]);
		let orage = new Set([95, 96, 99]);

		if (soleil.has(weather)) {
			src_img = '/meteo/soleil.png';
		} else if (soleil_nuage.has(weather)) {
			src_img = '/meteo/soleil-nuage.svg';
		} else if (nuage.has(weather)) {
			src_img = '/meteo/nuage.svg';
		} else if (pluie.has(weather)) {
			src_img = '/meteo/pluie.svg';
		} else if (verglas.has(weather)) {
			src_img = '/meteo/verglas.png';
		} else if (brouillard.has(weather)) {
			src_img = '/meteo/brouillard.png';
		} else if (neige.has(weather)) {
			src_img = '/meteo/neige.png';
		} else if (orage.has(weather)) {
			src_img = '/meteo/tempete.png';
		} else {
			src_img = '/meteo/interdit.png';
		}
	}

	/**
	 * Update the markers on the map
	 * We remove all the markers from the map and add the new ones without refreshing the entire map
	 */
	async function updateMapMarkers() {
		// Dynamically import CustomMarkers to avoid loading it on the server side. We verify that the mapContainer is not null for this
		if (mapContainer === null) return;
		const CustomMarkers = await import('$lib/CustomMarkers');
		const L = await import('leaflet');

		// We remove all the markers from the map
		if (map !== null) {
			map.eachLayer((layer) => {
				if (layer instanceof L.Marker) {
					map!.removeLayer(layer);
				}
			});
		}

		// We get the stations from the API and create a marker for each one
		let marker: CustomMarkers[] = [];
		let search = '';
		research.subscribe((value) => (search = value))();
		if (search == '') {
			marker = (await getStations()).map(
				(table) => new CustomMarkers.CustomMarkers(table.id, table.latitude, table.longitude)
			);
		} else {
			marker = (await getMarkersFromSearch(search)).map(
				(table) => new CustomMarkers.CustomMarkers(table.id, table.latitude, table.longitude)
			);
		}

		// We get the tables from the API and create a marker for each one
		markers.set(marker);

		// Now markers are stored, we can set the color of each marker
		const coloredMarker = await setMarkerColor();

		if (map !== null) {
			coloredMarker.forEach((marker) => {
				marker.getMarker().addTo(map!); // Add each marker to the map
			});
		}
		mapContainerStored.set(mapContainer);
	}

	/**
	 * Create the map and add it to the mapContainer
	 */
	async function createMap() {
		if (!mapContainer) return;

		// Dynamically import Leaflet and CustomMarkers to avoid loading them on the server side
		const L = await import('leaflet');

		// Initialize the map
		map = L.map(mapContainer).setView([45.74846, 4.84671], 13);
		map.zoomControl.setPosition('bottomright');
		L.tileLayer('https://igngp.geoapi.fr/tile.php/plan-ignv2/{z}/{x}/{y}.png', {
			attribution:
				'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
		}).addTo(map);

		await updateMapMarkers();
	}

	// onMount is a lifecycle function that is called when the component is mounted to the DOM
	onMount(async () => {
		await updateWeather();
		await createMap();
	});
</script>

<div>
	<input
		class="overlay overlay-date"
		value={`${dateJour.toISOString().split('T')[0]}`}
		type="date"
		id="date"
		name="date"
		placeholder="Date"
		min={new Date().toISOString().split('T')[0]}
		max={new Date(new Date().getTime() + 6 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]}
		on:change={(e: Event) => {
			if (e.target instanceof HTMLInputElement) {
				let minDate = new Date();
				let maxDate = new Date(new Date().getTime() + 6 * 24 * 60 * 60 * 1000);

				if (e.target.value === '') {
					dateJour = minDate;
					updateJour(minDate);
					return;
				}
				if (new Date(e.target.value) < minDate) {
					dateJour = minDate;
					updateJour(minDate);
				} else if (new Date(e.target.value) > maxDate) {
					dateJour = maxDate;
					updateJour(maxDate);
				} else {
					dateJour = new Date(e.target.value);
					updateJour(dateJour);
				}

				updateWeather();
			}
		}}
	/>

	<input
		class="overlay overlay-heure"
		type="time"
		id="heure"
		name="heure"
		value={localHoursAndMinutes}
		min={new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
		max="23:59"
		placeholder="Adresse"
		on:change={(e) => {
			if (e.target instanceof HTMLInputElement) {
				hoursAndMinutes = e.target.value;
				updateHour(Number(hoursAndMinutes.split(':')[0]));
				updateWeather();
			}
		}}
	/>

	<input
		class="overlay overlay-adress"
		type="text"
		id="adress"
		name="adress"
		placeholder="Rechercher..."
		on:change={(e) => {
			if (e.target instanceof HTMLInputElement) {
				updateEntries(e.target.value);
			}
		}}
	/>

	<img src={src_img} alt="Meteo" class="overlay overlay-meteo" />

	<div class="overlay overlay-temperature">
		{temp}°C
	</div>

	<img
		src="/dessin.svg"
		alt="Velovite, le site qui vous donne des vélos, vite."
		class="overlay overlay-logo"
	/>

	<map>
		<div id="map" bind:this={mapContainer}></div>
	</map>
</div>

<style>
	div {
		height: 100vh;
		width: 100vw;
		align-self: center;
		font-family: Verdana, Tahoma, sans-serif;
		position: relative;
	}

	.overlay {
		position: absolute;
		border-style: none;
		border-color: grey;
		font-size: medium;
		z-index: 999; /* ensure the overlay is above the map */
		background-color: rgba(255, 255, 255, 1); /* Optional : background for better readability */
		padding: 5px;
		border-radius: 60px;
		height: 60px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
	}

	.overlay-logo {
		top: 20px;
		right: 20px;
		padding: 0;
		background: none;
		border-radius: 60px;
	}

	.overlay-meteo {
		top: 20px;
		border-style: none;
		right: 100px;
		height: 60px;
		width: 70px;
		border-radius: 20px;
	}

	.overlay-temperature {
		top: 20px;
		border-style: none;
		right: 190px;
		height: 60px;
		width: 70px;
		border-radius: 20px;
		text-align: center;
		align-content: center;
		font-size: larger;
		font-weight: bold;
	}

	.overlay-date {
		padding: 10px;
		border-radius: 20px;
		top: 4%;
		left: 2%;
		height: 20px;
		width: 150px;
	}

	.overlay-heure {
		padding: 10px;
		border-radius: 20px;
		top: 12%;
		left: 2%;
		height: 20px;
		width: 150px;
	}

	.overlay-adress {
		padding: 10px;
		border-radius: 20px;
		top: 20%;
		left: 2%;
		height: 20px;
		width: 150px;
	}
</style>
