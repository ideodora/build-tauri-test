<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { save as dialogSave } from '@tauri-apps/api/dialog';
	import tokml from 'tokml';
	import { activeFeatureStoreArray, isSegmentFeature, isZoneFeature } from '~/store/featureStore';

	export const exportFeatures = async () => {
		const path = await dialogSave({
			defaultPath: 'exports.zip',
			filters: [
				{
					name: 'Zip',
					extensions: ['zip']
				}
			]
		});
		if (path) {
			const mapper = new Map<string, string>([]);

			for (const feature of $activeFeatureStoreArray) {
				if (isSegmentFeature(feature)) {
					const line = feature.line;
					const lineKml = tokml(line);
					console.log(line.properties.id);
					mapper.set(convertIdToKmlFileName(line.properties.id), lineKml);
				}
				if (isZoneFeature(feature)) {
					const zone = feature.zone;
					const zoneKml = tokml(zone);
					console.log(zone.properties.id);
					mapper.set(convertIdToKmlFileName(zone.properties.id), zoneKml);
				}
			}

			const body = {
				path,
				data: [...mapper.entries()]
			};

			await invoke('save_export', { body });
		}
	};

	export const saveActivesAs = async (fileName: string) => {
		console.log('watershed');
		console.log('saveActivesAs:', fileName);

		const mapper = new Map<string, string>([]);
		for (const feature of $activeFeatureStoreArray) {
			console.log(feature.id, feature);
			mapper.set(feature.id, JSON.stringify(feature));
		}
		console.log(mapper);

		// TODO: maybe calculate bounds here and save

		const body = {
			name: fileName,
			data: [...mapper.entries()]
		};
		await invoke('create_watershed', { body });
	};

	const convertIdToKmlFileName = (id: string) => {
		const replaced = id.replaceAll(':', '_');
		return `${replaced}.kml`;
	};
</script>
