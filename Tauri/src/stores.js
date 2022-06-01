// For more info of what stores are check https://svelte.dev/tutorial/writable-stores

import { writable } from 'svelte/store';


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
// if tree a Modal is open
export const isModalActive = writable(true);

export const modalTittle = writable(""); // Modal Tittle

export const modalSelectedDirPath = writable("");   // Current directory
export const modalListOfFiles = writable("");       // List Of files in the current directory
export const modalListOfFilesError = writable("");  // Error if problem with current directory
