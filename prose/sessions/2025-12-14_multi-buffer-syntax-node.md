Rust diagnostics too slow - justifies implementing diagnostic-stale indicators.


```
buffer.read(cx).snapshot();
buffer.update(cx, |buffer, cx| {
  let snapshot = buffer.snapshot(cx);
  ...
})
```

```
 pub fn resize_excerpt
```
