<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <bcanvas></bcanvas>
    <budget></budget>
  </div>
</template>

<script>
import Budget from './Budget'
import BCanvas from './BCanvas'

export default {
  name: 'HelloWorld',
  components: {
    budget: Budget,
    bcanvas: BCanvas
  },
  props: {
    msg: String
  },
  mounted() {
    let filt = {
      acode: -1,  // i32,     // -1 not used
      bcode: -1,  // i32,     // -1 not used
      ccode: -1,  // i32,     // -1 not used
      //bea: 2,  // Option<BEACat>,
      year: 20,
    }
    let wasm = this.$wasm
    //let rtn = wasm.wlog()
    let bob = wasm.budget
    //console.log(bob)
    //let t = wasm.net3(bob, filt)
    //t.then( tr => { console.log(tr)})
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
