import { Fragment as _Fragment, createVNode as _createVNode, vModelRadio as _vModelRadio, withDirectives as _withDirectives } from "vue";
_createVNode(_Fragment, null, [_withDirectives(_createVNode("input", {
    "type": "radio",
    "value": "1",
    "onUpdate:modelValue": $event => test = $event,
    "name": "test"
}, null, 8, ["onUpdate:modelValue"]), [[_vModelRadio, test]]), _withDirectives(_createVNode("input", {
    "type": "radio",
    "value": "2",
    "onUpdate:modelValue": $event => test = $event,
    "name": "test"
}, null, 8, ["onUpdate:modelValue"]), [[_vModelRadio, test]])]);
