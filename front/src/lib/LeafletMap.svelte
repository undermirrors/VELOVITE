<script lang="ts">
<<<<<<< Updated upstream
    import 'leaflet/dist/leaflet.css';
    import {onMount} from 'svelte';
    import {getDetailsById, getTables} from '$lib/rust_api';

    let mapContainer: string | HTMLElement;

    let tables: Table[];

	onMount(async () => {
		const L = await import('leaflet');
		const map = L.map(mapContainer, { zoomControl: false }).setView([45.74846, 4.84671], 13);
		new L.Control.Zoom({ position: 'bottomright' }).addTo(map);

        L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(map);
        tables = await getTables();
        tables.forEach(table => {
            // here to add marker
            // L.marker([table.latitude, table.longitude]).addTo(map);
        });
        let res: Details;
        res = await getDetailsById(1012);
        console.log(res);
    });


=======
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
>>>>>>> Stashed changes
</script>

<div id="map" bind:this={mapContainer}></div>

<style>
	#map {
		height: 600px;
		width: 100%;
	}
</style>
