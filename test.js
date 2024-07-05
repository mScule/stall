let i = 0;

const start = performance.now();
while (i < 10_000_000_000) {
    i++;
}
const end = performance.now();
console.log(`Time: ${((end - start) * 1000).toFixed(3)}µs`);
console.log(i);