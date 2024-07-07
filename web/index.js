import "https://unpkg.com/react@18/umd/react.development.js";
import "https://unpkg.com/react-dom@18/umd/react-dom.development.js";

import init, { App, WasmReact } from "../pkg/tetris_with_wasm.js";

await init();

WasmReact.useReact(window.React);

const root = ReactDOM.createRoot(document.getElementById("root"));

root.render(
  React.createElement(App, {
    width: 10,
    height: 25,
  })
);

//todo
//make react\next in web?