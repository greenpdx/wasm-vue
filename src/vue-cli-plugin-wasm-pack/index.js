import Vue from 'vue'
const VueWasm = import('../../pkg/index.js')
Vue.prototype.$wasm = {}

module.exports = api => {
    //nothing to do here
  };

VueWasm
    .then(wasm => {
        WasmPack.install = function(Vue, options) {
            Vue.prototype.$wasm = wasm
        }
    })
