import { createVNode as _createVNode, isVNode as _isVNode } from "vue";
function _isSlot(s) {
  return typeof s === "function" || ({}).toString.call(s) === "[object Object]" && !_isVNode(s);
}
const _a = function () {
  return a;
}();
import { defineComponent } from 'vue';
let a = 1;
const A = defineComponent({
  setup(_, {
    slots
  }) {
    return () => _createVNode("span", null, [slots.default()]);
  }
});
const _a2 = 2;
a = _a2;
a = _createVNode(A, null, _isSlot(a) ? a : {
  default: () => [_a],
  _: 2
});
