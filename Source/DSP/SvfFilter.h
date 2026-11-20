#pragma once
#include <cmath>
#include <algorithm>

class SvfFilter
{
public:
    SvfFilter() = default;

    void prepare(double sampleRate)
    {
        sr = std::max(1000.0, sampleRate);
        reset();
    }

    void reset()
    {
        s1_l = 0.0f;
        s2_l = 0.0f;
        s1_r = 0.0f;
        s2_r = 0.0f;
    }

    inline void processStereo(float in_l, float in_r,
                              float cutoff_hz, float resonance_q, float bp_mix,
                              float& out_l, float& out_r) noexcept
    {
        const float cutoff = std::clamp(cutoff_hz, 20.0f, static_cast<float>(sr * 0.44));
        const float q = std::clamp(resonance_q, 0.5f, 12.0f);

        // Prewarp bilinear transform
        const float g = std::tan(3.14159265358979323846f * cutoff / static_cast<float>(sr));
        const float k = 1.0f / q;

        // Left Channel
        const float hp_l = (in_l - s1_l * (2.0f * g + k) - s2_l) / (1.0f + 2.0f * g * k + g * g);
        const float bp_l = std::tanh(g * hp_l + s1_l); // Analog saturation in resonance loop
        const float lp_l = g * bp_l + s2_l;

        s1_l = g * hp_l + bp_l;
        s2_l = g * bp_l + lp_l;

        if (std::abs(s1_l) < 1e-18f) s1_l = 0.0f;
        if (std::abs(s2_l) < 1e-18f) s2_l = 0.0f;

        out_l = hp_l * (1.0f - bp_mix) + bp_l * bp_mix;

        // Right Channel
        const float hp_r = (in_r - s1_r * (2.0f * g + k) - s2_r) / (1.0f + 2.0f * g * k + g * g);
        const float bp_r = std::tanh(g * hp_r + s1_r);
        const float lp_r = g * bp_r + s2_r;

        s1_r = g * hp_r + bp_r;
        s2_r = g * bp_r + lp_r;

        if (std::abs(s1_r) < 1e-18f) s1_r = 0.0f;
        if (std::abs(s2_r) < 1e-18f) s2_r = 0.0f;

        out_r = hp_r * (1.0f - bp_mix) + bp_r * bp_mix;
    }

private:
    double sr = 44100.0;
    float s1_l = 0.0f, s2_l = 0.0f;
    float s1_r = 0.0f, s2_r = 0.0f;
};
