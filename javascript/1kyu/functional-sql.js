const id = x => x;
const just = x => id.bind(null, x);
const or = (f, g) => x => f(x) || g(x);
const and = (f, g) => x => f(x) && g(x);
const groupBy = (xs, fs) => {
  if (fs.length === 0) {
    return xs;
  }
  const f = fs[0];
  const groups = new Map();
  for (const x of xs) {
    const key = f(x);
    let group = groups.get(key);
    if (group === undefined) {
      group = [];
      groups.set(key, group);
    }
    group.push(x);
  }
  return [...groups].map(([key, group]) => [key, groupBy(group, fs.slice(1))]);
};

function query() {
  let dataSource = [];
  let filter = just(true);
  let projector = id;
  let discrimators = [];
  let groupFilter = just(true);
  const comparators = [];

  const builder = {
    select(f) {
      if (typeof f === "function") {
        projector = f;
      }
      return this;
    },
    from(...tables) {
      dataSource = tables.reduce((xs, ys) =>
        [].concat(
          ...xs.map(x => ys.map(y => (Array.isArray(x) ? [...x, y] : [x, y])))
        )
      );
      return this;
    },
    where(...predicates) {
      filter = and(filter, predicates.reduce(or));
      return this;
    },
    orderBy(comparator) {
      comparators.push(comparator);
      return this;
    },
    groupBy(...fs) {
      discrimators.push(...fs);
      return this;
    },
    having(...predicates) {
      if (discrimators.length === 0) {
        throw new Error("`groupBy` should be called before `having`");
      }
      groupFilter = and(groupFilter, predicates.reduce(or));
      return this;
    },
    execute() {
      let result = dataSource.filter(filter);
      if (discrimators.length > 0) {
        result = groupBy(result, discrimators).filter(groupFilter);
      }
      if (comparators.length > 0) {
        result.sort((a, b) =>
          comparators.reduce((x, f) => (x === 0 ? f(a, b) : x), 0)
        );
      }
      return result.map(projector);
    }
  };

  // If you repeat a SQL clause (except `where()` or `having()`), an exception will be thrown.
  ["select", "from", "orderBy", "groupBy", "execute"].forEach(key => {
    const impl = builder[key];
    let called = false;
    builder[key] = function() {
      if (called) {
        throw new Error(`Duplicate ${key.toUpperCase()}`);
      }
      called = true;
      return impl.apply(this, arguments);
    };
  });

  return builder;
}

module.exports = { query };
