#include "PluginProcessor.h"
#include "PluginEditor.h"

juce::AudioProcessorValueTreeState::ParameterLayout SmileUpAudioProcessor::createParameterLayout()
{
    std::vector<std::unique_ptr<juce::RangedAudioParameter>> params;

    params.push_back(std::make_unique<juce::AudioParameterFloat>(
        juce::ParameterID { "intensity", 1 },
        "Intensity",
        juce::NormalisableRange<float>(0.0f, 1.10f, 0.001f),
        0.0f,
        juce::String(),
        juce::AudioProcessorParameter::genericParameter,
        [](float val, int) {
            return juce::String(static_cast<int>(std::round(val * 100.0f)));
        },
        [](const juce::String& text) {
            return text.getFloatValue() / 100.0f;
        }
    ));

    juce::StringArray presetChoices;
    for (int i = 0; i < static_cast<int>(SmilePreset::NumPresets); ++i)
    {
        presetChoices.add(ModMatrix::getPresetName(static_cast<SmilePreset>(i)));
    }

    params.push_back(std::make_unique<juce::AudioParameterChoice>(
        juce::ParameterID { "preset", 1 },
        "Preset",
        presetChoices,
        0
    ));

    return { params.begin(), params.end() };
}

SmileUpAudioProcessor::SmileUpAudioProcessor()
#ifndef JucePlugin_PreferredChannelConfigurations
    : AudioProcessor(BusesProperties()
                     .withInput("Input", juce::AudioChannelSet::stereo(), true)
                     .withOutput("Output", juce::AudioChannelSet::stereo(), true)),
#endif
      apvts(*this, nullptr, "Parameters", createParameterLayout())
{
    intensityParam = apvts.getRawParameterValue("intensity");
    presetParam = apvts.getRawParameterValue("preset");
}

SmileUpAudioProcessor::~SmileUpAudioProcessor()
{
}

const juce::String SmileUpAudioProcessor::getName() const
{
    return JucePlugin_Name;
}

bool SmileUpAudioProcessor::acceptsMidi() const { return false; }
bool SmileUpAudioProcessor::producesMidi() const { return false; }
bool SmileUpAudioProcessor::isMidiEffect() const { return false; }
double SmileUpAudioProcessor::getTailLengthSeconds() const { return 3.0; }

int SmileUpAudioProcessor::getNumPrograms() { return 1; }
int SmileUpAudioProcessor::getCurrentProgram() { return 0; }
void SmileUpAudioProcessor::setCurrentProgram(int) {}
const juce::String SmileUpAudioProcessor::getProgramName(int) { return {}; }
void SmileUpAudioProcessor::changeProgramName(int, const juce::String&) {}

void SmileUpAudioProcessor::prepareToPlay(double sampleRate, int /*samplesPerBlock*/)
{
    filter.prepare(sampleRate);
    reverb.prepare(sampleRate);
    delay.prepare(sampleRate);
    saturator.reset();
    riserPhase = 0.0f;
    peakMeter.store(0.0f);
}

void SmileUpAudioProcessor::releaseResources()
{
    filter.reset();
    reverb.reset();
    delay.reset();
    saturator.reset();
    riserPhase = 0.0f;
}

bool SmileUpAudioProcessor::isBusesLayoutSupported(const BusesLayout& layouts) const
{
    if (layouts.getMainOutputChannelSet() != juce::AudioChannelSet::stereo())
        return false;

    if (layouts.getMainInputChannelSet() != juce::AudioChannelSet::stereo())
        return false;

    return true;
}

void SmileUpAudioProcessor::processBlock(juce::AudioBuffer<float>& buffer, juce::MidiBuffer&)
{
    juce::ScopedNoDenormals noDenormals;
    const int totalNumInputChannels = getTotalNumInputChannels();
    const int totalNumOutputChannels = getTotalNumOutputChannels();

    for (int i = totalNumInputChannels; i < totalNumOutputChannels; ++i)
        buffer.clear(i, 0, buffer.getNumSamples());

    const int numSamples = buffer.getNumSamples();
    if (numSamples == 0 || totalNumInputChannels < 2)
        return;

    const float intensity = intensityParam != nullptr ? intensityParam->load() : 0.0f;
    const int presetIndex = presetParam != nullptr ? static_cast<int>(presetParam->load()) : 0;
    const SmilePreset preset = static_cast<SmilePreset>(std::clamp(presetIndex, 0, (int)SmilePreset::NumPresets - 1));

    float* channelL = buffer.getWritePointer(0);
    float* channelR = buffer.getWritePointer(1);

    // Bypass transparente si intensity es 0%
    if (intensity <= 0.0005f)
    {
        float maxPeak = 0.0f;
        for (int i = 0; i < numSamples; ++i)
        {
            const float p = std::max(std::abs(channelL[i]), std::abs(channelR[i]));
            if (p > maxPeak) maxPeak = p;
        }
        float oldPeak = peakMeter.load();
        float smoothPeak = (maxPeak > oldPeak) ? maxPeak : (oldPeak * 0.88f + maxPeak * 0.12f);
        peakMeter.store(smoothPeak);
        return;
    }

    const DSPTargetParams targets = ModMatrix::computeTargets(preset, intensity);
    const float trimLinear = juce::Decibels::decibelsToGain(targets.output_trim_db);

    const float delayTimeL = 0.30f;
    const float delayTimeR = 0.42f;

    float maxPeak = 0.0f;

    for (int i = 0; i < numSamples; ++i)
    {
        float inL = channelL[i];
        float inR = channelR[i];

        // 1. Sweeper filter (HPF + resonant BP)
        float fltL = 0.0f, fltR = 0.0f;
        filter.processStereo(inL, inR, targets.cutoff_hz, targets.resonance_q, targets.bp_mix, fltL, fltR);

        // 2. Continual Riser modulation
        float riseL = fltL, riseR = fltR;
        if (targets.riser_depth > 0.001f)
        {
            riserPhase += 0.00045f * (1.0f + intensity * 2.0f);
            if (riserPhase > 1.0f) riserPhase -= 1.0f;
            const float modAmt = std::sin(riserPhase * 6.28318530717958647692f) * 0.08f * targets.riser_depth;
            riseL = fltL * (1.0f + modAmt);
            riseR = fltR * (1.0f - modAmt);
        }

        // 3. Reverb Wash-out
        float revL = 0.0f, revR = 0.0f;
        reverb.processStereo(riseL, riseR, targets.reverb_decay, 0.22f, targets.reverb_wet, revL, revR);

        // 4. Stereo Ping-Pong Delay
        float dlyL = 0.0f, dlyR = 0.0f;
        delay.processStereo(revL, revR, delayTimeL, delayTimeR, targets.delay_feedback, targets.delay_wet, dlyL, dlyR);

        // 5. Analog Tube Saturation
        float satL = 0.0f, satR = 0.0f;
        saturator.processStereo(dlyL, dlyR, targets.sat_drive, targets.sat_mix, satL, satR);

        // 6. Make-up gain and transparent soft clip
        float outL = SaturationUnit::softClip(satL * trimLinear);
        float outR = SaturationUnit::softClip(satR * trimLinear);

        const float p = std::max(std::abs(outL), std::abs(outR));
        if (p > maxPeak) maxPeak = p;

        channelL[i] = outL;
        channelR[i] = outR;
    }

    float oldPeak = peakMeter.load();
    float smoothPeak = (maxPeak > oldPeak) ? maxPeak : (oldPeak * 0.88f + maxPeak * 0.12f);
    peakMeter.store(smoothPeak);
}

bool SmileUpAudioProcessor::hasEditor() const
{
    return true;
}

juce::AudioProcessorEditor* SmileUpAudioProcessor::createEditor()
{
    return new SmileUpAudioProcessorEditor(*this);
}

void SmileUpAudioProcessor::getStateInformation(juce::MemoryBlock& destData)
{
    auto state = apvts.copyState();
    std::unique_ptr<juce::XmlElement> xml(state.createXml());
    copyXmlToBinary(*xml, destData);
}

void SmileUpAudioProcessor::setStateInformation(const void* data, int sizeInBytes)
{
    std::unique_ptr<juce::XmlElement> xmlState(getXmlFromBinary(data, sizeInBytes));
    if (xmlState.get() != nullptr && xmlState->hasTagName(apvts.state.getType()))
    {
        apvts.replaceState(juce::ValueTree::fromXml(*xmlState));
    }
}

juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter()
{
    return new SmileUpAudioProcessor();
}
