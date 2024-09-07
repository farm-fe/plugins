import { a, a as A, type b, type c } from "modules/a"
import * as StarB from "modules/b"
import "@/xxx"
import K from "@/k"
export * from "./test"

export { r  } from "./x/test"
export {
  a as P,
  A as Q,
  b as B,
  c,
  StarB
}
export const m = "a"

// export default function () {
//   return "a"
// }

export function m2() {
  return "a"
}

export const m3 = function () { }

export var m4 = function () { }

export class m5 {
  a = "a"
}

// export default class m6 {
//   a = "a"
// }

export class m7 {
  a = "a"
  static b = "b"
}

export default m7
