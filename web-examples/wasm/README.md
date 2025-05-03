An example showing how to use this crate with wasm-bindgen.
Uses some very silly hacks to work properly, and requires patching wasm-bindgen.

**This is not production-ready code and is more of a demo!**

wasm-bindgen patch:
```diff
diff --git a/crates/wasm-interpreter/src/lib.rs b/crates/wasm-interpreter/src/lib.rs
index a45772ad..85b24cb9 100644
--- a/crates/wasm-interpreter/src/lib.rs
+++ b/crates/wasm-interpreter/src/lib.rs
@@ -219,8 +219,23 @@ impl Interpreter {
         let func = module.funcs.get(id);
         log::debug!("starting a call of {:?} {:?}", id, func.name);
         log::debug!("arguments {:?}", args);
+
+        if func
+            .name
+            .as_ref()
+            .is_some_and(|n| n.starts_with("_ZN4") || n.starts_with("_GLOBAL_"))
+        {
+            log::debug!("nevermind");
+            self.scratch.push(0);
+            return None;
+        }
+
         let local = match &func.kind {
             walrus::FunctionKind::Local(l) => l,
+            walrus::FunctionKind::Import(i) => {
+                self.scratch.push(0);
+                return None;
+            }
             _ => panic!("can only call locally defined functions"),
         };
```