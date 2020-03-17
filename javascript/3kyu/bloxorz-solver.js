function bloxSolver(tiles) {
  // scale
  const rows = tiles.length, cols = tiles[0].length;
  // positions
  const UPRIGHT = 'U';
  const HORIZONTAL = 'H';
  const VERTICAL = 'V';
  // location
  class Location {
    constructor(position, x, y, previous = null, action = null) {
      this.position = position;
      this.x = x;
      this.y = y;
      this.encode = `${this.position}-${x < 10 ? '0' + x : x}-${y < 10 ? '0' + y : y}`;
      this.previous = previous;
      this.action = action;
    }

    get valid() {
      let conds;
      switch (this.position) {
        case UPRIGHT:
          return this.x >= 0 &&
            this.y >= 0 &&
            this.x < cols &&
            this.y < rows &&
            tiles[this.y].charAt(this.x) !== '0';
        case HORIZONTAL:
          return this.x >= 0 &&
            this.y >= 0 &&
            this.x + 1 < cols &&
            this.y < rows &&
            tiles[this.y].charAt(this.x) !== '0' &&
            tiles[this.y].charAt(this.x + 1) !== '0';
        case VERTICAL:
          return this.x >= 0 &&
            this.y >= 0 &&
            this.x < cols &&
            this.y + 1 < rows &&
            tiles[this.y].charAt(this.x) !== '0' &&
            tiles[this.y + 1].charAt(this.x) !== '0';
        default:
          throw new Error(`Unknown position: "${this.position}"`);
      }
    }

    up() {
      switch (this.position) {
        case UPRIGHT:
          return new Location(VERTICAL, this.x, this.y - 2, this, 'U');
        case HORIZONTAL:
          return new Location(HORIZONTAL, this.x, this.y - 1, this, 'U');
        case VERTICAL:
          return new Location(UPRIGHT, this.x, this.y - 1, this, 'U');
        default:
          throw new Error(`Unknown position: "${this.position}"`);
      }
    }

    down() {
      switch (this.position) {
        case UPRIGHT:
          return new Location(VERTICAL, this.x, this.y + 1, this, 'D');
        case HORIZONTAL:
          return new Location(HORIZONTAL, this.x, this.y + 1, this, 'D');
        case VERTICAL:
          return new Location(UPRIGHT, this.x, this.y + 2, this, 'D');
        default:
          throw new Error(`Unknown position: "${this.position}"`);
      }
    }

    left() {
      switch (this.position) {
        case UPRIGHT:
          return new Location(HORIZONTAL, this.x - 2, this.y, this, 'L');
        case HORIZONTAL:
          return new Location(UPRIGHT, this.x - 1, this.y, this, 'L');
        case VERTICAL:
          return new Location(VERTICAL, this.x - 1, this.y, this, 'L');
        default:
          throw new Error(`Unknown position: "${this.position}"`);
      }
    }

    right() {
      switch (this.position) {
        case UPRIGHT:
          return new Location(HORIZONTAL, this.x + 1, this.y, this, 'R');
        case HORIZONTAL:
          return new Location(UPRIGHT, this.x + 2, this.y, this, 'R');
        case VERTICAL:
          return new Location(VERTICAL, this.x + 1, this.y, this, 'R');
        default:
          throw new Error(`Unknown position: "${this.position}"`);
      }
    }
  }
  // locate the tile
  let initialCoordinate;
  let targetCoordinate;
  for (let i = 0; i < rows; i++) {
    for (let j = 0; j < cols; j++) {
      if (tiles[i].charAt(j) === 'B') {
        initialCoordinate = {
          x: j,
          y: i
        };
      } else if (tiles[i].charAt(j) === 'X') {
        targetCoordinate = {
          x: j,
          y: i
        };
      }
    }
  }
  // console.log('Initial', initialCoordinate);
  // console.log('Target', targetCoordinate)
  const queue = [
    new Location(
      UPRIGHT,
      initialCoordinate.x,
      initialCoordinate.y
    )
  ];
  const visited = new Set(queue);
  // BFS
  while (queue.length > 0) {
    let loc = queue.shift();
    if (
      loc.x === targetCoordinate.x &&
      loc.y === targetCoordinate.y &&
      loc.position === UPRIGHT
    ) {
      let solution = [];
      while (loc.previous) {
        solution.push(loc.action);
        loc = loc.previous;
      }
      return solution.reverse().join('');
    }
    [
      loc.up(),
      loc.down(),
      loc.left(),
      loc.right()
    ].
    filter(x => {
      return x.valid && !visited.has(x.encode);
    }).
    forEach(x => {
      visited.add(x.encode);
      queue.push(x);
    });
  }
}
