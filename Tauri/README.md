
## Start a new Tauri project

```bash
# Create a new tauri app
$ npx create-tauri-app

# Start tauri development window
$ npm run tauri dev

# Build an Tauri app
$ npm run tauri build
```

## Keep Tauri dependencies up to date

```bash
# If you use yarn
$ yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

# if you use npm
$ npm install @tauri-apps/cli@latest @tauri-apps/api@latest

# Detect versions
$ npm outdated @tauri-apps/cli
$ yarn outdated @tauri-apps/cli
```

## Install Tailwind on my Svelte project

https://css-tricks.com/how-to-use-tailwind-on-a-svelte-site/

```bash
$ npm install -D tailwindcss postcss autoprefixer svelte-preprocess
$ npx tailwindcss init tailwind.config.js 
```

tailwind.config.js
```css
module.exports = {
    //The content section is where you configure the paths to all 
    // files that contain Tailwind class names.
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {},
    },
    plugins: [],
}
```

rollup.config.js
```js
import sveltePreprocess from "svelte-preprocess";

plugins: [
		svelte({
			preprocess: sveltePreprocess({
				sourceMap: !production,
				postcss: {
				  plugins: [
				   require("tailwindcss"), 
				   require("autoprefixer"),
				  ],
				},
			}),
```