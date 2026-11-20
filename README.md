# SmileUp 🎛️

**SmileUp** is a high-impact build-up and tension VST3 audio plugin designed for electronic music production, EDM, techno, and cinematic transitions. Inspired by classic tension processors like *Endless Smile*, SmileUp simplifies complex risers and drops into a single, highly expressive master knob.

Built with **C++** and the **JUCE framework**, SmileUp delivers pristine audio quality, zero-latency bypass, and seamless native integration with modern digital audio workstations (including full right-click host automation in FL Studio, Ableton Live, Cubase, and Studio One).

---

## ✨ Features

- **Master Tension Knob (0% – 110%)**:
  - `0%`: 100% bit-transparent dry bypass with zero CPU overhead.
  - `0% – 100%`: Dynamic multi-stage build-up curve.
  - `100% – 110%`: Overdrive zone for explosive drop climax.
- **12 Calibrated Presets**:
  - **SOFT**: *Fist in the Air*, *Helium Jumping*, *Space Elevator*, *Sub Zero Chill*.
  - **MEDIUM**: *Face Melting Joy*, *Smack in the Face*, *Acid Sunrise*, *Dimension Warp*.
  - **HARD**: *Broken Teeth*, *Laser Cannon*.
  - **EXTREME**: *Dada Dynamite*, *Nuclear Drop*.
- **Multi-Stage DSP Architecture**:
  1. **Non-linear SVF Filter**: High-pass tension filter with saturation in the resonant loop to prevent harsh digital clipping.
  2. **Barberpole Continual Riser**: Subtle pitch-shimmer modulation creating an infinite rising psychoacoustic sensation.
  3. **High-Density Stereo Reverb**: 8 comb + 4 all-pass diffuser with LFO chorus modulation for massive spatial washouts.
  4. **Ping-Pong Delay**: Stereo cross-feedback delay with high-pass (200 Hz) and low-pass (5 kHz) analog conditioning.
  5. **Asymmetric Tube Saturation**: Warms harmonics and compresses dynamics under intense sweeps.
  6. **Make-up Gain & Safety Soft Clipper**: Keeps signal perceived loudness consistent while preventing digital overs.
- **Native DAW Host Context Menu**:
  - Right-clicking the knob directly triggers the host's native context menu (*"Create automation clip"*, *"Edit events"*, *"Link to controller..."* in FL Studio) without crashes or modal lockups.
- **Hardware Studio GUI**:
  - Compact 350 × 390 px vertical format.
  - Dark brushed slate chassis with 4 corner allen screws.
  - Cream convex rotary dial with luminous red progress arc and centered numeric readout.
  - Bubblegum pink preset capsule with quick navigation arrows and categorized dropdown menu.

---

## 🛠️ Building from Source

### Prerequisites
- **CMake** 3.20 or newer
- **C++17 Compiler** (MSVC 2022/2026, Clang, or GCC)
- **JUCE 7.0+** (cloned locally or as a sibling directory)

### Build Instructions (Windows)
```bash
# Configure the CMake project
cmake -B build -G "Visual Studio 17 2022" -A x64

# Compile Release VST3
cmake --build build --config Release
```

The compiled VST3 bundle will be generated in:
`build/SmileUp_artefacts/Release/VST3/SmileUp.vst3`

Copy it to your VST3 folder:
`C:\Program Files\Common Files\VST3\`

---

## 📄 License
MIT License. Created by [Miguel Carreño](https://github.com/MiguelC21).
