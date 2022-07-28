import { writable, get } from 'svelte/store';
import { onDestroy } from 'svelte';

class RawImageFrames {
    constructor() {
        this.imgs = [];     // Image raw pixels and image size.
        this.order = [];    // Order in which images are saved in cache. (TODO: re-write code to get rid of this)
        this.paths = [];    // List of paths of each image.
        this.workers = [];  // List of webworkers for each frame. Useful to terminate when needed.
        this.progress = writable([]);   // List with the progress bar info.
    }

    clearAll() {
        this.imgs = [];
        this.order = [];
        this.paths = [];
        this.progress.set([]);

        // Terminate any worker that is currently caching.
        for (let w in this.workers) {
            if (this.workers[w] !== undefined) {
                this.workers[w].terminate();
            }
        }

        this.workers = [];
    }

    pushToProgress(status) {
        this.progress.update( p => {
            return([...p, status]);
        } );
    }

    setStatusAtFrame(status, frame) {
        this.progress.update( p => {
            p[frame] = status;
            return(p);
        });
    }

    isPreCached(){
        let r = false;

        if (this.paths.length > 0) r = true;

        return(r);
    }

}

// 0: non-cached
// 1: caching
// 2: cached
// 3: error while caching
export const raw_images_a = new RawImageFrames();
export const raw_images_b = new RawImageFrames();