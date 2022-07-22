import { writable } from 'svelte/store';

class RawImageFrames {
    constructor() {
        this.imgs = [];
        this.order = [];
        this.paths = [];
        this.progress = writable([])
    }

    clearAll() {
        this.imgs = [];
        this.order = [];
        this.paths = [];
        this.progress.set([]);
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

}

// 0: non-cached
// 1: caching
// 2: cached
export const raw_images_a = new RawImageFrames();
export const raw_images_b = new RawImageFrames();