import { invoke } from '@tauri-apps/api/tauri';

export async function loadCurve(feature: any) {
	const { pid, prefCode } = feature.properties;
	const curveId = { pid, pref_code: prefCode };

	const result = await invoke<any>('curves', { curveId });

	return result.curves.map((curve: any) => {
		const splits: string[] = curve.segments.split('\\\\r\\\\n');
		const segments = splits.map((split) => {
			const splitStrs = split.split(' ');
			return [parseFloat(splitStrs[1]), parseFloat(splitStrs[0])];
		});
		return { curveId: curve.id, segments };
	});
}

export async function loadCurveByKey(curveKey: any) {
	const result = await invoke<any>('get_curves', { curveKey });

	return result.curves.map((curve: any) => {
		const splits: string[] = curve.segments.split('\\\\r\\\\n');
		const segments = splits.map((split) => {
			const splitStrs = split.split(' ');
			return [parseFloat(splitStrs[1]), parseFloat(splitStrs[0])];
		});
		return { curveId: curve.id, segments };
	});
}
