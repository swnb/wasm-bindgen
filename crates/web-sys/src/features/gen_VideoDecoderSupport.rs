#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoDecoderSupport)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoDecoderSupport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoDecoderSupport;
    #[cfg(feature = "VideoDecoderConfig")]
    #[wasm_bindgen(method, setter = "config")]
    fn config_shim(this: &VideoDecoderSupport, val: &VideoDecoderConfig);
    #[wasm_bindgen(method, setter = "supported")]
    fn supported_shim(this: &VideoDecoderSupport, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl VideoDecoderSupport {
    #[doc = "Construct a new `VideoDecoderSupport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Change the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`, `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn config(&mut self, val: &VideoDecoderConfig) -> &mut Self {
        self.config_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn supported(&mut self, val: bool) -> &mut Self {
        self.supported_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoDecoderSupport {
    fn default() -> Self {
        Self::new()
    }
}
