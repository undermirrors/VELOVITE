<script lang="ts">
    import 'leaflet/dist/leaflet.css';
    import { onMount } from 'svelte';

  let mapContainer: string | HTMLElement;

  onMount(async () => {
    const L = await import('leaflet');
    const map = L.map(mapContainer).setView([45.74846000, 4.84671000], 13);

    L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
    }).addTo(map);

  });
</script>

     <style>
        #map { height: 600px; width: 100%; }
     </style>

<div id="map" bind:this={mapContainer}></div>


