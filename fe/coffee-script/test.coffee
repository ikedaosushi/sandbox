extend = require("./extend")

data = {}
event = {
  a: 1, b: 2, pageX: 400, pageY: 300
}

[tagName, tagId, tagClass, tagParents, tagText, tagAttr] = ['tn', 'ti', 'tc', 'tp', 'tt', 'ta']

{pageX, pageY} = event
if pageX && pageY
  data = extend data, {pageX, pageY}

# console.log(data)

foo = 1
if foo in [2, 3]
  console.log 'hello'