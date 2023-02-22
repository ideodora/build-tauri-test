/// <reference types="vite/client" />

interface ImportMetaEnv {
	readonly VITE_GOOGLEMAP_API_KEY: string;
	readonly VITE_USE_GOOGLEMAP_TILE: string;
	// その他の環境変数...
}

interface ImportMeta {
	readonly env: ImportMetaEnv;
}
