// https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9/train/javascript
const findShort = s => Math.min(...s.split(/\s+/).map(w => w.length));
