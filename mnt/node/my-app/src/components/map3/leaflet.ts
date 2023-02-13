import L from 'leaflet';
import 'leaflet-lasso';
import 'leaflet.markercluster';

export const key = Symbol();

export const createMap = (element: HTMLElement) => {
	const tile = 'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png';
	const map = L.map(element, {
		preferCanvas: true,
		zoomControl: false,
		doubleClickZoom: false,
		markerZoomAnimation: false
		// zoomAnimation: false,
		// fadeAnimation: false
	}).setView([35.132454, 136.978166], 13);
	L.tileLayer(tile, {
		attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
	        &copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
		subdomains: 'abcd'
	}).addTo(map);

	return map;
};

export type MapContext = {
	getMap: () => L.Map;
};

export { L };
