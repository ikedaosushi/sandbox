var PROTOTYPE_FIELDS_, _extend, isPlainObject;

isPlainObject = require("./isPlainObject");

PROTOTYPE_FIELDS_ = ['constructor', 'hasOwnProperty', 'isPrototypeOf', 'propertyIsEnumerable', 'toLocaleString', 'toString', 'valueOf'];

module.exports = _extend = function(target, var_args) {
  var deep, i, j, key, source;
  key = void 0;
  source = void 0;
  i = 1;
  if (typeof target === 'boolean') {
    deep = target;
    target = var_args;
  }
  while (i < arguments.length) {
    source = arguments[i];
    if (!target && source) {
      throw new TypeError;
    }
    for (key in source) {
      if (target === source[key]) {
        continue;
      }
      if (deep && isPlainObject(source[key])) {
        target[key] = _extend(deep, {}, target[key], source[key]);
      } else {
        target[key] = source[key];
      }
    }
    j = 0;
    while (j < PROTOTYPE_FIELDS_.length) {
      key = PROTOTYPE_FIELDS_[j];
      if (source !== void 0 && source !== null && Object.prototype.hasOwnProperty.call(source, key)) {
        target[key] = source[key];
      }
      j++;
    }
    i++;
  }
  return target;
};
