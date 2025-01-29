<script lang="ts">
	import { onMount } from 'svelte';
	import LeafletMap from '$lib/LeafletMap.svelte';
	import { getWeatherForecast } from '$lib/rust_api';
	import { date, research } from '$lib/store';

	// Force re-rendering of the map and the weather icon
	let mapKey = 0;

	/**
	 * Update date and time selected by user,
	 * and reload the map in order to update the color of the markers
	 *
	 * @param selectedDate
	 */
	async function updateDate(selectedDate: Date) {
		date.set(selectedDate);
		mapKey++;
	}

	/**
	 * Update the date of the selected day only, and keep the time,
	 * and reload the map in order to update the color of the markers
	 *
	 * @param selectedJour : date chose by the user
	 */
	function updateJour(selectedJour: Date) {
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

		// reload the map
		mapKey++;
	}

	/**
	 * Update the hour of the selected day only, and keep the date,
	 * and reload the map in order to update the color of the markers
	 *
	 * @param selectedHour : hour chose by the user
	 */
	function updateHour(selectedHour: number) {
		// get the previous selected date
		let newDate: Date = new Date(selectedHour);
		date.subscribe((value) => (newDate = value))();

		// modify the hour of the new date
		newDate.setHours(selectedHour);
		date.set(newDate);

		// reload the map
		mapKey++;
	}

	/**
	 * Update the research entered by the user,
	 * and reload the map in order to update number of markers
	 *
	 * @param value : address or name of the station entered by the user
	 */
	function updateEntries(value: string) {
		research.set(value);
		mapKey++;
	}

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

	// Initialize the variables used for weather icon and temperature
	let temp = '';
	let weather = -1;
	let src_img = '';

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

	// onMount is a lifecycle function that is called when the component is mounted to the DOM
	onMount(async () => {
		await updateWeather();
	});

	// Visuel météo
	//paramètres température
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
				console.log(e.target.value);
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
		value={hoursAndMinutes}
		min={new Date().getHours() + ':' + new Date().getMinutes()}
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
		placeholder="Adresse"
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
		{#key mapKey}
			<LeafletMap />
		{/key}
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
