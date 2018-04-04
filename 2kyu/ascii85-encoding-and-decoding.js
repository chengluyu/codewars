String.prototype.tear = function (n, c) {
  const blocks = [];
  for (let i = 0; i < this.length; i += n) {
    blocks.push(this.substr(i, n));
  }
  if (blocks.length > 0) {
    const last = blocks.length - 1;
    if (blocks[last].length < n) {
      blocks[last] += c.repeat(n - this.length % n);
    }
  }
  return blocks;
}

String.prototype.toAscii85 = function () {
  // divide string into blocks
  const blocks = this.tear(4, '\0');
  console.log('Blocks', blocks);
  // encode each block
  const encoded = blocks.map(block => {
    let sum = [...block].reduce((r, c) => r * 256 + c.charCodeAt(0), 0);
    let result = '';
    for (let i = 0; i < 5; i++) {
      result = String.fromCharCode(33 + sum % 85) + result;
      sum = Math.floor(sum / 85);
    }
    return result;
  })
  console.log('Encoded', encoded);
  // truncate last block
  if (this.length % 4 > 0) {
    const last = encoded.length - 1;
    encoded[last] = encoded[last].substr(0, 1 + this.length % 4);
  }
  console.log('Truncated', encoded);
  // compress zero blocks
  const compressed = encoded.map(block => block === '!!!!!' ? 'z' : block);
  console.log('Compressed', compressed);
  // concatenate result together
  return `<~${compressed.join('')}~>`;
};

String.prototype.fromAscii85 = function () {
  // remove enclosing brackets and whitespaces
  const body = this.substring(2, this.length - 2).replace(/\s+/g, '').replace(/z/g, '!!!!!');
  console.log('Body', body);
  // divide string into blocks
  const blocks = body.tear(5, 'u');
  console.log('Blocks', blocks);
  // decode each block
  const encoded = blocks.map(block => {
    let sum = [...block].reduce((r, c) => r * 85 + c.charCodeAt(0) - 33, 0);
    let result = '';
    for (let i = 0; i < 4; i++) {
      result = String.fromCharCode(sum % 256) + result;
      sum = Math.floor(sum / 256);
    }
    return result;
  });
  console.log('Decoded', encoded);
  // truncate last block
  if (body.length % 5 > 0) {
    const last = encoded.length - 1;
    encoded[last] = encoded[last].substr(0, body.length % 5 - 1);
  }
  console.log('Truncated', encoded);
  return encoded.join('');
};
