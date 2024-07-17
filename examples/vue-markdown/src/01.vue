<template>
  <div class="markdown-body">
    <h1>Hello world!</h1>
    <p>Front matter: {{ frontmatter }}</p>

    <Counter /><br />
    <Counter2 />
    <router-link to="/">Home</router-link>
  </div>
</template>
<script setup>
import { useHead } from '@unhead/vue';
const frontmatter = {
  title: 'Hello',
  meta: [
    { name: 'description', content: 'Hello World' },
    { property: 'og:title', content: 'Hello' },
    { name: 'twitter:title', content: 'Hello' },
  ],
  test: 'test',
};
defineExpose({ frontmatter });
const head = {
  title: 'Hello',
  meta: [
    { name: 'description', content: 'Hello World' },
    { property: 'og:title', content: 'Hello' },
    { name: 'twitter:title', content: 'Hello' },
  ],
};
useHead(head);
import Counter2 from '../Counter2.vue';
</script>
<script>
export const title = 'Hello';
export const meta = [
  { name: 'description', content: 'Hello World' },
  { property: 'og:title', content: 'Hello' },
  { name: 'twitter:title', content: 'Hello' },
];
export const test = 'test';
</script>
<route>
{
  meta: {
    layout: 'home'
  }
}
</route>
<style>
h1 {
  color: cadetblue;
}
</style>
code
<template><div class="markdown-body"></div></template>
<script setup>
import { useHead } from '@unhead/vue';
const frontmatter = { meta: [] };
defineExpose({ frontmatter });
const head = { meta: [] };
useHead(head);
</script>
<script>
export const meta = [];
</script>
