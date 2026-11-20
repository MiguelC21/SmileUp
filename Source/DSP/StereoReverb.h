#pragma once
#include <vector>
#include <cmath>
#include <algorithm>

class CombFilter
{
public:
    void setSize(size_t size)
    {
        buffer.assign(std::max<size_t>(1, size), 0.0f);
        index = 0;
        filterStore = 0.0f;
    }

    void reset()
    {
        std::fill(buffer.begin(), buffer.end(), 0.0f);
        index = 0;
        filterStore = 0.0f;
    }

    inline float process(float input, float feedback, float damp) noexcept
    {
        const float output = buffer[index];
        filterStore = output * (1.0f - damp) + filterStore * damp;
        buffer[index] = input + filterStore * feedback;
        if (++index >= buffer.size())
            index = 0;
        return output;
    }

private:
    std::vector<float> buffer;
    size_t index = 0;
    float filterStore = 0.0f;
};

class AllpassFilter
{
public:
    void setSize(size_t size)
    {
        buffer.assign(std::max<size_t>(1, size), 0.0f);
        index = 0;
    }

    void reset()
    {
        std::fill(buffer.begin(), buffer.end(), 0.0f);
        index = 0;
    }

    inline float process(float input) noexcept
    {
        const float bufOut = buffer[index];
        const float output = -input + bufOut;
        buffer[index] = input + bufOut * 0.55f;
        if (++index >= buffer.size())
            index = 0;
        return output;
    }

private:
    std::vector<float> buffer;
    size_t index = 0;
};

class StereoReverb
{
public:
    StereoReverb() = default;

    void prepare(double sampleRate)
    {
        sr = std::max(1000.0, sampleRate);
        const float srScale = static_cast<float>(sr / 44100.0);

        const int combTunings[8] = { 1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617 };
        const int allpassTunings[4] = { 556, 441, 341, 225 };
        const int stereoSpread = 29;

        for (int i = 0; i < 8; ++i)
        {
            comb_l[i].setSize(static_cast<size_t>(combTunings[i] * srScale));
            comb_r[i].setSize(static_cast<size_t>((combTunings[i] + stereoSpread) * srScale));
        }

        for (int i = 0; i < 4; ++i)
        {
            allpass_l[i].setSize(static_cast<size_t>(allpassTunings[i] * srScale));
            allpass_r[i].setSize(static_cast<size_t>((allpassTunings[i] + stereoSpread) * srScale));
        }

        reset();
    }

    void reset()
    {
        for (auto& c : comb_l) c.reset();
        for (auto& c : comb_r) c.reset();
        for (auto& a : allpass_l) a.reset();
        for (auto& a : allpass_r) a.reset();
        lfoPhase = 0.0f;
    }

    inline void processStereo(float in_l, float in_r,
                              float decay, float damp, float wet,
                              float& out_l, float& out_r) noexcept
    {
        if (wet <= 0.0001f)
        {
            out_l = in_l;
            out_r = in_r;
            return;
        }

        lfoPhase += 0.6f / static_cast<float>(sr);
        if (lfoPhase > 1.0f) lfoPhase -= 1.0f;
        const float lfoMod = std::sin(lfoPhase * 6.28318530717958647692f) * 0.008f;

        const float feedback = std::clamp(0.75f + decay * 0.23f + lfoMod, 0.0f, 0.985f);
        const float dampFactor = std::clamp(damp, 0.0f, 0.90f);
        const float monoIn = (in_l + in_r) * 0.5f * 0.018f;

        float sum_l = 0.0f;
        for (auto& c : comb_l)
            sum_l += c.process(monoIn, feedback, dampFactor);

        float sum_r = 0.0f;
        for (auto& c : comb_r)
            sum_r += c.process(monoIn, feedback, dampFactor);

        for (auto& a : allpass_l)
            sum_l = a.process(sum_l);

        for (auto& a : allpass_r)
            sum_r = a.process(sum_r);

        const float wet_l = sum_l * 1.15f - sum_r * 0.15f;
        const float wet_r = sum_r * 1.15f - sum_l * 0.15f;

        out_l = in_l + wet_l * wet;
        out_r = in_r + wet_r * wet;
    }

private:
    double sr = 44100.0;
    CombFilter comb_l[8];
    CombFilter comb_r[8];
    AllpassFilter allpass_l[4];
    AllpassFilter allpass_r[4];
    float lfoPhase = 0.0f;
};
