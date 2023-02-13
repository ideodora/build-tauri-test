<script context="module" lang="ts">
	import L from 'leaflet';
	import type { LassoHandler } from 'leaflet-lasso';

	import { point } from '@turf/helpers';

	import iconRetinaUrl from '~/assets/marker-icon-2x.png';
	import iconUrl from '~/assets/marker-icon.png';
	import shadowUrl from '~/assets/marker-shadow.png';
	import circleUrl from '~/assets/circle.png';

	import type { createEventDispatcher as ICreateEventDispatcher } from 'svelte';
	type Dispatcher = ReturnType<typeof ICreateEventDispatcher<{ clickStartingPoint: any }>>;

	let dispatch: Dispatcher;
	let map: L.Map;
	let elm: any;
	let markerLayer: L.LayerGroup;
	let startingPointCluster: L.MarkerClusterGroup;
	let lasso: LassoHandler;

	export const iconDefault = () =>
		L.icon({
			iconRetinaUrl,
			iconUrl,
			shadowUrl,
			iconSize: [25, 41],
			iconAnchor: [12, 41],
			popupAnchor: [1, -34],
			tooltipAnchor: [16, -28],
			shadowSize: [41, 41]
		});

	export const circleIcon = () =>
		L.icon({
			iconRetinaUrl: circleUrl,
			iconUrl: circleUrl,
			iconSize: [12, 12],
			iconAnchor: [6, 6]
		});

	export const startIcon = () =>
		L.divIcon({
			html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 14"><defs><style>.cls-1{fill:#fff;}.cls-2{fill:#1fad19;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-1" cx="7" cy="7" r="6.5"/><path class="cls-2" d="M7,1A6,6,0,1,1,1,7,6,6,0,0,1,7,1M7,0a7,7,0,1,0,7,7A7,7,0,0,0,7,0Z"/></g></g></svg>',
			iconSize: [14, 14],
			iconAnchor: [7, 7],
			className: ''
		});

	export const endIcon = () =>
		L.divIcon({
			html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"><defs><style>.cls-12{fill:#fff;}.cls-22{fill:#c70143;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-12" cx="4" cy="4" r="3.5"/><path class="cls-22" d="M4,1A3,3,0,1,1,1,4,3,3,0,0,1,4,1M4,0A4,4,0,1,0,8,4,4,4,0,0,0,4,0Z"/></g></g></svg>',
			iconSize: [8, 8],
			iconAnchor: [4, 4],
			className: ''
		});

	export const startIconBig = () =>
		L.divIcon({
			html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 14"><defs><style>.cls-1{fill:#fff;}.cls-2{fill:#1fad19;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-1" cx="7" cy="7" r="6.5"/><path class="cls-2" d="M7,1A6,6,0,1,1,1,7,6,6,0,0,1,7,1M7,0a7,7,0,1,0,7,7A7,7,0,0,0,7,0Z"/></g></g></svg>',
			iconSize: [16, 16],
			iconAnchor: [8, 8],
			className: ''
		});

	export const endIconBig = () =>
		L.divIcon({
			html: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"><defs><style>.cls-12{fill:#fff;}.cls-22{fill:#c70143;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><circle class="cls-12" cx="4" cy="4" r="3.5"/><path class="cls-22" d="M4,1A3,3,0,1,1,1,4,3,3,0,0,1,4,1M4,0A4,4,0,1,0,8,4,4,4,0,0,0,4,0Z"/></g></g></svg>',
			iconSize: [16, 16],
			iconAnchor: [8, 8],
			className: ''
		});

	export function getBounds() {
		return map.getBounds();
	}

	export function addStartingPoint(startingPoint: any) {
		// const id = `${startingPoint.lat}${startingPoint.lng}`;
		// const pointFeature = point([startingPoint.lng, startingPoint.lat], {
		// 	id,
		// 	kind: 'StartingPoint',
		// 	icon: iconDefault(),
		// 	zIndexOffset: 3000,
		// 	riverName: 'riverName',
		// 	riverCode: 'riverCode'
		// });
		// const geoJson = L.geoJSON(pointFeature, {
		// 	onEachFeature: (feature, layer) => {
		// 		layer.bindTooltip(`${feature.properties.riverName} (${feature.properties.riverCode})`);
		// 		layer.on('click', (ev) => {
		// 			console.log(ev);
		// 			console.log(layer);
		// 			console.log(feature);
		// 			dispatch(`clickStartingPoint`, { feature });
		// 		});
		// 	},
		// 	pointToLayer: function (feature, latlng) {
		// 		const marker = L.marker(latlng, {
		// 			icon: feature.properties.icon,
		// 			zIndexOffset: feature.properties.zIndexOffset
		// 		});
		// 		return marker;
		// 	}
		// });
		// startingPointCluster.addLayer(geoJson);
	}

	export function removeStartingPoints() {
		startingPointCluster.clearLayers();
	}
</script>

<script lang="ts">
	console.log('map script');
	import { sMap, segments as storeSegments } from '~/components/mapStore';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { createEventDispatcher } from 'svelte';
	import StartingPointCluster from '~/components/StartingPointCluster.svelte';
	import SegmentCluster from '~/components/SegmentCluster.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import LassoControl from '~/components/map-control/LassoControl.svelte';

	dispatch = createEventDispatcher();

	onMount(async () => {
		if (browser) {
			await import('leaflet.markercluster');
			const Llasso = await import('leaflet-lasso');

			L.Marker.prototype.options.icon = iconDefault();

			// L = await import('leaflet');
			// const maobxTile =
			// 	'https://api.mapbox.com/styles/v1/ideodora/cku5gzltg2l8f19qjjxfx7fod/tiles/256/{z}/{x}/{y}@2x?access_token=pk.eyJ1IjoiaWRlb2RvcmEiLCJhIjoiY2lsaGJqYXV6MmJ5MXY2bTBwdHcwaWx1NCJ9.Ac5vxfupLYxTefZfP-lL1g';
			const tile = 'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png';
			map = L.map(elm, {
				preferCanvas: true,
				zoomControl: false,
				doubleClickZoom: false,
				zoomAnimation: false,
				markerZoomAnimation: false
				// fadeAnimation: false
			}).setView([35.132454, 136.978166], 13);
			L.tileLayer(tile, {
				attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
	        &copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
				subdomains: 'abcd'
			}).addTo(map);

			markerLayer = L.layerGroup();
			markerLayer.addTo(map);

			startingPointCluster = L.markerClusterGroup();
			startingPointCluster.addTo(map);

			sMap.set(map);

			lasso = L.lasso(map);

			// for geojson layer zindex
			map.createPane('buffers').style.zIndex = '250';
		}
	});

	const onClickedStartingPoint = async (event: any) => {
		console.log(event.detail);
		const { pid, prefCode } = event.detail.feature.properties;
		const curveId = { pid, pref_code: prefCode };

		const result = await invoke<any>('curves', { curveId });

		console.log(result.curves);
		result.curves.forEach((curve: any) => {
			const splits: string[] = curve.segments.split('\\\\r\\\\n');
			const segments = splits.map((split) => {
				const splitStrs = split.split(' ');
				return [parseFloat(splitStrs[1]), parseFloat(splitStrs[0])];
			});
			const segment = { curveId: curve.id, segments };

			const mapper = new Map($storeSegments.map((item) => [item.curveId, item]));
			mapper.set(curve.id, segment);
			storeSegments.set([...mapper.values()]);
		});
	};

	let sc: SegmentCluster;

	export const createZone = (num: number) => {
		sc.createZone(num);
	};
</script>

<div class="map" style="height:100%;width:100%" bind:this={elm} />
{#if $sMap}
	<StartingPointCluster map={$sMap} on:clickedStartingPoint={onClickedStartingPoint} />
	<SegmentCluster map={$sMap} bind:this={sc} />
	<LassoControl map={$sMap} />
{/if}
