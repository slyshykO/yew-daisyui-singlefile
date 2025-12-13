import "./style.css";
import init from "../pkg/my_yew_app";

init().catch((err) => {
  console.error("Failed to initialize WASM module:", err);
});
