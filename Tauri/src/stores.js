// For more info of what stores are check https://svelte.dev/tutorial/writable-stores

import { writable } from 'svelte/store';

// Video A to be loaded or currently loaded.
export const mediaA = writable(null);

// Canvas size [x, y]
export const canvasSize = writable([0, 0]);

// 0: non-cached
// 1: caching
// 2: cached
export const barFrameCacheStatus = writable([]);

export const barCurrentFrame = writable([]);

// Total of frames in video
export const videoTotalFrameLength = writable(0);

// Start frame of the video
export const videoStartFrame = writable(0);

// Video's current frame
// Frame that is selected or displayed at the time
export const videoCurrentFrame = writable(0);

// Pause Status
// If off then video plays
export const isVideoPaused = writable(true);

// Modal Status
// if true, a Modal is open
export const isModalActive = writable(false);

export const modalTittle = writable(""); // Modal Tittle

export const modalSelectedDirPath = writable("");   // Current directory
export const modalSelectedMedia = writable("");     // Selected media to import
export const modalListOfFiles = writable("");       // List Of files in the current directory
export const modalListOfFilesError = writable("");  // Error if problem with current directory
