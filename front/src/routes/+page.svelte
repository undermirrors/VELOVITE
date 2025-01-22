<script lang="ts">
	import L from 'leaflet';
	import MapToolbar from './MapToolbar.svelte';
	import MarkerPopup from './MarkerPopup.svelte';
	import * as markerIcons from './markers.js';
	let map;

	const markerLocations = [
		[29.8283, -96.5795],
		[37.8283, -90.5795],
		[43.8283, -102.5795],
		[48.40, -122.5795],
		[43.60, -79.5795],
		[36.8283, -100.5795],
		[38.40, -122.5795],
	];

	const initialView = [39.8283, -98.5795];
	function createMap(container) {
		let m = L.map(container, { preferCanvas: true }).setView(initialView, 5);
		L.tileLayer(
			'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png',
			{
				attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
				&copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
				subdomains: 'abcd',
				maxZoom: 14,
			}
		).addTo(m);

		return m;
	}

	let toolbar = L.control({ position: 'topright' });
	let toolbarComponent;
	toolbar.onAdd = (map) => {
		let div = L.DomUtil.create('div');
		toolbarComponent = new MapToolbar({
			target: div,
			props: {}
		});

		toolbarComponent.$on('click-eye', ({ detail }) => eye = detail);
		toolbarComponent.$on('click-lines', ({ detail }) => lines = detail);
		toolbarComponent.$on('click-reset', () => {
			if (map) {
				map.setView(initialView, 5, { animate: true });
			}
		});

		return div;
	};
</script>
