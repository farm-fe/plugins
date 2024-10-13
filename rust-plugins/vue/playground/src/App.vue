<script setup lang="ts">
import { ref } from 'vue'
import HelloWorld from './components/HelloWorld.vue'
const res = ref("Farm + Vue")
setTimeout(() => {
  res.value = "Farm + Vue + feature"
}, 1000);
</script>

<template>
  <div>
    <a href="https://farmfe.org/" target="_blank">
      <img src="./assets/logo.png" class="logo" alt="Farm logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <HelloWorld :msg="res" />
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #9F1A8Faa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
