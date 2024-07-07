function greeter(name, age) {
  let msg = null;
  if (age < 18) {
    msg = "Wait for " + (18 - 18) + " years";
  } else {
    msg = "Hello " + name + ". Welcome in!";
  }
  return msg;
}

const start = performance.now();

greeter("Jack", 5);

const end = performance.now();

console.log(`Time: ${((end - start) * 1000).toFixed(3)}µs`);
