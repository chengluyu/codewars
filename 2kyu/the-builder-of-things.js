class GlobalVariable {
  constructor(name) {
    this.name = name;
  }

  save() {
    this._hasPreviousValue = this.name in global;
    this._previousValue = global[this.name];
  }

  set(value) {
    global[this.name] = value;
  }

  restore() {
    if (this._hasPreviousValue) {
      global[this.name] = this._previousValue;
    } else {
      delete global[this.name];
    }
  }
}

class ThingArray extends Array {
  constructor(name, count) {
    super();
    const singularName = name.endsWith('s') && name.length > 1 ?
      name.substr(0, name.length - 1) : name;
    for (let i = 0; i < count; i++) {
      this.push(new Thing(singularName));
    }
  }

  each(fn) {
    const having = new GlobalVariable('having');
    const being_the = new GlobalVariable('being_the');
    having.save();
    being_the.save();
    this.forEach(thing => {
      having.set(thing.has.bind(thing));
      being_the.set(thing.is_the);
      fn(thing);
    });
    having.restore();
    being_the.restore();
  }
}

class _Thing {
  constructor(name) {
    this.name = name;

    const traits = new Map();

    const is_a = {};

    this.is_a = new Proxy(this, {
      get(obj, prop) {
        if (!traits.has(prop)) {
          traits.set(prop, true);
          obj['is_a_' + prop] = true;
          obj['is_not_a_' + prop] = false;
        }
        return traits.get(prop);
      }
    });

    const is_not_a = {};

    this.is_not_a = new Proxy(this, {
      get(obj, prop) {
        if (!traits.has(prop)) {
          traits.set(prop, false);
          obj['is_a_' + prop] = false;
          obj['is_not_a_' + prop] = true;
        }
        return traits.get(prop);
      }
    });

    const definition = new Map();
    this.is_the = new Proxy(this, {
      get(obj, key) {
        if (!definition.has(key)) {
          definition.set(key, new Proxy(obj, {
            get(target, value) {
              target[key] = value;
              return obj;
            }
          }))
        }
        return definition.get(key);
      }
    });

    this.and_the = this.being_the = this.is_the;

    const singleItems = new Map();

    this._singleItems = new Proxy(this, {
      get(self, item) {
        if (!singleItems.has(item)) {
          singleItems.set(item, self[item] = new Thing(item));
        }
        return singleItems.get(item);
      }
    })

    this._multipleItems = new Map();

    /**
     * The ability this thing able to do.
     * Defined by `can` property.
     * @type {Map<String, Function>}
     */
    const abilities = new Map();

    this.can = new Proxy(this, {
      get(self, ability) {
        // if this ability does not exist, create one
        if (!abilities.has(ability)) {
          abilities.set(ability, (pastTense, handler) => {
            if (typeof pastTense === 'function') {
              handler = pastTense;
              pastTense = null;
            }
            // add hook on handler:
            if (typeof pastTense === 'string') {
              // we need to do records if past tense is provided
              const log = self[pastTense] = [];
              self[ability] = function () {
                const name = new GlobalVariable('name');
                name.save();
                name.set(self.name);
                const result = handler.apply(this, arguments);
                log.push(result);
                name.restore();
                return result;
              }
            } else {
              self[ability] = function () {
                const name = new GlobalVariable('name');
                name.save();
                name.set(self.name);
                const result = handler.apply(this, arguments);
                name.restore();
                return result;
              }
            }
          });
        }
        // return the ability
        return abilities.get(ability);
      }
    })
  }

  has(n) {
    if (typeof n !== 'number' || Math.floor(n) !== n || n < 1) {
      throw new Error('n must be an integer which is greater than one');
    }
    if (n === 1) {
      return this._singleItems;
    } else {
      if (!this._multipleItems.has(n)) {
        this._multipleItems.set(n, new Proxy(this, {
          get(self, itemName) {
            return self[itemName] = new ThingArray(itemName, n);
          }
        }));
      }
      return this._multipleItems.get(n);
    }
  }

  having(n) {
    return this.has(n);
  }

  with(n) {
    return this.has(n);
  }
}

function Thing(name) {
  const realThing = new _Thing(name);
  const proxyThing = new Proxy(realThing, {
    get(target, key) {
      if (key === 'has') {
        return _Thing.prototype.has.bind(realThing);
      }
      if (key === 'having') {
        return _Thing.prototype.has.bind(realThing);
      }
      if (key === 'with') {
        return _Thing.prototype.has.bind(realThing);
      }
      return realThing[key];
    },
    ownKeys(target, key) {
      return [];
    }
  });
  proxyThing.__proto__ = Thing.prototype;
  return proxyThing;
}
