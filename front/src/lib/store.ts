import {writable} from 'svelte/store';
import type {CustomMarkers} from "$lib/CustomMarkers";

export const date = writable<Date>(new Date());

export const markers = writable<CustomMarkers[]>([]);

export const mapContainerStored = writable<HTMLElement | null>(null);

export const research = writable<string>('');