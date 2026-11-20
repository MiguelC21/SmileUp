#pragma once
#include <juce_gui_basics/juce_gui_basics.h>
#include <juce_audio_processors/juce_audio_processors.h>
#include "PluginProcessor.h"
#include "SmileKnob.h"
#include "PresetSelector.h"

class SmileUpAudioProcessorEditor : public juce::AudioProcessorEditor,
                                    public juce::Timer
{
public:
    SmileUpAudioProcessorEditor(SmileUpAudioProcessor&);
    ~SmileUpAudioProcessorEditor() override;

    void paint(juce::Graphics&) override;
    void resized() override;
    void timerCallback() override;

private:
    SmileUpAudioProcessor& audioProcessor;

    SmileKnob intensityKnob;
    std::unique_ptr<juce::AudioProcessorValueTreeState::SliderAttachment> intensityAttachment;

    PresetSelector presetSelector;
    juce::Image backgroundImage;

    void drawScrew(juce::Graphics& g, float cx, float cy, float angleRad);

    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SmileUpAudioProcessorEditor)
};
