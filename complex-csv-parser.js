/**
 * CSV Parser.  Takes a string as input and returns
 * an array of arrays (for each row).
 * 
 * @param input String, CSV input
 * @param separator String, single character used to separate fields.
 *        Defaults to ","
 * @param quote String, single character used to quote non-simple fields.
 *        Defaults to "\"".
 */
function parseCSV(input, separator, quote) {
  separator = separator || ',';
  quote = quote || '"';
  
  let stream = input[Symbol.iterator]();
  let peek = stream.next();
  let lineNo = 1, column = 1;

  function error(message) {
    throw new Error(`${message} at line ${lineNo}, column ${column}`);
  }

  function next() {
    peek = stream.next();
    if (peek.value === '\n') {
      lineNo++;
      column = 1;
    } else {
      column++;
    }
  }

  function cell() {
    let text = '';

    if (peek.value === quote) {
      next();
      while (true) {

        if (peek.done) {
          error('unmatch quote');
        }

        if (peek.value === quote) {
          next();
          if (peek.value === quote) {
            next();
            text += quote;
          } else {
            break;
          }
        } else {
          text += peek.value;
          next();
        }
      }
    } else {
      while (true) {
        if (peek.done || peek.value === separator || peek.value === '\n') {
          break;
        } else {
          text += peek.value;
          next();
        }
      }
    }

    return text;
  }

  function line() {
    let cs = [];

    while (true) {
      cs.push(cell());

      if (peek.done) {
        break;
      }

      else if (peek.value === '\n') {
        next();
        break;
      }

      else if (peek.value === separator) {
        next();
      }

      else {
        error('expect a separator');
      }

      
    }

    return cs;
  }

  let lns = [];
  while (true) {
    lns.push(line());

    if (peek.done) {
      break;
    }
  }
  return lns;
}