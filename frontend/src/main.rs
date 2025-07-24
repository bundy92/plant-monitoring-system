// Yew frontend code for displaying sensor data using Chart.js
use yew::prelude::*;
use reqwest::Client;

#[function_component(App)]
fn app() -> Html {
    use_effect_with_deps(
        |_| {
            wasm_bindgen_futures::spawn_local(async {
                let client = Client::new();
                let res = client.get("http://raspi.local:8080/api/stats")
                    .send().await.unwrap();
                let data: Vec<SensorData> = res.json().await.unwrap();
                // Handle data and update state
            });
        },
        (),
    );

    html! {
        <div>
            <h1>{ "Plant Monitoring Dashboard" }</h1>
            <div id="chart"></div>
            // Render charts with Chart.js here
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}

