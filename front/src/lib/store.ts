import {writable} from 'svelte/store';
import type {CustomMarkers} from "$lib/CustomMarkers";

/**
 * Store for the date in order to get in all the components
 */
export const date = writable<Date>(new Date());

/**
 * Store for the map markers in order to get in all the components
 */
export const markers = writable<CustomMarkers[]>([]);

/**
 * Store for the container of the map in order to refresh it
 */
export const mapContainerStored = writable<HTMLElement | null>(null);

/**
 * Store for the research in order to get in all the components
 */
export const research = writable<string>('');