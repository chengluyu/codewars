// Born to be chained
// 4kyu
function chain(fns, initial) {
  let rings = {
    execute () { return initial }
  }

  Object.keys(fns).forEach(key => {
    rings[key] = function () {
      let first = this.execute()
      let args = first ? [ first, ...arguments ] : [...arguments]
      let result = fns[key].apply(this, args)
      return chain(fns, result)
    }
  })

  return rings
}