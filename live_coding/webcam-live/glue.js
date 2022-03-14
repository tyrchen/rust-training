const invoke = window.__TAURI__.invoke;

export async function invokeSetWindowDecorations(decorations) {
  return await invoke("set_window_decorations", { decorations });
}
