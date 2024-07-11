use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

const BACKGROUND_ID: &str = "background";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = tsParticles, js_name = load)]
    fn ts_particles(dom_id: &str, configuration: JsValue);
}

pub struct Background;

impl Background {
    pub fn get_particles_config() -> JsValue {
        // Reference: https://particles.js.org/docs/interfaces/tsParticles_Engine.Options_Interfaces_Particles_IParticlesOptions.IParticlesOptions.html
        let config_str = include_str!("particles_config.json");

        js_sys::JSON::parse(config_str).unwrap()
    }

    pub fn render() {
        use_hook(|| ts_particles(BACKGROUND_ID, Self::get_particles_config()));
    }
}
