<template>
  <div>
    <h1>{{ count }}</h1>
    <button @click="increment">Increment</button>
    <button @click="decrement">Decrement</button>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  setup() {
    const count = ref(0);

    const increment = () => {
      count.value++;
    };

    const decrement = () => {
      count.value--;
    };

    return {
      count,
      increment,
      decrement
    };
  }
};
</script>

<style scoped>
h1 {
  font-size: 24px;
  margin-bottom: 16px;
}

button {
  padding: 8px 16px;
  margin-right: 8px;
}
</style>
