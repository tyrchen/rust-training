import * as Drash from "https://deno.land/x/drash@v2.5.4/mod.ts";

let todos: any[] = [];

// Create your resource

class HomeResource extends Drash.Resource {
  public paths = ["/"];

  public GET(request: Drash.Request, response: Drash.Response): void {
    return response.json({
      hello: "world",
      time: new Date(),
    });
  }
}

class TodResource extends Drash.Resource {
  public paths = ["/todos"];

  public GET(request: Drash.Request, response: Drash.Response): void {
    return response.json({
      todos,
    });
  }

  public POST(request: Drash.Request, response: Drash.Response): void {
    const todo = request.bodyAll();
    console.log(todo);
    todos.push(todo);
    return response.json({
      todos,
    });
  }
}

// Create and run your server

const server = new Drash.Server({
  hostname: "0.0.0.0",
  port: 1447,
  protocol: "http",
  resources: [HomeResource, TodResource],
});

server.run();

console.log(`Server running at ${server.address}.`);
