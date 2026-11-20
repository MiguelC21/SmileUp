#pragma once
#include <cmath>
#include <algorithm>

enum class SmilePreset
{
    FaceMeltingJoy = 0,
    SmackInTheFace,
    FistInTheAir,
    HeliumJumping,
    BrokenTeeth,
    DadaDynamite,
    SpaceElevator,
    AcidSunrise,
    LaserCannon,
    NuclearDrop,
    SubZeroChill,
    DimensionWarp,
    NumPresets
};

struct DSPTargetParams
{
    float cutoff_hz = 20.0f;
    float resonance_q = 0.707f;
    float bp_mix = 0.0f;
    float reverb_wet = 0.0f;
    float reverb_decay = 0.0f;
    float delay_wet = 0.0f;
    float delay_feedback = 0.0f;
    float sat_drive = 1.0f;
    float sat_mix = 0.0f;
    float riser_depth = 0.0f;
    float output_trim_db = 0.0f;
};

class ModMatrix
{
public:
    static const char* getPresetName(SmilePreset preset)
    {
        switch (preset)
        {
            case SmilePreset::FaceMeltingJoy: return "Face Melting Joy";
            case SmilePreset::SmackInTheFace: return "Smack in the Face";
            case SmilePreset::FistInTheAir:   return "Fist in the Air";
            case SmilePreset::HeliumJumping:  return "Helium Jumping";
            case SmilePreset::BrokenTeeth:    return "Broken Teeth";
            case SmilePreset::DadaDynamite:   return "Dada Dynamite";
            case SmilePreset::SpaceElevator:  return "Space Elevator";
            case SmilePreset::AcidSunrise:    return "Acid Sunrise";
            case SmilePreset::LaserCannon:    return "Laser Cannon";
            case SmilePreset::NuclearDrop:    return "Nuclear Drop";
            case SmilePreset::SubZeroChill:   return "Sub Zero Chill";
            case SmilePreset::DimensionWarp:  return "Dimension Warp";
            default: return "Face Melting Joy";
        }
    }

    static const char* getCategoryName(SmilePreset preset)
    {
        switch (preset)
        {
            case SmilePreset::FistInTheAir:
            case SmilePreset::HeliumJumping:
            case SmilePreset::SpaceElevator:
            case SmilePreset::SubZeroChill:
                return "SOFT";

            case SmilePreset::FaceMeltingJoy:
            case SmilePreset::SmackInTheFace:
            case SmilePreset::AcidSunrise:
            case SmilePreset::DimensionWarp:
                return "MEDIUM";

            case SmilePreset::BrokenTeeth:
            case SmilePreset::LaserCannon:
                return "HARD";

            case SmilePreset::DadaDynamite:
            case SmilePreset::NuclearDrop:
                return "EXTREME";

            default:
                return "MEDIUM";
        }
    }

    static DSPTargetParams computeTargets(SmilePreset preset, float intensity)
    {
        DSPTargetParams t;
        const float s = std::clamp(intensity, 0.0f, 1.10f);
        const float overdrive = (s > 1.0f) ? (s - 1.0f) * 10.0f : 0.0f;

        switch (preset)
        {
            // 1. Face Melting Joy (Anthem build-up classic)
            case SmilePreset::FaceMeltingJoy:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 1.8f) * 4200.0f + overdrive * 1600.0f;
                t.resonance_q = 0.75f + s * 2.4f + overdrive * 1.5f;
                t.bp_mix = s * 0.35f;

                t.reverb_wet = std::clamp(std::pow(s, 1.3f) * 0.80f + overdrive * 0.18f, 0.0f, 0.96f);
                t.reverb_decay = std::clamp(0.35f + s * 0.58f, 0.0f, 0.96f);

                t.delay_wet = std::clamp(std::pow(s, 1.6f) * 0.65f + overdrive * 0.20f, 0.0f, 0.82f);
                t.delay_feedback = std::clamp(0.28f + s * 0.55f, 0.0f, 0.86f);

                t.sat_drive = 1.0f + s * 2.4f + overdrive * 2.5f;
                t.sat_mix = s * 0.50f;

                t.riser_depth = s * 0.25f;
                t.output_trim_db = s * 2.5f + overdrive * 2.5f;
                break;
            }

            // 2. Smack in the Face (Aggressive barberpole riser)
            case SmilePreset::SmackInTheFace:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 1.6f) * 4800.0f + overdrive * 2000.0f;
                t.resonance_q = 0.80f + s * 3.6f + overdrive * 2.0f;
                t.bp_mix = s * 0.50f;

                t.reverb_wet = std::clamp(std::pow(s, 1.2f) * 0.75f + overdrive * 0.22f, 0.0f, 0.95f);
                t.reverb_decay = std::clamp(0.40f + s * 0.55f, 0.0f, 0.95f);

                t.delay_wet = std::clamp(std::pow(s, 1.8f) * 0.45f, 0.0f, 0.60f);
                t.delay_feedback = std::clamp(0.20f + s * 0.45f, 0.0f, 0.75f);

                t.sat_drive = 1.0f + s * 3.8f + overdrive * 3.2f;
                t.sat_mix = s * 0.65f;

                t.riser_depth = std::clamp(s * 0.80f + overdrive * 0.20f, 0.0f, 1.0f);
                t.output_trim_db = s * 3.2f + overdrive * 3.0f;
                break;
            }

            // 3. Fist in the Air (Subtle, clean HPF sweep)
            case SmilePreset::FistInTheAir:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 2.0f) * 240.0f + overdrive * 120.0f;
                t.resonance_q = 0.707f + s * 0.85f;
                t.bp_mix = 0.0f;

                t.reverb_wet = std::clamp(std::pow(s, 1.1f) * 0.60f + overdrive * 0.20f, 0.0f, 0.88f);
                t.reverb_decay = std::clamp(0.25f + s * 0.52f, 0.0f, 0.88f);

                t.delay_wet = 0.0f;
                t.delay_feedback = 0.0f;

                t.sat_drive = 1.0f + s * 0.9f + overdrive * 1.5f;
                t.sat_mix = s * 0.25f;

                t.riser_depth = 0.0f;
                t.output_trim_db = s * 1.2f + overdrive * 1.5f;
                break;
            }

            // 4. Helium Jumping (Airy ping-pong delay)
            case SmilePreset::HeliumJumping:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 1.9f) * 280.0f + overdrive * 140.0f;
                t.resonance_q = 0.72f + s * 1.2f;
                t.bp_mix = s * 0.12f;

                t.reverb_wet = std::clamp(std::pow(s, 1.2f) * 0.65f + overdrive * 0.20f, 0.0f, 0.88f);
                t.reverb_decay = std::clamp(0.30f + s * 0.55f, 0.0f, 0.90f);

                t.delay_wet = std::clamp(std::pow(s, 1.3f) * 0.65f + overdrive * 0.22f, 0.0f, 0.82f);
                t.delay_feedback = std::clamp(0.30f + s * 0.52f, 0.0f, 0.85f);

                t.sat_drive = 1.0f + s * 1.4f + overdrive * 2.0f;
                t.sat_mix = s * 0.38f;

                t.riser_depth = 0.0f;
                t.output_trim_db = s * 1.8f + overdrive * 2.0f;
                break;
            }

            // 5. Broken Teeth (Heavily driven club tension)
            case SmilePreset::BrokenTeeth:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 1.7f) * 5400.0f + overdrive * 2200.0f;
                t.resonance_q = 0.85f + s * 4.4f + overdrive * 2.5f;
                t.bp_mix = s * 0.55f;

                t.reverb_wet = std::clamp(std::pow(s, 1.3f) * 0.82f + overdrive * 0.16f, 0.0f, 0.96f);
                t.reverb_decay = std::clamp(0.42f + s * 0.54f, 0.0f, 0.98f);

                t.delay_wet = std::clamp(std::pow(s, 1.5f) * 0.68f + overdrive * 0.22f, 0.0f, 0.88f);
                t.delay_feedback = std::clamp(0.32f + s * 0.60f, 0.0f, 0.92f);

                t.sat_drive = 1.0f + s * 5.5f + overdrive * 4.5f;
                t.sat_mix = s * 0.80f;

                t.riser_depth = std::clamp(s * 0.88f + overdrive * 0.12f, 0.0f, 1.0f);
                t.output_trim_db = s * 4.0f + overdrive * 3.5f;
                break;
            }

            // 6. Dada Dynamite (Maximalist festival explosion)
            case SmilePreset::DadaDynamite:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 1.5f) * 6500.0f + overdrive * 3200.0f;
                t.resonance_q = 0.92f + s * 5.2f + overdrive * 3.5f;
                t.bp_mix = s * 0.68f;

                t.reverb_wet = std::clamp(std::pow(s, 1.1f) * 0.90f + overdrive * 0.10f, 0.0f, 0.98f);
                t.reverb_decay = std::clamp(0.50f + s * 0.48f, 0.0f, 0.99f);

                t.delay_wet = std::clamp(std::pow(s, 1.3f) * 0.78f + overdrive * 0.22f, 0.0f, 0.96f);
                t.delay_feedback = std::clamp(0.38f + s * 0.60f, 0.0f, 0.95f);

                t.sat_drive = 1.0f + s * 9.5f + overdrive * 6.5f;
                t.sat_mix = std::clamp(s * 0.98f, 0.0f, 1.0f);

                t.riser_depth = 1.0f;
                t.output_trim_db = s * 5.0f + overdrive * 4.0f;
                break;
            }

            // 7. Space Elevator (Infinite celestial wash with deep diffusion)
            case SmilePreset::SpaceElevator:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 2.2f) * 3500.0f + overdrive * 1500.0f;
                t.resonance_q = 0.707f + s * 1.5f;
                t.bp_mix = s * 0.15f;

                t.reverb_wet = std::clamp(std::pow(s, 1.1f) * 0.95f + overdrive * 0.05f, 0.0f, 0.98f);
                t.reverb_decay = std::clamp(0.55f + s * 0.43f, 0.0f, 0.99f);

                t.delay_wet = std::clamp(std::pow(s, 1.4f) * 0.50f, 0.0f, 0.70f);
                t.delay_feedback = std::clamp(0.35f + s * 0.45f, 0.0f, 0.80f);

                t.sat_drive = 1.0f + s * 1.1f + overdrive * 1.0f;
                t.sat_mix = s * 0.20f;

                t.riser_depth = s * 0.30f;
                t.output_trim_db = s * 1.5f + overdrive * 1.5f;
                break;
            }

            // 8. Acid Sunrise (Resonant 303 squelch with analog warmth)
            case SmilePreset::AcidSunrise:
            {
                t.cutoff_hz = 40.0f + std::pow(s, 1.4f) * 5800.0f + overdrive * 2500.0f;
                t.resonance_q = 1.2f + s * 6.0f + overdrive * 3.0f;
                t.bp_mix = s * 0.75f;

                t.reverb_wet = std::clamp(std::pow(s, 1.4f) * 0.60f, 0.0f, 0.85f);
                t.reverb_decay = std::clamp(0.30f + s * 0.45f, 0.0f, 0.88f);

                t.delay_wet = std::clamp(std::pow(s, 1.6f) * 0.60f + overdrive * 0.20f, 0.0f, 0.80f);
                t.delay_feedback = std::clamp(0.25f + s * 0.50f, 0.0f, 0.85f);

                t.sat_drive = 1.0f + s * 4.2f + overdrive * 3.5f;
                t.sat_mix = s * 0.70f;

                t.riser_depth = s * 0.40f;
                t.output_trim_db = s * 3.0f + overdrive * 2.5f;
                break;
            }

            // 9. Laser Cannon (Piercing focused beam with fast rise)
            case SmilePreset::LaserCannon:
            {
                t.cutoff_hz = 80.0f + std::pow(s, 1.3f) * 7200.0f + overdrive * 3000.0f;
                t.resonance_q = 1.5f + s * 7.5f + overdrive * 3.5f;
                t.bp_mix = 0.85f * s;

                t.reverb_wet = std::clamp(std::pow(s, 1.5f) * 0.50f, 0.0f, 0.75f);
                t.reverb_decay = std::clamp(0.25f + s * 0.40f, 0.0f, 0.80f);

                t.delay_wet = std::clamp(std::pow(s, 1.2f) * 0.75f + overdrive * 0.20f, 0.0f, 0.90f);
                t.delay_feedback = std::clamp(0.35f + s * 0.55f, 0.0f, 0.90f);

                t.sat_drive = 1.0f + s * 6.0f + overdrive * 4.0f;
                t.sat_mix = s * 0.75f;

                t.riser_depth = std::clamp(s * 0.95f, 0.0f, 1.0f);
                t.output_trim_db = s * 3.8f + overdrive * 3.0f;
                break;
            }

            // 10. Nuclear Drop (Overwhelming devastating energy buildup)
            case SmilePreset::NuclearDrop:
            {
                t.cutoff_hz = 30.0f + std::pow(s, 1.5f) * 6800.0f + overdrive * 3500.0f;
                t.resonance_q = 1.0f + s * 6.5f + overdrive * 4.0f;
                t.bp_mix = s * 0.70f;

                t.reverb_wet = std::clamp(std::pow(s, 1.1f) * 0.92f + overdrive * 0.08f, 0.0f, 0.98f);
                t.reverb_decay = std::clamp(0.48f + s * 0.50f, 0.0f, 0.99f);

                t.delay_wet = std::clamp(std::pow(s, 1.2f) * 0.85f + overdrive * 0.15f, 0.0f, 0.96f);
                t.delay_feedback = std::clamp(0.40f + s * 0.58f, 0.0f, 0.96f);

                t.sat_drive = 1.0f + s * 11.0f + overdrive * 8.0f;
                t.sat_mix = std::clamp(s * 1.0f, 0.0f, 1.0f);

                t.riser_depth = 1.0f;
                t.output_trim_db = s * 5.5f + overdrive * 4.5f;
                break;
            }

            // 11. Sub Zero Chill (Icy crystallizing delay and air)
            case SmilePreset::SubZeroChill:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 2.4f) * 1800.0f + overdrive * 800.0f;
                t.resonance_q = 0.707f + s * 0.60f;
                t.bp_mix = 0.0f;

                t.reverb_wet = std::clamp(std::pow(s, 1.2f) * 0.75f, 0.0f, 0.90f);
                t.reverb_decay = std::clamp(0.40f + s * 0.48f, 0.0f, 0.92f);

                t.delay_wet = std::clamp(std::pow(s, 1.1f) * 0.75f + overdrive * 0.15f, 0.0f, 0.88f);
                t.delay_feedback = std::clamp(0.35f + s * 0.50f, 0.0f, 0.86f);

                t.sat_drive = 1.0f + s * 0.8f;
                t.sat_mix = s * 0.15f;

                t.riser_depth = 0.0f;
                t.output_trim_db = s * 1.0f;
                break;
            }

            // 12. Dimension Warp (Psychedelic vortex barberpole swirl)
            case SmilePreset::DimensionWarp:
            {
                t.cutoff_hz = 20.0f + std::pow(s, 1.7f) * 4600.0f + overdrive * 1800.0f;
                t.resonance_q = 0.85f + s * 3.8f + overdrive * 2.0f;
                t.bp_mix = s * 0.45f;

                t.reverb_wet = std::clamp(std::pow(s, 1.2f) * 0.85f + overdrive * 0.12f, 0.0f, 0.95f);
                t.reverb_decay = std::clamp(0.45f + s * 0.50f, 0.0f, 0.95f);

                t.delay_wet = std::clamp(std::pow(s, 1.3f) * 0.70f + overdrive * 0.20f, 0.0f, 0.88f);
                t.delay_feedback = std::clamp(0.32f + s * 0.55f, 0.0f, 0.88f);

                t.sat_drive = 1.0f + s * 3.0f + overdrive * 2.5f;
                t.sat_mix = s * 0.55f;

                t.riser_depth = std::clamp(s * 0.90f + overdrive * 0.10f, 0.0f, 1.0f);
                t.output_trim_db = s * 2.8f + overdrive * 2.5f;
                break;
            }

            default:
                break;
        }

        return t;
    }
};
