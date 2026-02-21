# whisper-latex-dictation

A Rust-based CLI application that uses local Whisper transcription and global keyboard structural hints to dictate mathematical formulas into LaTeX.

## Features
- **Multi-modal Input**: Combine audio dictation with keyboard structural anchors (parentheses, brackets, powers, etc.).
- **Local Privacy**: Audio is transcribed locally using Whisper (via `whisper-rs`).
- **AI-Powered Synthesis**: Uses Google Gemini to convert the unified timeline into clean LaTeX.
- **Continuous Logging**: Every event is saved to `dictation_log.txt` in real-time.

## Keybindings
| Key | Symbol |
| :-- | :----- |
| `j` | `(` |
| `k` | `)` |
| `u` | `[` |
| `i` | `]` |
| `m` | `{` |
| `,` | `}` |
| `o` | `^` |
| `l` | `_` |
| `;` | `\\` |

## Installation

### 1. System Dependencies (Linux)
You need several system libraries for audio capture and global keyboard monitoring.

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev libx11-dev libxi-dev libxtst-dev clang pkg-config
```

### 2. Whisper Model
Download a GGML-compatible Whisper model (e.g., `base.en`). You can find them on Hugging Face (e.g., [ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp)).

Place the `.bin` file in a `models/` directory or set the path via environment variable:
```bash
export WHISPER_MODEL_PATH="path/to/ggml-base.en.bin"
```

### 3. Gemini API Key
Obtain a free API key from [Google AI Studio](https://aistudio.google.com/).
```bash
export GEMINI_API_KEY="your-api-key-here"
```

## Usage
1. Run the application:
   ```bash
   cargo run --release
   ```
2. Dictate your math formula while using the structural keys listed above.
3. Press `Ctrl+C` to stop recording and trigger the LaTeX generation.
4. The result will be printed to the console and saved in `dictation_log.txt`.

## Build Notes
- **Whisper-RS**: This crate links against `whisper.cpp`. The build script handles most of this, but ensure you have `clang` and `llvm` installed.
- **Keyboard Hook**: The `rdev` crate requires permissions to listen to global events. On some Linux setups, you might need to add your user to the `input` group or run as root.
