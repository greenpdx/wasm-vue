import Vue from 'vue'
import App from './App.vue'
//import './registerServiceWorker'
/*global globalThis*/
const WasmPack = import('../pkg/index.js')
Vue.prototype.$wasm = {}
Vue.prototype.$budget = {}

Vue.config.productionTip = false
//Vue.use(WasmPack)
WasmPack
  .then(m => {
    Vue.prototype.$wasm = m;
    let t = m.net2("./test-small.csv")
      .then( data => {
        Vue.prototype.$wasm.budget = data
        console.log("MAIN",data)
        new Vue({
          render: h => h(App),
            }).$mount('#app')
          })
      })
 