class EmptyBinaryTree {
  constructor(freeze = true) {
    if (freeze) {
      Object.freeze(this);
    }
  }

  isEmpty() {
    return true;
  }

  depth() {
    return 0;
  }

  count() {
    return 0;
  }

  inorder(fn) { }

  preorder(fn) { }

  postorder(fn) { }

  contains(x) {
    return false;
  }

  insert(x) {
    return new BinaryTreeNode(x, new EmptyBinaryTree(), new EmptyBinaryTree());
  }

  remove(x) {
    return this;
  }
}

class BinaryTreeNode extends EmptyBinaryTree {
  constructor(value, left, right) {
    super(false);
    this.value = value;
    this.left = left;
    this.right = right;
    Object.freeze(this);
  }

  isEmpty() {
    return false;
  }

  depth() {
    return Math.max(this.left.depth(), this.right.depth()) + 1;
  }

  count() {
    return this.left.count() + this.right.count() + 1;
  }

  inorder(fn) {
    this.left.inorder(fn);
    fn(this.value);
    this.right.inorder(fn);
  }

  preorder(fn) {
    fn(this.value);
    this.left.preorder(fn);
    this.right.preorder(fn);
  }

  postorder(fn) {
    this.left.postorder(fn);
    this.right.postorder(fn);
    fn(this.value);
  }

  contains(x) {
    if (x < this.value) {
      return this.left.contains(x);
    } else if (x > this.value) {
      return this.right.contains(x);
    } else if (x === this.value) {
      return true;
    } else {
      return false;
    }
  }

  insert(x) {
    if (x < this.value) {
      return new BinaryTreeNode(this.value, this.left.insert(x), this.right);
    } else if (x > this.value) {
      return new BinaryTreeNode(this.value, this.left, this.right.insert(x));
    } else {
      const lc = new BinaryTreeNode(x, this.left, new EmptyBinaryTree());
      return new BinaryTreeNode(this.value, lc, this.right);
    }
  }

  remove(x) {
    if (!this.contains(x)) {
      return this;
    }
    if (x < this.value) {
      return new BinaryTreeNode(this.value, this.left.remove(x), this.right);
    } else if (x > this.value) {
      return new BinaryTreeNode(this.value, this.left, this.right.remove(x));
    } else {
      // thie is the node to be remove
      // find the minimal element
      if (this.left.isEmpty()) {
        return this.right;
      } else if (this.right.isEmpty()) {
        return this.left;
      } else {
        // both left and right are not empty node
        let m = this.right;
        while (!m.left.isEmpty()) {
          m = m.left;
        }
        return new BinaryTreeNode(m.value, this.left, this.right.remove(m.value));
      }
    }
  }
}
