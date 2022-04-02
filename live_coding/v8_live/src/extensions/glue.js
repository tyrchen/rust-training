"use strict";

({ print, fetch }) => {
  globalThis.print = (args) => {
    return print(args);
  };
  globalThis.fetch = async (args) => {
    return await fetch(args);
  };
};
