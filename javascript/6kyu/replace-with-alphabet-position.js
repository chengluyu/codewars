// https://www.codewars.com/kata/546f922b54af40e1e90001da/train/javascript
const alphabetPosition = text =>
  [...text.toLowerCase()]
    .filter(x => "a" <= x && x <= "z")
    .map(c => c.charCodeAt(0) - 96)
    .join(" ");
