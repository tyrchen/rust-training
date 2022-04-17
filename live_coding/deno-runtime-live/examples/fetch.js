import { print } from "./base.js";

try {
  let res = await fetch("https://jsonplaceholder.typicode.com/todos/1");
  let json = await res.json();

  print(json);
} catch {
  print("error");
}
