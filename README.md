# Sync-channel Error

This repo is to show how the sync-channel cannot wait for a value to be sent through.

To test, do the following:

```
wasm-pack build
cd www
npm i
npm run start
```

Then open http://localhost:8080, open the javascript console, and you'll see:

```
sync_channel_bg.js:215 panicked at 'condvar wait not supported', library/std/src/sys/wasm/../unsupported/condvar.rs:21:9
[...]
sync_channel_bg.js:215 panicked at 'cannot recursively acquire mutex', library/std/src/sys/wasm/../unsupported/mutex.rs:22:9
[...]
```
