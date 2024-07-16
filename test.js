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

/*
hashmap
| get_const $cur, 0; 
array;
"0001";
"Mike";
10i;
"0002";
"Jack";
20i;
": ";
", ";
func
| debug_cursor: "", 0, 0;
| get_const: $cur, 0;
| def_var;
| get_const: $cur, 1;
| get_const: $cur, 3;
| push;
| get_const: $cur, 4;
| push;
| get_const: $cur, 2;
| set_var: $cur, 0;
| get_var: $cur, 0;
| get_const: $cur, 1;
| get_const: $cur, 6;
| push;
| get_const: $cur, 7;
| push;



*/

/**
 * var users = hashmap
 *
 * users .= set("0001" { name = "Mike" age = 10 })
 * users .= set("0002" { name = "Jack" age = 20 })
 *
 * for (key user) in users.entries {
 *     log.info(key": "user.name", "age)
 * }
 */
