function createCounter(start) {
  function inc() {
      start = start + 1
      return start
  }

  return inc
}

var inc1 = createCounter(0 - 5)
var z = inc1()
var inc2 = createCounter(200)


var x = inc1()
x = inc1()


var y = inc2()

console.log(x, y, z)