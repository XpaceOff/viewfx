// For more info of what stores are check https://svelte.dev/tutorial/writable-stores

import { writable } from 'svelte/store';


// 0: non-cached
// 1: caching
// 2: cached
export const barFrameCacheStatus = writable([]);