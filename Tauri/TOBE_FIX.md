# TODO
- Make sure that the entire video load. This is not possible sometimes due to "promise time out"
- Media B should match Media A's resolution so it can be compare. Re-scale media B to match A.
- Show notifications about errors. Maybe this repo can help you https://github.com/keenethics/svelte-notifications

# List of bugs or things that need to be fixed.

- the media player's bar needs to be fixed.
- [BUG] In the `LoadFileModal` module, when a file has a seq format but only has a length of one. It shows different name. Needs to be fixed.

## Fixed / DONE
- ~~[BUG] when an image with length of one is to be loaded, does't show up.~~
- ~~[BUG] when player is in Diff mode, it shows the "normal" image first - Needs to be fixed.~~
- ~~[BUG] When "diff" button is clicked while the video has not loaded entirely, the program stops working properly. (it might happens with "A/B" as well... check that)~~