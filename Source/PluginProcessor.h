#pragma once
#include <juce_audio_processors/juce_audio_processors.h>
#include "DSP/SvfFilter.h"
#include "DSP/StereoReverb.h"
#include "DSP/StereoDelay.h"
#include "DSP/SaturationUnit.h"
#include "DSP/ModMatrix.h"

class SmileUpAudioProcessor : public juce::AudioProcessor
{
public:
    SmileUpAudioProcessor();
    ~SmileUpAudioProcessor() override;

    void prepareToPlay(double sampleRate, int samplesPerBlock) override;
    void releaseResources() override;

    bool isBusesLayoutSupported(const BusesLayout& layouts) const override;

    void processBlock(juce::AudioBuffer<float>&, juce::MidiBuffer&) override;

    juce::AudioProcessorEditor* createEditor() override;
    bool hasEditor() const override;

    const juce::String getName() const override;

    bool acceptsMidi() const override;
    bool producesMidi() const override;
    bool isMidiEffect() const override;
    double getTailLengthSeconds() const override;

    int getNumPrograms() override;
    int getCurrentProgram() override;
    void setCurrentProgram(int index) override;
    const juce::String getProgramName(int index) override;
    void changeProgramName(int index, const juce::String& newName) override;

    void getStateInformation(juce::MemoryBlock& destData) override;
    void setStateInformation(const void* data, int sizeInBytes) override;

    juce::AudioProcessorValueTreeState& getAPVTS() { return apvts; }
    juce::RangedAudioParameter* getIntensityParam() const { return apvts.getParameter("intensity"); }
    juce::RangedAudioParameter* getPresetParam() const { return apvts.getParameter("preset"); }

    float getPeakLevel() const { return peakMeter.load(); }

    static juce::AudioProcessorValueTreeState::ParameterLayout createParameterLayout();

private:
    juce::AudioProcessorValueTreeState apvts;
    std::atomic<float>* intensityParam = nullptr;
    std::atomic<float>* presetParam = nullptr;

    SvfFilter filter;
    StereoReverb reverb;
    StereoDelay delay;
    SaturationUnit saturator;

    float riserPhase = 0.0f;
    std::atomic<float> peakMeter { 0.0f };

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SmileUpAudioProcessor)
};
