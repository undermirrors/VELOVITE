<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import { onMount } from 'svelte';

	let mapContainer: string | HTMLElement;

	const markerLocations = [];

	onMount(async () => {
		const L = await import('leaflet');
		const map = L.map(mapContainer).setView([45.74846, 4.84671], 13);
		map.zoomControl.setPosition('bottomright');
		L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
			attribution:
				'&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
		}).addTo(map);

		//markers
		var icon = L.Icon.extend({
			options: {
				iconUrl: 'marker_icon.svg'
			}
		});

		var blackIcon = new icon();

		L.marker([45.7484, 4.84671], { icon: blackIcon }).addTo(map);
	});
</script>

<div id="map" bind:this={mapContainer}></div>

<style>
    #map {
        height: 600px;
        width: 100%;
    }
</style>
