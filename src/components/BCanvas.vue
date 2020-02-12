<template>
  <div class="bcanvas">
    <canvas ref="canvas3d" id="canvas" class="canvas3d"></canvas>
  </div>
</template>

<script>

export default {
  name: 'BCanvas',
  components: {
  },
  props: {
    //msg: String
  },
  created() {
  },
  mounted() {
     let wasm = this.$wasm
    console.log(wasm)
    let rtn = wasm.start()
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.canvas3d {
  height: 300px;
  width: 300px;
}

</style>
