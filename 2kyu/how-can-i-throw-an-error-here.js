const F = (function*(){}).constructor;
const bang = () => (new F(`th${''}row new Err${''}or("Just t${''}hrow like this!")`))().next();
