<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <div>
        <input type="range" v-model="rng" v-on:input="rngchg"/><br>
        <span>{{ rng  - 50 }}</span>
    </div>
    <div>
        <table id="nodeTbl" class="nodetbl">
            <tr v-for="node in nodes" v-bind:key = "node.idx">
                <td width="8%">{{ node.idx }}</td>
                <td width="15%">{{ vals[node.idx] }}</td>
                <td width="20%">{{ node.name }}</td>
                <td width="5%">{{ node.parnt }}</td>
                <td width="20%">{{ node.chld }}</td>
                <td width="20%"><div>
                    {{ node.leaf }}

                </div></td>
                <td width="10%">
                    <input type="checkbox" />
                    {{ node.lock }}</td>
                <td width="10%">{{ node.chld }}</td>
                
                
            </tr>
        </table>
    </div>
    
  </div>
</template>

<script>
//const budget = this.$wasm.budget

export default {
  name: 'Budget',
  props: {
    msg: String
  },
  data() {
      return {
        nodes: [],
        //filter: {}
        worker: null,
        rng: 50,
        vals: []
      }
  },
  precreate() {

  },
  created () {
    let wasm = this.$wasm

        let filt = {
      acode: -1,  // i32,     // -1 not used
      bcode: -1,  // i32,     // -1 not used
      ccode: -1,  // i32,     // -1 not used
      //bea: 2,  // Option<BEACat>,
      year: 20,
    }
    let bdgt = wasm.budget
   
    let ctx_ptr = wasm.init_app(bdgt);
    this.ctx_ptr = ctx_ptr

    //console.log(ctx_ptr); //<-- some number

    let nodes = wasm.jsnodes(ctx_ptr);
    this.nodes = nodes
    this.vals = wasm.getvals(ctx_ptr)
    console.log("Nodes ",nodes)

  },
  mounted() {
    console.log("MTD", this.$wasm.getvals(this.ctx_ptr))
    let wasm = this.$wasm
    //let rtn = wasm.wlog()
    //let bob = wasm.budget
    //let t = wasm.net3(bob, filt)
    //t.then( tr => { console.log(tr)})
  },
  beforeDestroy() {
    let wasm = this.$wasm
    wasm.free_nodes(this.ctx_ptr) 
  },
   methods: {
      jschgNode (idx, chg) {
          let wasm = this.$wasm
          let newvals = wasm.chg_node(idx, chg)
      },
      rngchg (evt) {
            let tgt = evt.target
            let val = tgt.value - 50
          this.vals = this.$wasm.chgval(this.ctx_ptr, 3, val )
      }
  }

}

</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.nodetbl {

}
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
