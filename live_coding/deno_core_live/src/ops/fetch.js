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
    let res = await Deno.core.opAsync("op_fetch", args);
    res.text = () => {
      let body = res.body;
      if (!body) return null;
      return Deno.core.opSync("op_decode_utf8", body);
    };
    res.json = () => {
      let text = res.text();
      if (!text) return null;
      return JSON.parse(text);
    };
    return res;
  }

  window.fetch = fetch;
})(this);
