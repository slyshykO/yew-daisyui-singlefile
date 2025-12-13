use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div class="min-h-screen flex items-center justify-center px-4">
            <div class="card bg-base-100 shadow-xl w-full max-w-md">
                <div class="card-body">
                    <h2 class="card-title">{"Yew + DaisyUI + Vite"}</h2>
                    <p class="text-sm opacity-80">
                        {"Single-file demo with local Tailwind 4 + DaisyUI 5, no CDNs."}
                    </p>

                    <div class="mt-6 flex items-center gap-4">
                        <button class="btn btn-primary" {onclick}>
                            {"Increment"}
                        </button>
                        <span class="badge badge-lg">
                            { format!("Count: {}", *counter) }
                        </span>
                    </div>

                    <p class="mt-4 text-xs opacity-60">
                        {"Build with `npm run wasm && npm run build` to get a single dist/index.html."}
                    </p>
                </div>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    yew::Renderer::<App>::new().render();
    Ok(())
}
