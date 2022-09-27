use commands::{Command, ListContainerResponse, ListContainers};
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_ref(|| NodeRef::default());

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    println!("{:?}", &*name);

                    let resp = ListContainers {}.invoke().await;

                    let string = format!("{:?}", resp);

                    log(&string);
                    greet_msg.set(string);
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |_| {
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <p>
                {"Recommended IDE setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>

            <div class="row">
                <input id="greet-input" ref={&*greet_input_ref} placeholder="Enter a name..." />
                <button type="button" onclick={greet}>{"Greet"}</button>
            </div>

            <p><b>{ &*greet_msg }</b></p>
            <Section title="On Deck">
            <a class="button" href="https://019590178469.signin.aws.amazon.com/console" target="_blank">

            <svg width="1.2em" height="1em" viewBox="0 0 256 153" version="1.1" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid">
                    <g>
                            <path d="M72.392053,55.4384106 C72.392053,58.5748344 72.7311258,61.1178808 73.3245033,62.9827815 C74.002649,64.8476821 74.8503311,66.8821192 76.0370861,69.0860927 C76.4609272,69.7642384 76.6304636,70.4423841 76.6304636,71.0357616 C76.6304636,71.8834437 76.1218543,72.7311258 75.0198675,73.5788079 L69.6794702,77.1390728 C68.9165563,77.6476821 68.1536424,77.9019868 67.4754967,77.9019868 C66.6278146,77.9019868 65.7801325,77.4781457 64.9324503,76.7152318 C63.7456954,75.4437086 62.7284768,74.0874172 61.8807947,72.7311258 C61.0331126,71.2900662 60.1854305,69.6794702 59.2529801,67.7298013 C52.6410596,75.5284768 44.3337748,79.4278146 34.3311258,79.4278146 C27.210596,79.4278146 21.5311258,77.3933775 17.3774834,73.3245033 C13.2238411,69.2556291 11.1046358,63.8304636 11.1046358,57.0490066 C11.1046358,49.8437086 13.6476821,43.994702 18.818543,39.586755 C23.989404,35.1788079 30.8556291,32.9748344 39.586755,32.9748344 C42.4688742,32.9748344 45.4357616,33.2291391 48.5721854,33.6529801 C51.7086093,34.0768212 54.9298013,34.7549669 58.3205298,35.5178808 L58.3205298,29.3298013 C58.3205298,22.8874172 56.9642384,18.394702 54.3364238,15.7668874 C51.6238411,13.1390728 47.0463576,11.8675497 40.5192053,11.8675497 C37.5523179,11.8675497 34.5006623,12.2066225 31.3642384,12.9695364 C28.2278146,13.7324503 25.1761589,14.6649007 22.2092715,15.8516556 C20.8529801,16.4450331 19.8357616,16.784106 19.2423841,16.9536424 C18.6490066,17.1231788 18.2251656,17.207947 17.8860927,17.207947 C16.6993377,17.207947 16.1059603,16.3602649 16.1059603,14.5801325 L16.1059603,10.4264901 C16.1059603,9.07019868 16.2754967,8.05298013 16.6993377,7.45960265 C17.1231788,6.86622517 17.8860927,6.27284768 19.0728477,5.6794702 C22.0397351,4.15364238 25.6,2.88211921 29.7536424,1.86490066 C33.9072848,0.762913907 38.3152318,0.254304636 42.9774834,0.254304636 C53.0649007,0.254304636 60.4397351,2.54304636 65.186755,7.1205298 C69.8490066,11.6980132 72.2225166,18.6490066 72.2225166,27.9735099 L72.2225166,55.4384106 L72.392053,55.4384106 Z M37.9761589,68.3231788 C40.7735099,68.3231788 43.6556291,67.8145695 46.7072848,66.797351 C49.7589404,65.7801325 52.4715232,63.9152318 54.7602649,61.3721854 C56.1165563,59.7615894 57.1337748,57.981457 57.6423841,55.9470199 C58.1509934,53.9125828 58.4900662,51.4543046 58.4900662,48.5721854 L58.4900662,45.0119205 C56.0317881,44.418543 53.4039735,43.9099338 50.6913907,43.5708609 C47.9788079,43.2317881 45.3509934,43.0622517 42.7231788,43.0622517 C37.0437086,43.0622517 32.8900662,44.1642384 30.0927152,46.4529801 C27.2953642,48.7417219 25.9390728,51.9629139 25.9390728,56.2013245 C25.9390728,60.1854305 26.9562914,63.1523179 29.0754967,65.186755 C31.1099338,67.3059603 34.0768212,68.3231788 37.9761589,68.3231788 Z M106.045033,77.4781457 C104.519205,77.4781457 103.501987,77.2238411 102.823841,76.6304636 C102.145695,76.1218543 101.552318,74.9350993 101.043709,73.3245033 L81.1231788,7.7986755 C80.6145695,6.10331126 80.3602649,5.0013245 80.3602649,4.40794702 C80.3602649,3.05165563 81.0384106,2.28874172 82.394702,2.28874172 L90.7019868,2.28874172 C92.3125828,2.28874172 93.4145695,2.54304636 94.007947,3.13642384 C94.6860927,3.64503311 95.194702,4.83178808 95.7033113,6.44238411 L109.944371,62.5589404 L123.168212,6.44238411 C123.592053,4.74701987 124.100662,3.64503311 124.778808,3.13642384 C125.456954,2.62781457 126.643709,2.28874172 128.169536,2.28874172 L134.950993,2.28874172 C136.561589,2.28874172 137.663576,2.54304636 138.341722,3.13642384 C139.019868,3.64503311 139.613245,4.83178808 139.952318,6.44238411 L153.345695,63.2370861 L168.010596,6.44238411 C168.519205,4.74701987 169.112583,3.64503311 169.70596,3.13642384 C170.384106,2.62781457 171.486093,2.28874172 173.011921,2.28874172 L180.895364,2.28874172 C182.251656,2.28874172 183.01457,2.96688742 183.01457,4.40794702 C183.01457,4.83178808 182.929801,5.25562914 182.845033,5.76423841 C182.760265,6.27284768 182.590728,6.95099338 182.251656,7.88344371 L161.822517,73.4092715 C161.313907,75.1046358 160.72053,76.2066225 160.042384,76.7152318 C159.364238,77.2238411 158.262252,77.5629139 156.821192,77.5629139 L149.531126,77.5629139 C147.92053,77.5629139 146.818543,77.3086093 146.140397,76.7152318 C145.462252,76.1218543 144.868874,75.0198675 144.529801,73.3245033 L131.390728,18.6490066 L118.336424,73.2397351 C117.912583,74.9350993 117.403974,76.0370861 116.725828,76.6304636 C116.047682,77.2238411 114.860927,77.4781457 113.335099,77.4781457 L106.045033,77.4781457 Z M214.972185,79.7668874 C210.564238,79.7668874 206.156291,79.2582781 201.917881,78.2410596 C197.67947,77.2238411 194.37351,76.1218543 192.169536,74.8503311 C190.813245,74.0874172 189.880795,73.2397351 189.541722,72.4768212 C189.202649,71.7139073 189.033113,70.8662252 189.033113,70.1033113 L189.033113,65.7801325 C189.033113,64 189.711258,63.1523179 190.982781,63.1523179 C191.491391,63.1523179 192,63.2370861 192.508609,63.4066225 C193.017219,63.5761589 193.780132,63.9152318 194.627815,64.2543046 C197.509934,65.5258278 200.646358,66.5430464 203.952318,67.2211921 C207.343046,67.8993377 210.649007,68.2384106 214.039735,68.2384106 C219.380132,68.2384106 223.533775,67.3059603 226.415894,65.4410596 C229.298013,63.5761589 230.823841,60.8635762 230.823841,57.3880795 C230.823841,55.0145695 230.060927,53.0649007 228.535099,51.4543046 C227.009272,49.8437086 224.127152,48.402649 219.97351,47.0463576 L207.682119,43.2317881 C201.49404,41.2821192 196.916556,38.4 194.119205,34.5854305 C191.321854,30.8556291 189.880795,26.7019868 189.880795,22.2940397 C189.880795,18.7337748 190.643709,15.597351 192.169536,12.8847682 C193.695364,10.1721854 195.729801,7.7986755 198.272848,5.93377483 C200.815894,3.98410596 203.698013,2.54304636 207.088742,1.52582781 C210.47947,0.508609272 214.039735,0.0847682119 217.769536,0.0847682119 C219.634437,0.0847682119 221.584106,0.169536424 223.449007,0.42384106 C225.398675,0.678145695 227.178808,1.01721854 228.95894,1.35629139 C230.654305,1.78013245 232.264901,2.20397351 233.790728,2.71258278 C235.316556,3.22119205 236.503311,3.72980132 237.350993,4.2384106 C238.537748,4.91655629 239.38543,5.59470199 239.89404,6.35761589 C240.402649,7.03576159 240.656954,7.96821192 240.656954,9.15496689 L240.656954,13.1390728 C240.656954,14.9192053 239.978808,15.8516556 238.707285,15.8516556 C238.029139,15.8516556 236.927152,15.5125828 235.486093,14.8344371 C230.654305,12.6304636 225.229139,11.5284768 219.210596,11.5284768 C214.378808,11.5284768 210.564238,12.2913907 207.936424,13.9019868 C205.308609,15.5125828 203.952318,17.9708609 203.952318,21.4463576 C203.952318,23.8198675 204.8,25.8543046 206.495364,27.4649007 C208.190728,29.0754967 211.327152,30.6860927 215.819868,32.1271523 L227.856954,35.9417219 C233.960265,37.8913907 238.368212,40.6039735 240.996026,44.0794702 C243.623841,47.5549669 244.895364,51.5390728 244.895364,55.9470199 C244.895364,59.592053 244.13245,62.8980132 242.691391,65.7801325 C241.165563,68.6622517 239.131126,71.205298 236.503311,73.2397351 C233.875497,75.3589404 230.739073,76.8847682 227.09404,77.986755 C223.27947,79.1735099 219.295364,79.7668874 214.972185,79.7668874 Z" fill="currentColor" fill-rule="nonzero"></path>
                            <path d="M230.993377,120.964238 C203.104636,141.562914 162.58543,152.498013 127.745695,152.498013 C78.9192053,152.498013 34.9245033,134.442384 1.69536424,104.434437 C-0.932450331,102.060927 1.4410596,98.8397351 4.57748344,100.704636 C40.5192053,121.557616 84.8529801,134.188079 130.712583,134.188079 C161.65298,134.188079 195.645033,127.745695 226.924503,114.521854 C231.586755,112.402649 235.570861,117.57351 230.993377,120.964238 Z M242.606623,107.740397 C239.046358,103.162914 219.04106,105.536424 209.970861,106.638411 C207.258278,106.977483 206.834437,104.603974 209.292715,102.823841 C225.229139,91.6344371 251.422517,94.8556291 254.474172,98.5854305 C257.525828,102.4 253.62649,128.593377 238.707285,141.139073 C236.418543,143.088742 234.21457,142.071523 235.231788,139.528477 C238.622517,131.136424 246.166887,112.233113 242.606623,107.740397 Z" fill="currentColor"></path>
                    </g>
            </svg>

            {"AWS Dashboard"}</a>
            <a class="button" href="https://github.com/pulls?q=is%3Apr+author%3A%40me+archived%3Afalse+label%3Areleased+-label%3Achecked+" target="_blank">
                                <svg width="1em" height="1em" viewBox="0 0 256 250" version="1.1" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid">
                                <g>
                                <path d="M128.00106,0 C57.3172926,0 0,57.3066942 0,128.00106 C0,184.555281 36.6761997,232.535542 87.534937,249.460899 C93.9320223,250.645779 96.280588,246.684165 96.280588,243.303333 C96.280588,240.251045 96.1618878,230.167899 96.106777,219.472176 C60.4967585,227.215235 52.9826207,204.369712 52.9826207,204.369712 C47.1599584,189.574598 38.770408,185.640538 38.770408,185.640538 C27.1568785,177.696113 39.6458206,177.859325 39.6458206,177.859325 C52.4993419,178.762293 59.267365,191.04987 59.267365,191.04987 C70.6837675,210.618423 89.2115753,204.961093 96.5158685,201.690482 C97.6647155,193.417512 100.981959,187.77078 104.642583,184.574357 C76.211799,181.33766 46.324819,170.362144 46.324819,121.315702 C46.324819,107.340889 51.3250588,95.9223682 59.5132437,86.9583937 C58.1842268,83.7344152 53.8029229,70.715562 60.7532354,53.0843636 C60.7532354,53.0843636 71.5019501,49.6441813 95.9626412,66.2049595 C106.172967,63.368876 117.123047,61.9465949 128.00106,61.8978432 C138.879073,61.9465949 149.837632,63.368876 160.067033,66.2049595 C184.49805,49.6441813 195.231926,53.0843636 195.231926,53.0843636 C202.199197,70.715562 197.815773,83.7344152 196.486756,86.9583937 C204.694018,95.9223682 209.660343,107.340889 209.660343,121.315702 C209.660343,170.478725 179.716133,181.303747 151.213281,184.472614 C155.80443,188.444828 159.895342,196.234518 159.895342,208.176593 C159.895342,225.303317 159.746968,239.087361 159.746968,243.303333 C159.746968,246.709601 162.05102,250.70089 168.53925,249.443941 C219.370432,232.499507 256,184.536204 256,128.00106 C256,57.3066942 198.691187,0 128.00106,0 Z M47.9405593,182.340212 C47.6586465,182.976105 46.6581745,183.166873 45.7467277,182.730227 C44.8183235,182.312656 44.2968914,181.445722 44.5978808,180.80771 C44.8734344,180.152739 45.876026,179.97045 46.8023103,180.409216 C47.7328342,180.826786 48.2627451,181.702199 47.9405593,182.340212 Z M54.2367892,187.958254 C53.6263318,188.524199 52.4329723,188.261363 51.6232682,187.366874 C50.7860088,186.474504 50.6291553,185.281144 51.2480912,184.70672 C51.8776254,184.140775 53.0349512,184.405731 53.8743302,185.298101 C54.7115892,186.201069 54.8748019,187.38595 54.2367892,187.958254 Z M58.5562413,195.146347 C57.7719732,195.691096 56.4895886,195.180261 55.6968417,194.042013 C54.9125733,192.903764 54.9125733,191.538713 55.713799,190.991845 C56.5086651,190.444977 57.7719732,190.936735 58.5753181,192.066505 C59.3574669,193.22383 59.3574669,194.58888 58.5562413,195.146347 Z M65.8613592,203.471174 C65.1597571,204.244846 63.6654083,204.03712 62.5716717,202.981538 C61.4524999,201.94927 61.1409122,200.484596 61.8446341,199.710926 C62.5547146,198.935137 64.0575422,199.15346 65.1597571,200.200564 C66.2704506,201.230712 66.6095936,202.705984 65.8613592,203.471174 Z M75.3025151,206.281542 C74.9930474,207.284134 73.553809,207.739857 72.1039724,207.313809 C70.6562556,206.875043 69.7087748,205.700761 70.0012857,204.687571 C70.302275,203.678621 71.7478721,203.20382 73.2083069,203.659543 C74.6539041,204.09619 75.6035048,205.261994 75.3025151,206.281542 Z M86.046947,207.473627 C86.0829806,208.529209 84.8535871,209.404622 83.3316829,209.4237 C81.8013,209.457614 80.563428,208.603398 80.5464708,207.564772 C80.5464708,206.498591 81.7483088,205.631657 83.2786917,205.606221 C84.8005962,205.576546 86.046947,206.424403 86.046947,207.473627 Z M96.6021471,207.069023 C96.7844366,208.099171 95.7267341,209.156872 94.215428,209.438785 C92.7295577,209.710099 91.3539086,209.074206 91.1652603,208.052538 C90.9808515,206.996955 92.0576306,205.939253 93.5413813,205.66582 C95.054807,205.402984 96.4092596,206.021919 96.6021471,207.069023 Z" fill="currentColor" />
                                </g>
                                </svg>
                                {"Unreviewed PRs"}</a>
                                <a class="button" href="https://github.com/pulls/review-requested" target="_blank">

                <svg width="1em" height="1em" viewBox="0 0 256 250" version="1.1" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid">
                    <g>
                        <path d="M128.00106,0 C57.3172926,0 0,57.3066942 0,128.00106 C0,184.555281 36.6761997,232.535542 87.534937,249.460899 C93.9320223,250.645779 96.280588,246.684165 96.280588,243.303333 C96.280588,240.251045 96.1618878,230.167899 96.106777,219.472176 C60.4967585,227.215235 52.9826207,204.369712 52.9826207,204.369712 C47.1599584,189.574598 38.770408,185.640538 38.770408,185.640538 C27.1568785,177.696113 39.6458206,177.859325 39.6458206,177.859325 C52.4993419,178.762293 59.267365,191.04987 59.267365,191.04987 C70.6837675,210.618423 89.2115753,204.961093 96.5158685,201.690482 C97.6647155,193.417512 100.981959,187.77078 104.642583,184.574357 C76.211799,181.33766 46.324819,170.362144 46.324819,121.315702 C46.324819,107.340889 51.3250588,95.9223682 59.5132437,86.9583937 C58.1842268,83.7344152 53.8029229,70.715562 60.7532354,53.0843636 C60.7532354,53.0843636 71.5019501,49.6441813 95.9626412,66.2049595 C106.172967,63.368876 117.123047,61.9465949 128.00106,61.8978432 C138.879073,61.9465949 149.837632,63.368876 160.067033,66.2049595 C184.49805,49.6441813 195.231926,53.0843636 195.231926,53.0843636 C202.199197,70.715562 197.815773,83.7344152 196.486756,86.9583937 C204.694018,95.9223682 209.660343,107.340889 209.660343,121.315702 C209.660343,170.478725 179.716133,181.303747 151.213281,184.472614 C155.80443,188.444828 159.895342,196.234518 159.895342,208.176593 C159.895342,225.303317 159.746968,239.087361 159.746968,243.303333 C159.746968,246.709601 162.05102,250.70089 168.53925,249.443941 C219.370432,232.499507 256,184.536204 256,128.00106 C256,57.3066942 198.691187,0 128.00106,0 Z M47.9405593,182.340212 C47.6586465,182.976105 46.6581745,183.166873 45.7467277,182.730227 C44.8183235,182.312656 44.2968914,181.445722 44.5978808,180.80771 C44.8734344,180.152739 45.876026,179.97045 46.8023103,180.409216 C47.7328342,180.826786 48.2627451,181.702199 47.9405593,182.340212 Z M54.2367892,187.958254 C53.6263318,188.524199 52.4329723,188.261363 51.6232682,187.366874 C50.7860088,186.474504 50.6291553,185.281144 51.2480912,184.70672 C51.8776254,184.140775 53.0349512,184.405731 53.8743302,185.298101 C54.7115892,186.201069 54.8748019,187.38595 54.2367892,187.958254 Z M58.5562413,195.146347 C57.7719732,195.691096 56.4895886,195.180261 55.6968417,194.042013 C54.9125733,192.903764 54.9125733,191.538713 55.713799,190.991845 C56.5086651,190.444977 57.7719732,190.936735 58.5753181,192.066505 C59.3574669,193.22383 59.3574669,194.58888 58.5562413,195.146347 Z M65.8613592,203.471174 C65.1597571,204.244846 63.6654083,204.03712 62.5716717,202.981538 C61.4524999,201.94927 61.1409122,200.484596 61.8446341,199.710926 C62.5547146,198.935137 64.0575422,199.15346 65.1597571,200.200564 C66.2704506,201.230712 66.6095936,202.705984 65.8613592,203.471174 Z M75.3025151,206.281542 C74.9930474,207.284134 73.553809,207.739857 72.1039724,207.313809 C70.6562556,206.875043 69.7087748,205.700761 70.0012857,204.687571 C70.302275,203.678621 71.7478721,203.20382 73.2083069,203.659543 C74.6539041,204.09619 75.6035048,205.261994 75.3025151,206.281542 Z M86.046947,207.473627 C86.0829806,208.529209 84.8535871,209.404622 83.3316829,209.4237 C81.8013,209.457614 80.563428,208.603398 80.5464708,207.564772 C80.5464708,206.498591 81.7483088,205.631657 83.2786917,205.606221 C84.8005962,205.576546 86.046947,206.424403 86.046947,207.473627 Z M96.6021471,207.069023 C96.7844366,208.099171 95.7267341,209.156872 94.215428,209.438785 C92.7295577,209.710099 91.3539086,209.074206 91.1652603,208.052538 C90.9808515,206.996955 92.0576306,205.939253 93.5413813,205.66582 C95.054807,205.402984 96.4092596,206.021919 96.6021471,207.069023 Z" fill="currentColor" />
                    </g>
                </svg>
                                {"Review Requests"}</a>
                                <a class="button" href="https://app.clickup.com/14312306/v/l/dmrvj-1488?pr=26304556" target="_blank">
                                <svg width="1em" height="1em" viewBox="0 0 126 125" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path fill-rule="evenodd" clip-rule="evenodd" d="M25.9434 85.7138L39.4618 75.3593C46.6406 84.7317 54.2642 89.0508 62.7593 89.0508C71.2072 89.0508 78.6213 84.7815 85.4783 75.4832L99.1831 85.5899C89.2965 98.9962 76.9963 106.079 62.7593 106.079C48.5693 106.079 36.1552 99.046 25.9434 85.7138Z" fill="currentColor"/>
                                <path fill-rule="evenodd" clip-rule="evenodd" d="M62.7135 40.7078L38.6528 61.4406L27.5371 48.5407L62.7614 18.1885L97.7118 48.5644L86.5414 61.417L62.7135 40.7078Z" fill="currentColor"/>
                                </svg>
                {"Clickup Tasks"}</a>
            </Section>
            <Section title="Frontend">
            {
                "Hi"
            }
            </Section>

            <Section title="Backend">
            {"Hi"}
            </Section>
            <PodmanSection />
            <Section title="SSH Tunnel">
            <button>{"Staging"}</button>
            <button>{"Production"}</button>
            </Section>
        </main>
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    title: String,
    children: Children, // the field name `children` is important!
}

#[function_component(Section)]
fn section(props: &SectionProps) -> Html {
    html! {
        <div style="border-top: 1px solid rgb(224, 224, 224); position: relative;">
                <header style="position: absolute; top: -13px; font-weight: 500; left: 10px; background-color: #f6f6f6; color: rgb(180, 180, 180); padding: 0 0.8em;">{&props.title}</header>
                <div class="button-container">
                 { for props.children.iter() }
                </div>
                </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PodmanSectionProps {}

#[function_component(PodmanSection)]
fn podman_section(props: &PodmanSectionProps) -> Html {
    let greet_msg = use_state(|| Option::None);

    {
        let greet_msg = greet_msg.clone();
        let timeout = Timeout::new(1_000, move || {
            spawn_local(async move {
                let resp = ListContainers {}.invoke().await;
                greet_msg.set(Some(resp));
            });
        });

        timeout.forget();
    }

    html! {
        <Section title="Podman">
            <div style="display: flex; flex-direction: column; width: 100%; padding: 1em 2em; gap: 0.5em">{match &*greet_msg {
            Some(ListContainerResponse::Ok(items)) => html! {
                { items.iter().cloned().map(|name| html!{<PodmanEntry title={name} status={PodmanStatus::Running} />}).collect::<Html>() }
            },
            _ => html! {{"Loading"}}
        }}</div></Section>
    }
}

#[derive(Properties, PartialEq)]
pub struct PodmanEntryProps {
    title: String,
    status: PodmanStatus,
}

#[derive(PartialEq)]
enum PodmanStatus {
    Running,
    Stopping,
    Stopped,
}

#[function_component(PodmanEntry)]
fn podman_entry(props: &PodmanEntryProps) -> Html {
    html! {
        <div style="display: flex; align-items: center; justify-content: space-between"><div>{&props.title}</div><button>{match props.status {
            PodmanStatus::Running => "Running",
            PodmanStatus::Stopping => "Stopping",
            PodmanStatus::Stopped=> "Stopped",
        }}</button></div>
    }
}
