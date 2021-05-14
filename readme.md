This is a mini demo of webassembly via rust. We're checking which language is faster at counting.

You can use the `Makefile` to install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) if you haven't allready. Once that is done you can compile everything with;

```
make compile
```

This will generate the webassembly/js bindings for you. You can refer to it via the provided `index.html`. You can serve it via; 

```
python3 -m http.server
```

The console in the browser should confirm that the webassembly implementation outperforms the js one.