<template>
  <div id="app">
    <!-- img alt="Vue logo" src="./assets/logo.png" -->
    <HelloWorld msg="Welcome to Your Vue.js App"/>
  </div>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'

export default {
  name: 'app',
  components: {
    HelloWorld
  },
  data () {
    return {
      test: 42
    }
  },
  beforeCreate() {
    let wasm = this.$wasm
    let bt = wasm.tryit('beforeCreate')
    let tw = wasm.tryit(wasm)
  },
  created() {
    let wasm = this.$wasm
    console.log(wasm)
    wasm.wlog()
    let st = 'Hello'
    let bt = wasm.tryit(st)
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
