<script>
	import { onMount } from 'svelte';
	import LeafletMap from '$lib/LeafletMap.svelte';
	import { getWeatherForecast } from '$lib/rust_api';

	// Obtenir la date actuelle au format ISO (YYYY-MM-DD)
	const today = new Date().toISOString().split('T')[0];

	let jour = today;

	// Obtenir l'heure actuelle au format HH:MM
	const now = new Date();
	const currentHour = now.getHours().toString().padStart(2, '0'); // Ajouter un 0 si nécessaire
	const currentMinutes = now.getMinutes().toString().padStart(2, '0');
	const currentTime = `${currentHour}:${currentMinutes}`;

	let heure = currentTime;

	let timeZ = jour + 'T' + heure + 'Z';
	let temp = '';
	let meteo = -1;

	onMount(async () => {
		let global_meteo = await getWeatherForecast();
		if (global_meteo !== null) {
			temp = String(global_meteo.get(timeZ)?.temperature_2m) ?? '?';
			meteo = Number(global_meteo.get(timeZ)?.weather_code) ?? -1;
		}
	});

	// Visuel météo

	let src_img = '/meteo/Interdit.png';

	let soleil = new Set([0, 1]);
	let soleil_nuage = new Set([2]);
	let nuage = new Set([3]);
	let pluie = new Set([51, 53, 55, 61, 63, 65, 80, 81, 82]);
	let verglas = new Set([56, 57, 66, 67]);
	let brouillard = new Set([45, 48]);
	let neige = new Set([71, 73, 75, 77, 85, 86]);
	let orage = new Set([95, 96, 99]);

	if (soleil.has(meteo)) {
		src_img = '/meteo/autre-chose.png';
	} else if (soleil_nuage.has(meteo)) {
		src_img = '/meteo/soleil-nuage.svg';
	} else if (nuage.has(meteo)) {
		src_img = '/meteo/nuage.svg';
	} else if (pluie.has(meteo)) {
		src_img = '/meteo/pluie.svg';
	} else if (verglas.has(meteo)) {
		src_img = '/meteo/Verglas.png';
	} else if (brouillard.has(meteo)) {
		src_img = '/meteo/Brouillard.png';
	} else if (neige.has(meteo)) {
		src_img = '/meteo/Neige.png';
	} else if (orage.has(meteo)) {
		src_img = '/meteo/tempete.png';
	} else {
		src_img = '/meteo/Interdit.png';
	}

	//paramètres température
</script>

<div>
	<input
		class="overlay overlay-date"
		bind:value={jour}
		type="date"
		id="date"
		name="date"
		placeholder="Date"
	/>

	<input
		class="overlay overlay-heure"
		type="time"
		id="heure"
		name="heure"
		value={currentTime}
		placeholder="Adresse"
	/>

	<input
		class="overlay overlay-adress"
		type="text"
		id="adress"
		name="adress"
		placeholder="Adresse"
	/>

	<div class="overlay overlay-stations">Stations proches</div>

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
		<LeafletMap />
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
		z-index: 999; /* Plus élevé que la carte */
		background-color: rgba(255, 255, 255, 1); /* Optionnel : un fond pour plus de lisibilité */
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

	.overlay-stations {
		top: 28%;
		left: 2%;
		padding: 10px;
		border-radius: 20px;
		height: 200px;
		width: 150px;
	}
</style>
