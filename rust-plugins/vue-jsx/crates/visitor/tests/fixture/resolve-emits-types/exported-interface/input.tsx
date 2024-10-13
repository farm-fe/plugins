import { defineComponent, type SetupContext } from 'vue'

export interface Emits { (e: 'foo' | 'bar'): void }

defineComponent((_, ctx: SetupContext<Emits>) => {})
