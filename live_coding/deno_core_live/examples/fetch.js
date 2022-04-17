function print(data) {
  Deno.core.print(`${data}\n`);
}

print("starting to fetch...");
let res = await fetch("https://www.google.com");
print(`status: ${res.status}`);
print(`headers: ${JSON.stringify(res.headers)}`);
