import { defineComponent } from 'vue'

interface T { bar: number }
interface S { nested: { foo: T['bar'] }}

defineComponent((props: S['nested']) => { })
