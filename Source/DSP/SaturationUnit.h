#pragma once
#include <cmath>
#include <algorithm>

class SaturationUnit
{
public:
    SaturationUnit() = default;

    void reset() {}

    inline void processStereo(float in_l, float in_r,
                              float drive, float mix,
                              float& out_l, float& out_r) noexcept
    {
        if (mix <= 0.0001f || drive <= 1.001f)
        {
            out_l = in_l;
            out_r = in_r;
            return;
        }

        const float d = std::max(1.0f, drive);
        const float sqrtD = std::sqrt(d);

        const float sat_l = tubeShape(in_l * d) / sqrtD;
        const float sat_r = tubeShape(in_r * d) / sqrtD;

        out_l = in_l * (1.0f - mix) + sat_l * mix;
        out_r = in_r * (1.0f - mix) + sat_r * mix;
    }

    static inline float tubeShape(float x) noexcept
    {
        if (x > 0.0f)
            return 1.0f - std::exp(-x);
        else
            return std::tanh(x);
    }

    static inline float softClip(float x) noexcept
    {
        if (x > 1.0f)
            return 1.0f - std::exp(-(x - 1.0f)) * 0.08f;
        else if (x < -1.0f)
            return -1.0f + std::exp(-(-x - 1.0f)) * 0.08f;
        else
            return x;
    }
};
