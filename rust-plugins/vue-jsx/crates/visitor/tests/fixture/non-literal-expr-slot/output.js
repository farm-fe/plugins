import { createVNode as _createVNode, isVNode as _isVNode, resolveComponent as _resolveComponent } from "vue";
function _isSlot(s) {
  return typeof s === "function" || ({}).toString.call(s) === "[object Object]" && !_isVNode(s);
}
let _slot;
const foo = () => 1;
_createVNode(_resolveComponent("A"), null, _isSlot(_slot = foo()) ? _slot : {
  default: () => [_slot],
  _: 1
});
