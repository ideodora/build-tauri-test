# Svelte challenge

# this is for firebase

# setup

- comment out attaching `docker` volumes
- dcu
- open node container in remote devcontainer
- `npm create svelte@latest my-app`
- close remote devcontainer
- attach node_modules volume for svelte
- open devcontainer again
- set workspace to `my-app` to make IDE easier
- npm install
- npm install -D tailwindcss postcss autoprefixer prettier-plugin-tailwindcss @tailwindcss/typography @tailwindcss/forms @tailwindcss/line-clamp @tailwindcss/aspect-ratio
- npx tailwindcss init tailwind.config.cjs -p
- Enable use of PostCSS in <style> blocks

```
# svelte.config.js

import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter()
  },
  preprocess: vitePreprocess()
};

export default config;
```

- Configure your template paths

```
# tailwind.config.cjs

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {}
  },
  plugins: [
    require('@tailwindcss/typography'),
		require('@tailwindcss/forms'),
		require('@tailwindcss/line-clamp'),
		require('@tailwindcss/aspect-ratio')
  ]
};
```

- Add the Tailwind directives to your CSS

```
# ./src/app.css

@tailwind base;
@tailwind components;
@tailwind utilities;
```

- import css file

```
# ./src/routes/+layout.svelte

<script>
  import "../app.css";
</script>

<slot />
```

- change npm script run dev

```
"scripts": {
    # add --host for run in conainer access from host
		"dev": "vite dev --host",
```

- `npm run dev` to start
- be aware of localhost service worker worked from another app

## firebase

- inside devcontainer
- firebase login --no-localhost
- login in firebase copy and paste code
- firebase init
