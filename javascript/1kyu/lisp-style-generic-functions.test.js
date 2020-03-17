const {
  getPrototypeChain,
  defgeneric,
  callNextMethod,
  compareSpecificity
} = require("./lisp-style-generic-functions");

class Mammal {}
class Platypus extends Mammal {}
class Rhino extends Mammal {}

describe("getPrototypeChain", () => {
  it("Intrinsic types", () => {
    expect(getPrototypeChain(0)).toEqual(["Number", "Object"]);
    expect(getPrototypeChain("test")).toEqual(["String", "Object"]);
    expect(getPrototypeChain(true)).toEqual(["Boolean", "Object"]);
    expect(getPrototypeChain(function() {})).toEqual(["Function", "Object"]);
  });
  it("User-defined classes", () => {
    expect(getPrototypeChain(new Mammal())).toEqual(["Mammal", "Object"]);
    expect(getPrototypeChain(new Rhino())).toEqual([
      "Rhino",
      "Mammal",
      "Object"
    ]);
  });
});

describe("compareSpecificity", () => {
  it("empty arrays", () => {
    expect(compareSpecificity([], [])).toEqual(0);
  });
  it("same arrays", () => {
    expect(compareSpecificity([0], [0])).toEqual(0);
    expect(compareSpecificity([0, 8], [0, 8])).toEqual(0);
    expect(compareSpecificity([0, 8, 16], [0, 8, 16])).toEqual(0);
  });
  it("different arrays", () => {
    expect(compareSpecificity([0], [1])).toEqual(-1);
    expect(compareSpecificity([0, 8], [0, 7])).toEqual(1);
    expect(compareSpecificity([0, 8, 16], [0, 0, 16])).toEqual(8);
  });
  it("sort", () => {
    const a = [1, 2, 3];
    const b = [1, 1, 0];
    const c = [2, 3, 4];
    const d = [0, 8, 2];
    expect([a, b, c, d].sort(compareSpecificity)).toEqual([d, b, a, c]);
  });
});

describe("defgeneric", () => {
  it("example 1", () => {
    var append = defgeneric("append");
    append.defmethod("Array,Array", function(a, b) {
      return a.concat(b);
    });
    append.defmethod("*,Array", function(a, b) {
      return [a].concat(b);
    });
    append.defmethod("Array,*", function(a, b) {
      return a.concat([b]);
    });

    expect(append([1, 2], [3, 4])).toEqual([1, 2, 3, 4]);
    expect(append(1, [2, 3, 4])).toEqual([1, 2, 3, 4]);
    expect(append([1, 2, 3], 4)).toEqual([1, 2, 3, 4]);
    expect(append.bind(null, 1, 2, 3, 4)).toThrow(
      "No method found for append with args: number,number,number,number"
    );
  });

  it("example 2", () => {
    var laysEggs = defgeneric("laysEggs")
      .defmethod("Mammal", function() {
        return false;
      })
      .defmethod("Platypus", function() {
        return true;
      });
    expect(laysEggs(new Rhino())).toBe(false);
    expect(laysEggs(new Platypus())).toBe(true);
    expect(laysEggs.bind(null, 5)).toThrow(
      "No method found for laysEggs with args: number"
    );
  });

  it("before & after", () => {
    let lines = [];
    var laysEggs = defgeneric("laysEggs")
      .defmethod("Mammal", function() {
        return false;
      })
      .defmethod("Platypus", function() {
        return true;
      })
      .defmethod(
        "Platypus",
        function() {
          lines.push("Before platypus egg check.");
        },
        "before"
      )
      .defmethod(
        "Mammal",
        function() {
          lines.push("Before mammal egg check.");
        },
        "before"
      )
      .defmethod(
        "*",
        function() {
          lines.push("Before egg check.");
        },
        "before"
      )
      .defmethod(
        "Platypus",
        function() {
          lines.push("After platypus egg check.");
        },
        "after"
      )
      .defmethod(
        "Mammal",
        function() {
          lines.push("After mammal egg check.");
        },
        "after"
      );

    expect(laysEggs(new Platypus())).toEqual(true);
    expect(lines).toEqual([
      "Before platypus egg check.",
      "Before mammal egg check.",
      "Before egg check.",
      "After mammal egg check.",
      "After platypus egg check."
    ]);
  });

  it("callNextMethod", () => {
    var describe = defgeneric("describe")
      .defmethod("Mammal", function() {
        return "Warm-blooded animal with large four-chambered heart.";
      })
      .defmethod("Platypus", function(p) {
        return callNextMethod(this, p) + " [Aquatic]";
      });
    expect(describe(new Platypus())).toEqual(
      "Warm-blooded animal with large four-chambered heart. [Aquatic]"
    );
  });

  it("callNextMethod & around", () => {
    const logs = [];
    var laysEggs = defgeneric("laysEggs")
      .defmethod("Mammal", function() {
        return false;
      })
      .defmethod("Platypus", function() {
        return true;
      });
    laysEggs
      .defmethod(
        "Platypus",
        function() {
          logs.push("Before platypus egg check.");
        },
        "before"
      )
      .defmethod(
        "Mammal",
        function() {
          logs.push("Before mammal egg check.");
        },
        "before"
      )
      .defmethod(
        "*",
        function() {
          logs.push("Before egg check.");
        },
        "before"
      )
      .defmethod(
        "Platypus",
        function() {
          logs.push("After platypus egg check.");
        },
        "after"
      )
      .defmethod(
        "Mammal",
        function() {
          logs.push("After mammal egg check.");
        },
        "after"
      );
    laysEggs.defmethod(
      "Platypus",
      function(p) {
        logs.push(">>>Around platypus check.");
        var ret = callNextMethod(this, p);
        logs.push("<<<Around platypus check.");
        return ret;
      },
      "around"
    );
    laysEggs.defmethod(
      "Mammal",
      function(p) {
        logs.push(">>>Around mammal check.");
        var ret = callNextMethod(this, p);
        logs.push("<<<Around mammal check.");
        return ret;
      },
      "around"
    );
    expect(laysEggs(new Platypus())).toBe(true);
    expect(logs).toEqual([
      ">>>Around platypus check.",
      ">>>Around mammal check.",
      "Before platypus egg check.",
      "Before mammal egg check.",
      "Before egg check.",
      "After mammal egg check.",
      "After platypus egg check.",
      "<<<Around mammal check.",
      "<<<Around platypus check."
    ]);
  });

  it("findMethod", () => {
    var append = defgeneric("append");
    append.defmethod("Array,Array", function(a, b) {
      return a.concat(b);
    });
    append.defmethod("*,Array", function(a, b) {
      return [a].concat(b);
    });
    append.defmethod("Array,*", function(a, b) {
      return a.concat([b]);
    });

    var appendLists = append.findMethod([], []);
    expect(appendLists([1, 2], [3, 4])).toEqual(append([1, 2], [3, 4]));
  });

  it("findMethod with same type will return the same function", () => {
    var describe = defgeneric("describe")
      .defmethod("Mammal", function() {
        return "Warm-blooded animal with large four-chambered heart.";
      })
      .defmethod("Platypus", function(p) {
        return callNextMethod(this, p) + " [Aquatic]";
      });

    var find1 = describe.findMethod(new Platypus());
    var find2 = describe.findMethod(new Platypus());

    describe.removeMethod("Platypus");

    var find3 = describe.findMethod(new Platypus());
    var find4 = describe.findMethod(new Mammal());

    expect(find1).toBe(find2);
    expect(find3).toBe(find4);
  });

  it("name", () => {
    var name = defgeneric("name")
      .defmethod("Mammal", function() {
        return "Mammy";
      })
      .defmethod("Platypus", function(p) {
        return "Platy " + callNextMethod(this, p);
      });

    var find1 = name.findMethod(new Platypus());

    name.defmethod("Platypus", function() {
      return "Pat";
    });
    name.removeMethod("Mammal");

    expect(find1(new Platypus())).toBe("Platy Mammy");
    expect(name(new Platypus())).toBe("Pat");
  });

  it("Before and After methods", function() {
    var msgs = "";
    var log = function(str) {
      msgs += str;
    };
    var describe = defgeneric("describe")
      .defmethod("Platypus", function() {
        log("Platy" + arguments.length.toString());
        return "P";
      })
      .defmethod("Mammal", function() {
        log("Mammy" + arguments.length.toString());
        return "M";
      })
      .defmethod(
        "Platypus",
        function() {
          log("platypus" + arguments.length.toString());
        },
        "before"
      )
      .defmethod(
        "Mammal",
        function() {
          log("mammal" + arguments.length.toString());
        },
        "before"
      )
      .defmethod(
        "object",
        function() {
          log("object" + arguments.length.toString());
        },
        "before"
      )
      .defmethod(
        "Platypus",
        function() {
          log("/platypus" + arguments.length.toString());
        },
        "after"
      )
      .defmethod(
        "Mammal",
        function() {
          log("/mammal" + arguments.length.toString());
        },
        "after"
      )
      .defmethod(
        "object",
        function() {
          log("/object" + arguments.length.toString());
        },
        "after"
      );

    var tryIt = function(a) {
      msgs = "";
      var ret = describe(a);
      return ret + ":" + msgs;
    };

    expect(tryIt(new Platypus())).toBe(
      "P:platypus1mammal1object1Platy1/object1/mammal1/platypus1"
    );
  });
});

describe("Tests from the checker", () => {
  it("No method found", () => {
    var append = defgeneric("append");
    append.defmethod("*,Array", function(a, b) {
      return [a].concat(b);
    });
    append.defmethod("Array,*", function(a, b) {
      return a.concat([b]);
    });
    append.defmethod("Array,Array", function(a, b) {
      return a.concat(b);
    });

    expect(append.bind(null, 1, 2)).toThrow(
      "No method found for append with args: number,number"
    );
  });
});
