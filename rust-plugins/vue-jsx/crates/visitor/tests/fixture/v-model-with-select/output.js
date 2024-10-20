import {
  createTextVNode as _createTextVNode,
  createVNode as _createVNode,
  vModelSelect as _vModelSelect,
  withDirectives as _withDirectives,
} from "vue";
_withDirectives(_createVNode("select", {
  "onUpdate:modelValue": $event => test = $event
}, [_createVNode("option", {
  "value": "1"
}, [_createTextVNode("a")]), _createVNode("option", {
  "value": 2
}, [_createTextVNode("b")]), _createVNode("option", {
  "value": 3
}, [_createTextVNode("c")])], 8, ["onUpdate:modelValue"]), [[_vModelSelect, test]]);
