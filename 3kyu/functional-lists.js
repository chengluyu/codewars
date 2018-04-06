class EmptyList {
  constructor(freeze = true) {
    if (freeze) {
      Object.freeze(this);
    }
  }

  toString() {
    return '()';
  }

  isEmpty() {
    return true;
  }

  length() {
    return 0;
  }

  push(x) {
    return new ListNode(x, this);
  }

  remove(x) {
    return this;
  }

  append(x) {
    return new ListNode(x, this);
  }
}

class ListNode extends EmptyList {
  constructor(value, next, freeze = true) {
    super(false);
    this.value = value;
    this.next = next;
    if (freeze) {
      Object.freeze(this);
    }
  }

  toString() {
    const values = [];
    let t = this;
    while (!t.isEmpty()) {
      values.push(t.value);
      t = t.next;
    }
    return `(${values.join(' ')})`;
  }

  isEmpty() {
    return false;
  }

  length() {
    return this.next.length() + 1;
  }

  push(x) {
    return new ListNode(x, this);
  }

  head() {
    return this.value;
  }

  tail() {
    return this.next;
  }

  remove(x) {
    if (this.value === x) {
      return this.next.remove(x);
    }
    const head = new ListNode(this.value, null, false);
    let src = this.next, dst = head, removed = false;
    while (!src.isEmpty()) {
      if (src.value === x) {
        removed = true;
        dst.next = src.next.remove(x);
        Object.freeze(dst);
        break;
      } else {
        dst.next = new ListNode(src.value, null, false);
        Object.freeze(dst);
        dst = dst.next;
        src = src.next;
      }
    }
    return removed ? head : this;
  }

  append(that) {
    const head = new ListNode(this.value, null, false);
    let src = this.next, dst = head;
    while (!src.isEmpty()) {
      dst.next = new ListNode(src.value, null, false);
      Object.freeze(dst);
      dst = dst.next;
      src = src.next;
    }
    dst.next = that;
    Object.freeze(dst);
    return head;
  }
}

var list0 = new EmptyList();        // => "()"
console.log(list0.toString());
var list1 = list0.push(3);          // => "(3)"
console.log(list1.toString());
var list2 = list1.push(2);          // => "(2 3)"
console.log(list2.toString());
var list3 = list2.push(1);          // => "(1 2 3)"
console.log(list3.toString());
var list13 = list1.append(list3);   // => "(3 1 2 3)"
console.log(list13.toString());
var list1313 = list13.append(list13);   // => "(3 1 2 3)"
console.log(list1313.toString());
var list1313remove3 = list1313.remove(4);
console.log(list1313remove3.toString());
