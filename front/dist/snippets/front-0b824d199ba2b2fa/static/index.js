


const invoke = window.__TAURI__.invoke

export async function invokePrint(buf) {
    return await invoke("printer", {buf: buf});
}
