<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import { onMount } from 'svelte';

	let mapContainer: string | HTMLElement;

	onMount(async () => {
		const L = await import('leaflet');
		const map = L.map(mapContainer, { zoomControl: false }).setView([45.74846, 4.84671], 13);
		new L.Control.Zoom({ position: 'bottomright' }).addTo(map);

		L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
			attribution:
				'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
		}).addTo(map);
	});
</script>

<div id="map" bind:this={mapContainer}></div>

<style>
	#map {
		height: 600px;
		width: 100%;
	}
</style>
