#include "PluginProcessor.h"
#include "PluginEditor.h"
#include <BinaryData.h>

SmileUpAudioProcessorEditor::SmileUpAudioProcessorEditor(SmileUpAudioProcessor& p)
    : AudioProcessorEditor(&p),
      audioProcessor(p),
      intensityKnob(p.getIntensityParam()),
      presetSelector(p.getAPVTS())
{
    setSize(350, 390);

    // Try loading embedded background texture
    if (BinaryData::background_jpgSize > 0)
    {
        backgroundImage = juce::ImageFileFormat::loadFrom(BinaryData::background_jpg, (size_t)BinaryData::background_jpgSize);
    }

    // Intensity Knob setup
    addAndMakeVisible(intensityKnob);
    intensityAttachment = std::make_unique<juce::AudioProcessorValueTreeState::SliderAttachment>(
        p.getAPVTS(), "intensity", intensityKnob
    );

    // Preset selector setup
    addAndMakeVisible(presetSelector);

    startTimerHz(30);
}

SmileUpAudioProcessorEditor::~SmileUpAudioProcessorEditor()
{
    stopTimer();
    intensityAttachment.reset();
}

void SmileUpAudioProcessorEditor::timerCallback()
{
    presetSelector.updateDisplay();
}

void SmileUpAudioProcessorEditor::resized()
{
    // Knob placed in upper middle
    intensityKnob.setBounds(getWidth() / 2 - 75, 84, 150, 150);

    // Preset selector placed lower
    presetSelector.setBounds(getWidth() / 2 - 130, 276, 260, 56);
}

void SmileUpAudioProcessorEditor::drawScrew(juce::Graphics& g, float cx, float cy, float angleRad)
{
    const float r = 5.5f;

    // Outer dark groove
    g.setColour(juce::Colour(0xff151518));
    g.fillEllipse(cx - r - 1.0f, cy - r - 1.0f, (r + 1.0f) * 2.0f, (r + 1.0f) * 2.0f);

    // Metallic head
    juce::ColourGradient grad(
        juce::Colour(0xff6e737c), cx - 3.0f, cy - 3.0f,
        juce::Colour(0xff2d3137), cx + 3.0f, cy + 3.0f,
        false
    );
    g.setGradientFill(grad);
    g.fillEllipse(cx - r, cy - r, r * 2.0f, r * 2.0f);

    g.setColour(juce::Colour(0xff1d2024));
    g.drawEllipse(cx - r, cy - r, r * 2.0f, r * 2.0f, 0.8f);

    // Screw slot
    const float slotLen = r - 1.2f;
    const float dx = slotLen * std::cos(angleRad);
    const float dy = slotLen * std::sin(angleRad);
    g.setColour(juce::Colour(0xff16171a));
    g.drawLine(cx - dx, cy - dy, cx + dx, cy + dy, 1.4f);
}

void SmileUpAudioProcessorEditor::paint(juce::Graphics& g)
{
    // 1. Background
    if (backgroundImage.isValid())
    {
        g.drawImage(backgroundImage, getLocalBounds().toFloat(),
                    juce::RectanglePlacement::stretchToFit);
    }
    else
    {
        // Procedural brushed dark slate
        juce::ColourGradient bgGrad(
            juce::Colour(0xff292b30), 0.0f, 0.0f,
            juce::Colour(0xff18191c), 0.0f, static_cast<float>(getHeight()),
            false
        );
        g.setGradientFill(bgGrad);
        g.fillAll();
    }

    // 2. Chassis Screws in 4 corners
    drawScrew(g, 18.0f, 18.0f, 0.75f);
    drawScrew(g, static_cast<float>(getWidth()) - 18.0f, 18.0f, 2.30f);
    drawScrew(g, 18.0f, static_cast<float>(getHeight()) - 18.0f, 1.10f);
    drawScrew(g, static_cast<float>(getWidth()) - 18.0f, static_cast<float>(getHeight()) - 18.0f, 0.40f);

    // 3. Top Title: "SMILE UP"
    g.setFont(juce::Font(21.0f, juce::Font::bold));
    g.setColour(juce::Colours::white);
    g.drawText("SMILE UP", 0, 22, getWidth(), 24, juce::Justification::centred, false);

    // 4. Red Smile Arc beneath Title
    juce::Path smileArc;
    const float smileCenterX = static_cast<float>(getWidth()) * 0.5f;
    smileArc.startNewSubPath(smileCenterX - 24.0f, 49.0f);
    smileArc.quadraticTo(smileCenterX, 57.0f, smileCenterX + 24.0f, 49.0f);
    g.setColour(juce::Colour(0xffff3b4e));
    g.strokePath(smileArc, juce::PathStrokeType(2.8f, juce::PathStrokeType::curved, juce::PathStrokeType::rounded));

    // 5. "INTENSITY" label
    g.setFont(juce::Font(11.5f, juce::Font::bold));
    g.setColour(juce::Colour(0xff9e9ea3));
    g.drawText("INTENSITY", 0, 246, getWidth(), 16, juce::Justification::centred, false);
}
