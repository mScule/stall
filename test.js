const start = performance.now();

let msg = null;
if (18 < 18) {
    msg = "Wait for " + (18 - 18) + " years";
} else {
    msg = "Hello " + "Jack" + ". Welcome in!";
}
msg;

const end = performance.now();

console.log(`Time: ${((end - start) * 1000).toFixed(3)}µs`);
console.log(msg);