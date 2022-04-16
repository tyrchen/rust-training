async function hello() {
  return new Promise((res, _rej) => {
    Deno.core.print("Hello world!\n");
    res("Hello");
  });
}

hello();
