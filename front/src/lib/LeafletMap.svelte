<script lang="ts">
    import 'leaflet/dist/leaflet.css';
    import {onMount} from 'svelte';
    import {getTables, setMarkerColor} from '$lib/rust_api';
    import {mapContainerStored, markers} from '$lib/store';

    let mapContainer: HTMLElement | null = null;

    // onMount is a lifecycle function that is called when the component is mounted to the DOM
    onMount(async () => {
        if (!mapContainer) return;

        // Dynamically import Leaflet and CustomMarkers to avoid loading them on the server side
        const L = await import('leaflet');
        const CustomMarkers = await import("$lib/CustomMarkers");

        const map = L.map(mapContainer).setView([45.74846, 4.84671], 13);
        map.zoomControl.setPosition('bottomright');
        L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution:
                '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(map);

        // We get the tables from the API and create a marker for each one
        let marker = (await getTables()).map(table => new CustomMarkers.CustomMarkers(table.id, table.latitude, table.longitude));
        markers.set(marker);

        // Now markers is stored, we can set the color of each marker
        const coloredMarker = await setMarkerColor();

        coloredMarker.forEach(marker => {
            marker.getMarker().addTo(map); // Add each marker to the map
        });

        mapContainerStored.set(mapContainer);
    });
</script>

<div id="map" bind:this={mapContainer}></div>

<style>
    #map {
        height: 100vh;
        width: 100vw;
    }
</style>
