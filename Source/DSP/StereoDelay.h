#pragma once
#include <vector>
#include <cmath>
#include <algorithm>

class StereoDelay
{
public:
    StereoDelay() = default;

    void prepare(double sampleRate)
    {
        sr = std::max(1000.0, sampleRate);
        const size_t maxSamples = static_cast<size_t>(sr * 3.0);
        buffer_l.assign(maxSamples, 0.0f);
        buffer_r.assign(maxSamples, 0.0f);
        reset();
    }

    void reset()
    {
        std::fill(buffer_l.begin(), buffer_l.end(), 0.0f);
        std::fill(buffer_r.begin(), buffer_r.end(), 0.0f);
        writePos = 0;
        lp_state_l = 0.0f;
        lp_state_r = 0.0f;
        hp_state_l = 0.0f;
        hp_state_r = 0.0f;
    }

    inline void processStereo(float in_l, float in_r,
                              float time_sec_l, float time_sec_r,
                              float feedback, float wet,
                              float& out_l, float& out_r) noexcept
    {
        if (wet <= 0.0001f || buffer_l.empty())
        {
            out_l = in_l;
            out_r = in_r;
            return;
        }

        const size_t bufLen = buffer_l.size();
        const size_t delay_s_l = std::clamp(static_cast<size_t>(time_sec_l * sr), (size_t)1, bufLen - 1);
        const size_t delay_s_r = std::clamp(static_cast<size_t>(time_sec_r * sr), (size_t)1, bufLen - 1);

        const size_t read_l = (writePos + bufLen - delay_s_l) % bufLen;
        const size_t read_r = (writePos + bufLen - delay_s_r) % bufLen;

        const float tap_l = buffer_l[read_l];
        const float tap_r = buffer_r[read_r];

        // High-pass filter at ~200 Hz
        hp_state_l = hp_state_l * 0.96f + tap_l * 0.04f;
        hp_state_r = hp_state_r * 0.96f + tap_r * 0.04f;
        const float hp_tap_l = tap_l - hp_state_l;
        const float hp_tap_r = tap_r - hp_state_r;

        // Low-pass filter at ~5 kHz
        lp_state_l = lp_state_l * 0.35f + hp_tap_l * 0.65f;
        lp_state_r = lp_state_r * 0.35f + hp_tap_r * 0.65f;

        const float fb = std::clamp(feedback, 0.0f, 0.94f);
        // Ping-pong cross feedback with soft tanh saturation
        const float next_l = std::tanh(in_l + lp_state_r * fb);
        const float next_r = std::tanh(in_r + lp_state_l * fb);

        buffer_l[writePos] = next_l;
        buffer_r[writePos] = next_r;

        writePos = (writePos + 1) % bufLen;

        out_l = in_l + tap_l * wet;
        out_r = in_r + tap_r * wet;
    }

private:
    double sr = 44100.0;
    std::vector<float> buffer_l;
    std::vector<float> buffer_r;
    size_t writePos = 0;
    float lp_state_l = 0.0f, lp_state_r = 0.0f;
    float hp_state_l = 0.0f, hp_state_r = 0.0f;
};
