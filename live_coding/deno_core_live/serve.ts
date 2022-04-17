import { serve } from "https://deno.land/std@0.120.0/http/server.ts";

function handler(req: Request): Response {
  return new Response("Hello My dear friends!!!");
}

console.log("Listening on http://localhost:8080");
await serve(handler, { port: 8080 });
