# `naga-native`

---

**TO BE HONEST, THIS IS A TERRIBLE, UNMAINTAINABLE MESS, DON'T USE THIS**

I suggest you write your own tiny ffi library that implements exactly what you need instead of implementing everything.
That is what I will be doing moving forward.

---

This project has nothing to do with `wgpu-native` nor `wgpu`.
This repository contains WIP C bindings to `naga`.

Check `naga.h` for what's unimplemented and supported.

## Design Questions

- `default()` value functions?
- Simpler error handling?
- Memory Management Strategy (arena allocator?)
- Individual slice type?
- How to prefix struct names?

