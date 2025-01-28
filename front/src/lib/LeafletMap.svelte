<script lang="ts">
    import 'leaflet/dist/leaflet.css';
    import {onMount} from 'svelte';
    import {getDetailsById, getTables, getWeatherForecast} from '$lib/rust_api';


    let mapContainer: string | HTMLElement;

    onMount(async () => {

        const L = await import('leaflet');

        class Icon extends L.Icon {
            constructor() {
                super({iconUrl: 'marker_icon.svg'});
            }
        }

        class Markers {
            id: number;
            marker: L.Marker;

            constructor(id: number, latitude: number, longitude: number) {
                this.id = id;
                const blackIcon = new Icon();
                this.marker = L.marker([latitude, longitude], {icon: blackIcon}).addEventListener('click', async () => {
                    let advanced_data = await getDetailsById(this.id);
                    console.log(advanced_data);
                });
            }

            getMarker() {
                return this.marker;
            }

            getId() {
                return this.id;
            }
        }

        const map = L.map(mapContainer).setView([45.74846, 4.84671], 13);
        map.zoomControl.setPosition('bottomright');
        L.tileLayer('http://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution:
                '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(map);

        let markers = (await getTables()).map(table => new Markers(table.id, table.latitude, table.longitude));
        markers.forEach(marker => marker.getMarker().addTo(map));

        let meteo: Map<string, WeatherForecast> = await getWeatherForecast();
        console.log(meteo.get("2025-01-27T00:00:00Z"));
    });
</script>

<div id="map" bind:this={mapContainer}></div>

<style>
    #map {
        height: 100vh;
        width: 100vw;
    }
</style>
