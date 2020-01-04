module.exports = function(obj) {
  if (!obj) {
    return false;
  }
  if (typeof obj !== "object") {
    return false;
  }
  if (obj.constructor && !Object.prototype.hasOwnProperty.call(obj.constructor.prototype, "isPrototypeOf")) {
    return false;
  }
  return true;
};
