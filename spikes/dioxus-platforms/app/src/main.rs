//! Throwaway spike: the rehearsal screen on Linux, Android and the web.
//! See docs/superpowers/specs/2026-11-14-cross-platform-spike-design.md.

use cuecard_spike_core::{blank, initials, matching_words};
use dioxus::prelude::*;
use serde::Deserialize;

const CSS: &str = include_str!("style.css");

// Public-domain text: Molière, Tartuffe, acte III, scène 2.
const CUE: &str = "Couvrez ce sein que je ne saurais voir.";
const LINE: &str = "Vous êtes donc bien tendre à la tentation ?";

// Microphone and audio go through the browser media APIs on every target
// (the web page itself, or the system WebView on desktop and Android).
const MIC_START_JS: &str = r#"
(async () => {
  try {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    const ctx = new AudioContext();
    const analyser = ctx.createAnalyser();
    analyser.fftSize = 1024;
    ctx.createMediaStreamSource(stream).connect(analyser);
    const samples = new Uint8Array(analyser.fftSize);
    window.__cuecardMic = { stream, ctx };
    const tick = () => {
      if (!window.__cuecardMic) { return; }
      analyser.getByteTimeDomainData(samples);
      let sum = 0;
      for (const v of samples) { const d = (v - 128) / 128; sum += d * d; }
      dioxus.send({ level: Math.sqrt(sum / samples.length) });
      requestAnimationFrame(tick);
    };
    tick();
  } catch (e) {
    dioxus.send({ error: String(e && e.name ? e.name : e) });
  }
})();
"#;

const MIC_STOP_JS: &str = r#"
const mic = window.__cuecardMic;
window.__cuecardMic = null;
if (mic) { mic.stream.getTracks().forEach((t) => t.stop()); mic.ctx.close(); }
"#;

const TONE_JS: &str = r#"
const ctx = new AudioContext();
const osc = ctx.createOscillator();
const gain = ctx.createGain();
osc.frequency.value = 440;
gain.gain.value = 0.1;
osc.connect(gain);
gain.connect(ctx.destination);
osc.start();
setTimeout(() => { osc.stop(); ctx.close(); }, 600);
"#;

#[derive(Clone, Copy, PartialEq)]
enum Level {
    Full,
    Initials,
    Blank,
}

#[derive(Deserialize)]
struct MicMessage {
    level: Option<f64>,
    error: Option<String>,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn LevelButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            "aria-pressed": "{active}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

#[component]
fn App() -> Element {
    let mut level = use_signal(|| Level::Initials);
    let mut mic_on = use_signal(|| false);
    let mut mic_level = use_signal(|| 0.0_f64);
    let mut mic_status = use_signal(|| String::from("Micro arrêté"));
    let mut typed = use_signal(String::new);

    let shown = match level() {
        Level::Full => LINE.to_string(),
        Level::Initials => initials(LINE),
        Level::Blank => blank(LINE),
    };
    let result = matching_words(LINE, &typed());
    let meter = (mic_level() * 300.0).min(100.0);

    let start_mic = move |_| {
        if mic_on() {
            return;
        }
        mic_on.set(true);
        mic_status.set(String::from("Autorisation demandée…"));
        spawn(async move {
            let mut eval = document::eval(MIC_START_JS);
            while let Ok(msg) = eval.recv::<MicMessage>().await {
                if let Some(error) = msg.error {
                    mic_status.set(format!("Erreur : {error}"));
                    mic_on.set(false);
                    break;
                }
                if let Some(value) = msg.level {
                    mic_level.set(value);
                    mic_status.set(String::from("Micro actif"));
                }
            }
        });
    };

    let stop_mic = move |_| {
        let _ = document::eval(MIC_STOP_JS);
        mic_on.set(false);
        mic_level.set(0.0);
        mic_status.set(String::from("Micro arrêté"));
    };

    let play_tone = move |_| {
        let _ = document::eval(TONE_JS);
    };

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        style { "{CSS}" }
        div { class: "app",
            nav { class: "nav", "aria-label": "Navigation",
                a { class: "nav-item active", href: "#", "Bibliothèque" }
                a { class: "nav-item", href: "#", "Progression" }
                a { class: "nav-item", href: "#", "Réglages" }
            }
            main { class: "screen",
                div { class: "bar",
                    div {
                        strong { "Tartuffe · Acte III, scène 2" }
                        div { class: "note", "Je joue Elmire · réplique 12 sur 74" }
                    }
                    div { class: "levels",
                        LevelButton {
                            label: "Complet",
                            active: level() == Level::Full,
                            onclick: move |_| level.set(Level::Full),
                        }
                        LevelButton {
                            label: "Initiales",
                            active: level() == Level::Initials,
                            onclick: move |_| level.set(Level::Initials),
                        }
                        LevelButton {
                            label: "Vide",
                            active: level() == Level::Blank,
                            onclick: move |_| level.set(Level::Blank),
                        }
                    }
                }
                section { class: "card", "aria-label": "Réplique donnée",
                    div { class: "label", "Réplique donnée · Tartuffe" }
                    div { class: "text", "{CUE}" }
                }
                section { class: "card mine", "aria-label": "Ma réplique",
                    div { class: "label", "À moi · Elmire" }
                    div { class: "text", "{shown}" }
                }

                div { class: "row",
                    if mic_on() {
                        button { class: "primary", onclick: stop_mic, "Arrêter le micro" }
                    } else {
                        button { class: "primary", onclick: start_mic, "Je dis ma réplique" }
                    }
                    div { class: "meter", role: "meter", "aria-label": "Niveau du micro",
                        div { style: "width: {meter}%" }
                    }
                }
                div { class: "note", "{mic_status}" }

                div { class: "row",
                    button { onclick: play_tone, "Jouer un son" }
                    button { disabled: true, "Kokoro (phase B)" }
                }

                div { class: "field",
                    label { r#for: "typed", "Tape ta réplique (é è ê à ç œ ’ « »)" }
                    input {
                        id: "typed",
                        r#type: "text",
                        value: "{typed}",
                        oninput: move |e| typed.set(e.value()),
                    }
                    div { class: "note", "{result.matched} mots sur {result.total}" }
                }
            }
        }
    }
}
