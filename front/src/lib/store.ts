import {writable} from 'svelte/store';
import type {CustomMarkers} from "$lib/CustomMarkers";

export const date = writable<string>(new Date().toISOString());

export const markers = writable<CustomMarkers[]>([]);

export const mapContainerStored = writable<HTMLElement | null>(null);
