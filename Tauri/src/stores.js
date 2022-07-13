// For more info of what stores are check https://svelte.dev/tutorial/writable-stores

import { writable } from 'svelte/store';

// Contains the platform-specific path segment separator:
// `\` on Windows
// `/` on POSIX
export const osSepChar = writable('\\');

export const mediaToBeImported = writable("");

// This two variables shows which img is shown on canvas
// if it's A then B have to be false
export const imgDrawOnCanvasIsA = writable(true);
export const imgDrawOnCanvasIsB = writable(false);

// If this is true then canvas will show the 
// difference between A and B
export const imgDrawOnCanvasIsDiff = writable(false);

// If this is true then canvas will show the 
// difference between A and B
export const imgDrawOnCanvasIsAB = writable(false);
export const abHandlePos = writable(10);

// Video to be loaded or already loaded
// [A, B]
// If any is not null it is because that media 
// is loaded
export const mediaSlot = writable([null, null]);

// Canvas size [x, y]
export const canvasSize = writable([0, 0]);

// 0: non-cached
// 1: caching
// 2: cached
export const barFrameCacheStatusA = writable([]);
export const barFrameCacheStatusB = writable([]);

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
export const modalListOfFiles = writable("");       // List Of files in the current directory
export const modalListOfFilesError = writable("");  // Error if problem with current directory

export const internalViewwerSize = writable([]);

// If this is true then everytime the canvas changes size
// the media will be re-cache 
export const isCanvasAutoReload = writable(false);

// If this is true then the image to be load will be 
// 100% loaded to the canvas no matter the size of the window
export const isLoadFullImg = writable(false);

export const addrAndPort = writable("127.0.0.1:3000");