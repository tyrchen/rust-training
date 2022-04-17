function print(data) {
  Deno.core.print(`${data}\n`);
}

print("starting to fetch...");
let res = await fetch({
  url: "https://jsonplaceholder.typicode.com/todos/1",
  method: "GET",
});
print(`status: ${res.status}`);
print(`headers: ${JSON.stringify(res.headers, null, 2)}`);
print(`body: ${JSON.stringify(res.json(), null, 2)}`);
