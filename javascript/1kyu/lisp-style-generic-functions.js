// This Kata spent me nearly 5 hours, according to Toggl.

// medium.com/fhinkel/v8-internals-how-small-is-a-small-integer-e0badc18b6da
const V8_SMI_MAX = 1 << (31 - 1);
const TYPE_MISMATCH = -1;

class OverloadList {
  constructor() {
    this.overloads = [];
    this.typesIndexMap = new Map();
  }

  add(overload) {
    const index = this.typesIndexMap.get(overload.types);
    if (index === undefined) {
      this.typesIndexMap.set(overload.types, this.overloads.length);
      this.overloads.push(overload);
    } else {
      this.overloads[index] = overload;
    }
  }

  remove(overloadTypes) {
    const index = this.typesIndexMap.get(overloadTypes);
    if (index !== undefined) {
      this.typesIndexMap.delete(overloadTypes);
      this.overloads.splice(index, 1);
    }
  }

  getCandidates(args, reverse = false) {
    const factor = reverse ? -1 : 1;
    return this.overloads
      .map(x => ({
        overload: x,
        specificity: x.guard.apply(null, args)
      }))
      .filter(x => x.specificity !== null)
      .sort(
        (a, b) => factor * compareSpecificity(a.specificity, b.specificity)
      );
  }
}

function getPrototypeChain(any) {
  const chain = [];
  let proto = any;
  do {
    proto = Object.getPrototypeOf(proto);
    chain.push(proto.constructor.name);
  } while (proto.constructor.name !== "Object");
  return chain;
}

function compareSpecificity(a, b) {
  if (a.length !== b.length) {
    throw new Error("specificity arrays must be of the same length");
  }
  const length = a.length;
  for (let i = 0; i < length; i++) {
    if (a[i] !== b[i]) {
      return a[i] - b[i];
    }
  }
  return 0;
}

function callNextMethod(context, ...args) {
  if (context.primary) {
    // The first time of execute the primary.
    if (context.index === 0) {
      context.index += 1;
      // Before
      for (const c of context.beforeCandidates) {
        c.overload.implementation.apply(null, args);
      }
      // Primary
      const ret = context.primaryCandidates[0].overload.implementation.apply(
        context,
        args
      );
      // After
      for (const c of context.afterCandidates) {
        c.overload.implementation.apply(null, args);
      }
      return ret;
    } else if (context.index < context.primaryCandidates.length) {
      return context.primaryCandidates[
        context.index++
      ].overload.implementation.apply(context, args);
    } else {
      throw `No next method found for ${context.name} in primary`;
    }
  } else if (context.index < context.aroundCandidates.length) {
    const index = context.index;
    // Advance the index.
    context.index += 1;
    // Switch to `primary` if the around candidates are run out.
    if (
      context.index === context.aroundCandidates.length &&
      context.primaryCandidates.length > 0
    ) {
      context.primary = true;
      context.index = 0;
    }
    return context.aroundCandidates[index].overload.implementation.apply(
      context,
      arguments
    );
  } else {
    throw `No next method found for ${context.name} in around`;
  }
}

function defgeneric(name) {
  let version = 0;
  const arounds = new OverloadList();
  const befores = new OverloadList();
  const overloads = new OverloadList();
  const afters = new OverloadList();

  function getTypeName(any) {
    let type = typeof any;
    if (type === "object") {
      if (any === null) {
        type = "null";
      } else if (any.constructor.name !== "Object") {
        type = any.constructor.name;
      }
    }
    return type;
  }

  function run(context, args) {
    if (context.aroundCandidates.length > 0) {
      context.primary = false;
      return callNextMethod(context, ...args);
    } else if (context.primaryCandidates.length > 0) {
      context.primary = true;
      return callNextMethod(context, ...args);
    } else {
      const argumentTypes = args.map(getTypeName).join(",");
      throw `No method found for ${name} with args: ${argumentTypes}`;
    }
  }

  function func(...args) {
    return run(
      {
        name,
        aroundCandidates: arounds.getCandidates(args),
        primaryCandidates: overloads.getCandidates(args),
        beforeCandidates: befores.getCandidates(args),
        afterCandidates: afters.getCandidates(args, true),
        primary: null,
        index: 0
      },
      args
    );
  }

  const findMethodCache = new Map();

  function findMethod(...args) {
    const aroundCandidates = arounds.getCandidates(args);
    const primaryCandidates = overloads.getCandidates(args);
    const beforeCandidates = befores.getCandidates(args);
    const afterCandidates = afters.getCandidates(args, true);
    const mostSpecificCandidate =
      aroundCandidates.length > 0
        ? aroundCandidates[0]
        : primaryCandidates.length > 0
        ? primaryCandidates[0]
        : null;
    if (mostSpecificCandidate === null) {
      const argumentTypes = [...arguments].map(getTypeName).join(",");
      throw `No method found for ${name} with args: ${argumentTypes}`;
    }

    // Make a key from current version and types of the most specific candidate.
    const key = `${version}_${mostSpecificCandidate.overload.types}`;
    let item = findMethodCache.get(key);

    // If no cached function, make one.
    if (item === undefined) {
      item = (...args) =>
        run(
          {
            name,
            aroundCandidates,
            primaryCandidates,
            beforeCandidates,
            afterCandidates,
            primary: null,
            index: 0
          },
          args
        );

      findMethodCache.set(key, item);
    }

    // Return the cached function.
    return item;
  }

  /**
   * Translate the argument type string to a guard function.
   * @param {string} types a string describe types of arguments
   * @returns {(...args: any[]) => number[] | null}
   */
  function parseGuard(types) {
    const guards = types.split(/\s*,\s*/g).map(type => {
      switch (type) {
        case "*":
          return () => V8_SMI_MAX;
        case "undefined":
          return x => (x === undefined ? 0 : TYPE_MISMATCH);
        case "null":
          return x => (x === null ? 0 : TYPE_MISMATCH);
        case "boolean":
          return x => (typeof x === "boolean" ? 0 : TYPE_MISMATCH);
        case "number":
          return x => (typeof x === "number" ? 0 : TYPE_MISMATCH);
        case "string":
          return x => (typeof x === "string" ? 0 : TYPE_MISMATCH);
        case "object":
          return x => {
            if (x === null) {
              return TYPE_MISMATCH;
            }
            return getPrototypeChain(x).indexOf("Object");
          };
        case "function":
          return x => (typeof x === "function" ? 0 : TYPE_MISMATCH);
        default:
          return x => getPrototypeChain(x).indexOf(type);
      }
    });
    return function() {
      if (arguments.length !== guards.length) {
        return null;
      }
      const specificity = guards.map((g, i) => g(arguments[i]));
      return specificity.indexOf(TYPE_MISMATCH) >= 0 ? null : specificity;
    };
  }

  func.defmethod = function(types, implementation, combination = "primary") {
    version += 1;
    const overload = {
      types,
      guard: parseGuard(types),
      implementation,
      combination
    };
    if (combination === "before") {
      befores.add(overload);
    } else if (combination === "primary") {
      overloads.add(overload);
    } else if (combination === "after") {
      afters.add(overload);
    } else if (combination === "around") {
      arounds.add(overload);
    } else {
      throw new Error(`unknown combination "${combination}"`);
    }
    return func;
  };

  func.removeMethod = function(types, combination = "primary") {
    version += 1;
    if (combination === "before") {
      befores.remove(types);
    } else if (combination === "primary") {
      overloads.remove(types);
    } else if (combination === "after") {
      afters.remove(types);
    } else if (combination === "around") {
      arounds.remove(types);
    } else {
      throw new Error(`unknown combination "${combination}"`);
    }
    return func;
  };

  func.findMethod = findMethod;

  return func;
}

module.exports = {
  getPrototypeChain,
  compareSpecificity,
  callNextMethod,
  defgeneric
};
