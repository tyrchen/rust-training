"use strict";

({ print, fetch }) => {
  globalThis.print = (args) => {
    return print(args);
  };
  globalThis.fetch = (args) => {
    return fetch(args);
  };
};
