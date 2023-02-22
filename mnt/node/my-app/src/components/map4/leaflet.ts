import L from 'leaflet';
import 'leaflet-lasso';
import 'leaflet.markercluster';
import '@geoman-io/leaflet-geoman-free';
import 'leaflet.gridlayer.googlemutant/dist/Leaflet.GoogleMutant';
import { Loader as GoogleMapApiLoader } from '@googlemaps/js-api-loader';

import iconRetinaUrl from '~/assets/marker-icon-2x.png';
import iconUrl from '~/assets/marker-icon.png';
import shadowUrl from '~/assets/marker-shadow.png';

export type { LassoHandlerFinishedEvent } from 'leaflet-lasso';

declare global {
	namespace L {
		namespace gridLayer {
			function googleMutant(options: any): any;
		}
	}
}

const USE_GOOGLEMAP_TILE = import.meta.env.VITE_USE_GOOGLEMAP_TILE ?? 'no';
const GOOGLEMAP_API_KEY = import.meta.env.VITE_GOOGLEMAP_API_KEY ?? '';

export const key = Symbol();

export const createMap = async (element: HTMLElement) => {
	if (USE_GOOGLEMAP_TILE === 'yes') {
		const googleMapApiLoader = new GoogleMapApiLoader({
			apiKey: GOOGLEMAP_API_KEY,
			version: 'weekly'
		});
		await googleMapApiLoader.load();
	}

	const map = L.map(element, {
		// preferCanvas: true,
		maxZoom: 20,
		zoomControl: false,
		doubleClickZoom: false,
		markerZoomAnimation: false
		// zoomAnimation: false,
		// fadeAnimation: false
	}).setView([35.132454, 136.978166], 13);

	setTileToMap(map);

	return map;
};

function setTileToMap(map: L.Map): void {
	if (USE_GOOGLEMAP_TILE === 'yes') {
		L.gridLayer
			.googleMutant({
				type: 'roadmap'
				// styles: [
				//   { featureType: 'poi', stylers: [{ visibility: 'off' }] },
				// ],
			})
			.addTo(map);
		return;
	}

	const tile = 'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png';
	L.tileLayer(tile, {
		attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
	        &copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
		subdomains: 'abcd'
	}).addTo(map);
}

export type MapContext = {
	getMap: () => L.Map;
	selectable: () => boolean;
};

export { L };

export const iconDefault = () => {
	return L.icon({
		iconRetinaUrl,
		iconUrl,
		shadowUrl,
		iconSize: [25, 41],
		iconAnchor: [12, 41],
		popupAnchor: [1, -34],
		tooltipAnchor: [16, -28],
		shadowSize: [41, 41]
	});
};

export const startIcon = () => {
	return L.divIcon({
		html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 14"><defs><style>.cls-1{fill:#fff;}.cls-2{fill:#1fad19;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-1" cx="7" cy="7" r="6.5"/><path class="cls-2" d="M7,1A6,6,0,1,1,1,7,6,6,0,0,1,7,1M7,0a7,7,0,1,0,7,7A7,7,0,0,0,7,0Z"/></g></g></svg>',
		iconSize: [14, 14],
		iconAnchor: [7, 7],
		className: ''
	});
};

export const endIcon = () => {
	return L.divIcon({
		html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"><defs><style>.cls-12{fill:#fff;}.cls-22{fill:#c70143;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-12" cx="4" cy="4" r="3.5"/><path class="cls-22" d="M4,1A3,3,0,1,1,1,4,3,3,0,0,1,4,1M4,0A4,4,0,1,0,8,4,4,4,0,0,0,4,0Z"/></g></g></svg>',
		iconSize: [8, 8],
		iconAnchor: [4, 4],
		className: ''
	});
};

export const startIconBig = () => {
	return L.divIcon({
		html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 14"><defs><style>.cls-1{fill:#fff;}.cls-2{fill:#1fad19;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-1" cx="7" cy="7" r="6.5"/><path class="cls-2" d="M7,1A6,6,0,1,1,1,7,6,6,0,0,1,7,1M7,0a7,7,0,1,0,7,7A7,7,0,0,0,7,0Z"/></g></g></svg>',
		iconSize: [16, 16],
		iconAnchor: [8, 8],
		className: ''
	});
};

export const endIconBig = () => {
	return L.divIcon({
		html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"><defs><style>.cls-12{fill:#fff;}.cls-22{fill:#c70143;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-12" cx="4" cy="4" r="3.5"/><path class="cls-22" d="M4,1A3,3,0,1,1,1,4,3,3,0,0,1,4,1M4,0A4,4,0,1,0,8,4,4,4,0,0,0,4,0Z"/></g></g></svg>',
		iconSize: [16, 16],
		iconAnchor: [8, 8],
		className: ''
	});
};

export type FeaturePropertyKind =
	| 'SegmentLine'
	| 'SegmentStart'
	| 'SegmentEnd'
	| 'SegmentZone'
	| 'StartingPoint';

export type FeatureProperty = {
	id: string;
	kind: FeaturePropertyKind;
};

export type FeaturePropertySegmentZone = FeatureProperty & {
	kind: 'SegmentZone';
	fillOpacity: number;
	weight: number;
	color: string;
};
