# 1. Use Kokoro for text-to-speech

- Status: Accepted
- Date: 2026-11-14

## Context

Cuecard reads the other characters' lines aloud so an actor can rehearse their own. This needs a text-to-speech (TTS) engine that:

- runs locally on Linux, with no cloud dependency;
- speaks French well, since the scripts are French;
- has a license compatible with an open-source repo under a permissive license (MIT or Apache-2.0).

Breeze TTS 2 was the first candidate. It was dropped for two reasons:

- Its model weights, derivatives and self-hosted outputs are under a research and non-commercial license. Only its inference code is Apache-2.0.
- It supports only English and Chinese.

## Decision

Use **Kokoro-82M** as the default TTS engine.

- The weights are Apache-2.0, with a single license covering all voices.
- It has 82M parameters and 54 voices across 8 languages, French included, and runs faster than real time on a CPU.
- It was trained on permissive or public-domain audio only. Part of the training data is CC BY (including the SIWIS dataset, CC BY 4.0), so we credit it in the README.

The engine sits behind a small `Tts` interface in the code, so it can be replaced without touching the rest of the app.

## Consequences

Good:

- Fully local and offline, with no GPU required.
- No license friction for our own code: Apache-2.0 or MIT both work.
- Small model, so the download and memory cost are low.

Bad or to watch:

- **One French voice.** Kokoro appears to ship a single French voice (a female voice called Siwis). Scenes with several characters will share it, so we need a way to tell speakers apart, for example by adjusting speed, or by adding another engine later.
- **Phonemizer licensing is unverified.** The text-to-phoneme step for French may rely on espeak-ng, which is GPL-3.0. Before bundling anything, check that this does not pull GPL code into our binary. Options: run it as a separate process, or use a permissively licensed phonemizer.
- **Rust integration is not decided.** Kokoro is a Python/ONNX-style model. We have not yet chosen between an ONNX runtime from Rust, a local subprocess, or a small local server. This needs its own ADR once we prototype.
- Voice quality in French has not been evaluated by ear yet.

## Alternatives considered

- **Breeze TTS 2**: rejected, non-commercial weights and no French (see Context).
- **Chatterbox Multilingual** (MIT, French, voice cloning): good fit for giving each character a distinct voice, but its official setup is GPU-oriented and every output carries an embedded watermark. Kept as a possible second engine behind the same interface.
- **Piper**: fast and CPU-only with many French voices, but the maintained engine is GPL-3.0 and individual voices carry their own licenses, some marked non-commercial. Rejected for now.

## References

- Kokoro-82M: <https://huggingface.co/hexgrad/Kokoro-82M>
- Chatterbox: <https://github.com/resemble-ai/chatterbox>
- Piper (maintained fork): <https://github.com/OHF-Voice/piper1-gpl>
- Breeze TTS 2: <https://huggingface.co/BreezeBlue/Breeze-TTS-2>
