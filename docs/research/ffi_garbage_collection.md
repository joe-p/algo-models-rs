# FFI Garbage Collection

One of the challenges with using FFIs in some languages is that they do not offer a way to integrate into the garbage collector. If consumers of the FFI interfaces are not aware of this, they could potentially expose themselves to memory leaks. This document is an overview of the state of garbage collection integration across various languages and if/how our chosen tools use them. If a language or tool is not listed, it should be assumed that the state of garbage collection integration is unknown.

Because GC-integration varies significantly between languages, it seems like the best approach for a safe and consistent FFI across multiple languages is to limit the amount of Rust-owned objects that shared over the FFI boundary.

## Summary

This table indicates whether the specific language can safely use Rust-owned objects through the bindings via GC-integration

| Language              | Status |
| --------------------- | ------ |
| JavaScript (Web/Node) | ✅     |
| Python                | ✅     |
| Swift                 | ❓     |
| React Native          | ❌     |
| Kotlin                | ❌     |
| Go                    | ❌     |

## Languages

### JavaScript (Web/Node)

The [WeakRefs TC39 proposal](https://github.com/tc39/proposal-weakrefs) enables objects to integrate with the JavaScript engine's garbage collection via `WeakRefFinalizationRegistry`. This is currently supported by [95% of browsers](https://caniuse.com/mdn-javascript_builtins_weakref).

The following note, however, should be taken into consideration if we were to rely on this behavior ([source](https://github.com/tc39/proposal-weakrefs?tab=readme-ov-file#a-note-of-caution))

> Garbage collectors are complicated. If an application or library depends on GC cleaning up a WeakRef or calling a finalizer in a timely, predictable manner, it's likely to be disappointed: the cleanup may happen much later than expected, or not at all. Sources of variability include:
>
> - One object might be garbage-collected much sooner than another object, even if they become unreachable at the same time, e.g., due to generational collection.
> - Garbage collection work can be split up over time using incremental and concurrent techniques.
> - Various runtime heuristics can be used to balance memory usage, responsiveness.
> - The JavaScript engine may hold references to things which look like they are unreachable (e.g., in closures, or inline caches).
> - Different JavaScript engines may do these things differently, or the same engine may change its algorithms across versions.
> - Complex factors may lead to objects being held alive for unexpected amounts of time, such as use with certain APIs.

`wasm-bindgen` makes use of `WeakRefFinalizationRegistry` [by default](https://rustwasm.github.io/wasm-bindgen/reference/weak-references.html)
Source:

### React Native

The React Native UniFFI bindings do not currently make use of `FinalizationRegistry` due to lack of support in Hermes ([source](<(https://jhugman.github.io/uniffi-bindgen-react-native/idioms/gc.html)>))

### Python

`__del__` is the destructor for a given object that is called upon garbage collection. UniFFI bindings make use of `__del__` for objects with Rust-owned memory.

### Kotlin

Kotlin provides various ways to handle object lifecycles, but it is not possible to integrate directly into the GC. Various strategies for using foreign objects generated by UniFFI bindings can be found [here](https://mozilla.github.io/application-services/book/howtos/uniffi-object-destruction-on-kotlin.html).

### Go

Go has [runtime.SetFinalizer](https://pkg.go.dev/runtime#SetFinalizer) but it is not a perfect solution, especially for objects in memory. From the documentation:

> The finalizer is scheduled to run at some arbitrary time after the program can no longer reach the object to which obj points. There is no guarantee that finalizers will run before a program exits, so typically they are useful only for releasing non-memory resources associated with an object during a long-running program.

This means we should likely not rely on `SetFinalizer` for garbage collecting Rust-owned objects. Or, at the very least, more extensive testing will be required if we were to use `SetFinalizer`

### Swift

Swift is not a GC language, but uses reference-counted pointers. All objects going over the FFI via UniFFI are wrapped in a Rust ARC (even if not explicit, it is done automatically by the binding generator). This presumably means that the reference count is shared between Swift and Rust