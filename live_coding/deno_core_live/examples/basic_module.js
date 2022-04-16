import { print } from "./base.js";

async function hello() {
  return new Promise((res, _rej) => {
    print("Hello world!\n");
    res("Hello");
  });
}

await hello();
