((window) => {
  async function fetch(args) {
    if (typeof args === "string") {
      args = { url: args, method: "GET", headers: [], body: null };
    } else if (typeof args === "object") {
      if (args.url) {
        args.method = args.method || "GET";
        args.headers = args.headers || [];
        args.body = args.body || null;
      } else {
        throw new Error("Invalid arguments");
      }
    } else {
      throw new Error("Invalid fetch args, should be string or object");
    }
    return await Deno.core.opAsync("op_fetch", args);
  }

  window.fetch = fetch;
})(this);
