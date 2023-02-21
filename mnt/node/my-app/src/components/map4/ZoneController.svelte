<script lang="ts">
	import { save as dialogSave } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { default as turfBuffer } from '@turf/buffer';
	import { default as turfSimplify } from '@turf/simplify';
	import tokml from 'tokml';
	import {
		activeFeatureStoreArray,
		featureStore,
		isSegmentFeature,
		isZoneFeature,
		type ZoneFeature
	} from '~/components/map4/watershedStore';
	// import { type FeaturePropertySegmentZone } from '~/components/map4/leaflet';
	import { asgstr, isComposingZone, sgstr } from '~/components/mapStore';
	import { nanoid } from 'nanoid';

	export const buildZone = (num: number = 100) => {
		// tempController.clearLayers();
		const id = 'temp:zone';
		featureStore.remove(id);

		for (const feature of $activeFeatureStoreArray) {
			if (!isSegmentFeature(feature)) continue;

			$isComposingZone = true;

			const line = feature.line;
			const buffered = turfBuffer(line, num, {
				units: 'meters'
			});
			const tolerance = num * 0.0000025; // 20m => 0.00005
			const simplified = turfSimplify(buffered, {
				tolerance
			}) as GeoJSON.Feature<
				GeoJSON.Polygon,
				{
					id: string;
					layerId?: number;
					kind: 'Zone';
					weight: number;
					color: string;
					fillOpacity: number;
				}
			>;
			simplified.properties.id = id;
			simplified.properties.kind = 'Zone';
			simplified.properties.color = '#00FF00';
			simplified.properties.weight = 1;
			simplified.properties.fillOpacity = 0.3;

			const zoneFeature = {
				kind: 'ZoneFeature',
				id,
				zone: simplified
			} satisfies ZoneFeature;

			featureStore.add(id, zoneFeature);

			// tempController.addData(simplified);

			// seg.zone = simplified;
			// if (zone) {
			// 	seg.features.pop();
			// }
			// seg.features.push(simplified);
			// controller.addData(simplified);
		}
	};

	export const fixedZone = () => {
		const tempId = 'temp:zone';
		const feature = $featureStore.get(tempId);
		if (!feature) return;
		if (!isZoneFeature(feature)) return;

		const random = nanoid();
		const id = `${random}:zone`;

		feature.id = id;
		feature.zone.properties.id = id;
		feature.zone.properties.color = '#0EA5E9';

		featureStore.add(id, feature);
		featureStore.remove(tempId);

		// const layers = tempController.getLayers();
		// const layer = layers[0];
		// // const feature = (layer as any).feature;

		// feature.properties.color = '#0EA5E9';

		// const curveId = feature.properties.id.split(':')[0];
		// const segment = $sgstr.get(curveId);

		// const orgZone = segment.zone;
		// if (orgZone) {
		// 	segment.features.pop();
		// }
		// segment.zone = feature;
		// segment.features.push(feature);

		// controller.addData(feature);
		$isComposingZone = false;
	};

	// export const exportSegments = async (ev: any) => {
	// 	console.log(ev.fileName);

	// 	const path = await dialogSave({
	// 		defaultPath: 'exports.zip',
	// 		filters: [
	// 			{
	// 				name: 'Zip',
	// 				extensions: ['zip']
	// 			}
	// 		]
	// 	});
	// 	if (path) {
	// 		const mapper = new Map<string, string>([]);

	// 		for (const seg of $asgstr.values()) {
	// 			const lineKml = tokml(seg.line);
	// 			mapper.set(convertIdToKmlFileName(seg.line.properties.id), lineKml);

	// 			if (seg.zone) {
	// 				const zoneKml = tokml(seg.zone);
	// 				mapper.set(convertIdToKmlFileName(seg.zone.properties.id), zoneKml);
	// 			}
	// 		}

	// 		// writeTextFile(path, 'abc');

	// 		const body = {
	// 			path,
	// 			data: [...mapper.entries()]
	// 		};

	// 		await invoke('save_export', { body });
	// 	}
	// };

	// const convertIdToKmlFileName = (id: string) => {
	// 	const replaced = id.replace(':', '_');
	// 	return `${replaced}.kml`;
	// };
</script>
