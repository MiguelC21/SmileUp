#pragma once
#include <juce_gui_basics/juce_gui_basics.h>
#include <juce_audio_processors/juce_audio_processors.h>
#include "DSP/ModMatrix.h"

class PresetSelector : public juce::Component
{
public:
    PresetSelector(juce::AudioProcessorValueTreeState& apvts)
        : treeState(apvts)
    {
        // Category Badge
        categoryLabel.setFont(juce::Font(10.0f, juce::Font::bold));
        categoryLabel.setColour(juce::Label::textColourId, juce::Colours::white);
        categoryLabel.setJustificationType(juce::Justification::centred);
        addAndMakeVisible(categoryLabel);

        // Prev Button
        prevButton.setButtonText("<");
        prevButton.onClick = [this] { cyclePreset(-1); };
        addAndMakeVisible(prevButton);

        // Next Button
        nextButton.setButtonText(">");
        nextButton.onClick = [this] { cyclePreset(1); };
        addAndMakeVisible(nextButton);

        // Center Dropdown button
        presetBoxButton.onClick = [this] { showPresetMenu(); };
        addAndMakeVisible(presetBoxButton);

        updateDisplay();
    }

    void resized() override
    {
        // Total bounds: 260 x 54
        // Category badge sits at top center
        categoryLabel.setBounds(getWidth() / 2 - 38, 0, 76, 16);

        const int boxY = 14;
        const int boxHeight = 36;
        const int arrowWidth = 32;

        prevButton.setBounds(0, boxY, arrowWidth, boxHeight);
        nextButton.setBounds(getWidth() - arrowWidth, boxY, arrowWidth, boxHeight);
        presetBoxButton.setBounds(arrowWidth + 4, boxY, getWidth() - (arrowWidth * 2) - 8, boxHeight);
    }

    void paint(juce::Graphics& g) override
    {
        // Paint Category badge background
        const auto badgeArea = categoryLabel.getBounds().toFloat();
        g.setColour(juce::Colour(0xff121214));
        g.fillRoundedRectangle(badgeArea, 4.0f);
        g.setColour(juce::Colour(0xff333333));
        g.drawRoundedRectangle(badgeArea, 4.0f, 1.0f);

        // Paint Pink Outer Pill container behind center box and buttons
        const int boxY = 14;
        const int boxHeight = 36;
        const juce::Rectangle<float> pinkArea(0, static_cast<float>(boxY),
                                              static_cast<float>(getWidth()), static_cast<float>(boxHeight));

        g.setColour(juce::Colour(0xffff4985));
        g.fillRoundedRectangle(pinkArea, 8.0f);

        g.setColour(juce::Colour(0xff1a1a1a));
        g.drawRoundedRectangle(pinkArea, 8.0f, 1.5f);
    }

    void updateDisplay()
    {
        if (auto* param = treeState.getRawParameterValue("preset"))
        {
            const int idx = std::clamp(static_cast<int>(param->load()), 0, (int)SmilePreset::NumPresets - 1);
            currentPreset = static_cast<SmilePreset>(idx);

            categoryLabel.setText(ModMatrix::getCategoryName(currentPreset), juce::dontSendNotification);
            presetBoxButton.setButtonText(juce::String(ModMatrix::getPresetName(currentPreset)));
            repaint();
        }
    }

private:
    void cyclePreset(int delta)
    {
        if (auto* param = treeState.getParameter("preset"))
        {
            int idx = static_cast<int>(currentPreset) + delta;
            if (idx < 0) idx = static_cast<int>(SmilePreset::NumPresets) - 1;
            if (idx >= static_cast<int>(SmilePreset::NumPresets)) idx = 0;

            const float normVal = param->convertTo0to1(static_cast<float>(idx));
            param->setValueNotifyingHost(normVal);
            updateDisplay();
        }
    }

    void showPresetMenu()
    {
        juce::PopupMenu menu;
        juce::String currentCategory = "";

        for (int i = 0; i < static_cast<int>(SmilePreset::NumPresets); ++i)
        {
            const auto p = static_cast<SmilePreset>(i);
            const juce::String cat = ModMatrix::getCategoryName(p);

            if (cat != currentCategory)
            {
                currentCategory = cat;
                menu.addSectionHeader(cat);
            }

            const juce::String name = ModMatrix::getPresetName(p);
            menu.addItem(i + 1, name, true, p == currentPreset);
        }

        menu.showMenuAsync(juce::PopupMenu::Options{}.withTargetComponent(&presetBoxButton),
                           [this](int result)
        {
            if (result > 0)
            {
                const int newIdx = result - 1;
                if (auto* param = treeState.getParameter("preset"))
                {
                    const float normVal = param->convertTo0to1(static_cast<float>(newIdx));
                    param->setValueNotifyingHost(normVal);
                    updateDisplay();
                }
            }
        });
    }

    // Custom Button Look
    class OrangeArrowButton : public juce::Button
    {
    public:
        OrangeArrowButton(bool isLeft) : juce::Button("arrow"), pointLeft(isLeft) {}

        void paintButton(juce::Graphics& g, bool shouldDrawButtonAsHighlighted, bool shouldDrawButtonAsDown) override
        {
            const auto bounds = getLocalBounds().toFloat().reduced(2.0f);
            
            juce::Colour baseColor = shouldDrawButtonAsDown ? juce::Colour(0xffd45800)
                                   : shouldDrawButtonAsHighlighted ? juce::Colour(0xffff9438)
                                   : juce::Colour(0xffff781f);

            g.setColour(baseColor);
            g.fillRoundedRectangle(bounds, 5.0f);

            g.setColour(juce::Colour(0xff222222));
            g.drawRoundedRectangle(bounds, 5.0f, 1.2f);

            // Draw white arrow triangle
            const float cx = bounds.getCentreX();
            const float cy = bounds.getCentreY();
            juce::Path arrow;
            if (pointLeft)
            {
                arrow.addTriangle(cx + 4.0f, cy - 6.0f,
                                  cx + 4.0f, cy + 6.0f,
                                  cx - 4.0f, cy);
            }
            else
            {
                arrow.addTriangle(cx - 4.0f, cy - 6.0f,
                                  cx - 4.0f, cy + 6.0f,
                                  cx + 4.0f, cy);
            }

            g.setColour(juce::Colours::white);
            g.fillPath(arrow);
        }

    private:
        bool pointLeft = false;
    };

    class WhiteCenterBoxButton : public juce::Button
    {
    public:
        WhiteCenterBoxButton() : juce::Button("centerBox") {}

        void paintButton(juce::Graphics& g, bool shouldDrawButtonAsHighlighted, bool shouldDrawButtonAsDown) override
        {
            const auto bounds = getLocalBounds().toFloat().reduced(1.0f);

            juce::Colour bgColor = shouldDrawButtonAsDown ? juce::Colour(0xfff0ede6)
                                 : shouldDrawButtonAsHighlighted ? juce::Colour(0xffffffff)
                                 : juce::Colour(0xfffcfbfa);

            g.setColour(bgColor);
            g.fillRoundedRectangle(bounds, 5.0f);

            g.setColour(juce::Colour(0xff222222));
            g.drawRoundedRectangle(bounds, 5.0f, 1.2f);

            // Preset text
            g.setFont(juce::Font(13.5f, juce::Font::bold));
            g.setColour(juce::Colour(0xff181818));
            
            // Text area leaving room for drop indicator
            const auto textArea = bounds.reduced(10.0f, 0.0f);
            g.drawText(getButtonText(), textArea.toNearestInt(), juce::Justification::centredLeft, true);

            // Dropdown square indicator on right
            const float indX = bounds.getRight() - 16.0f;
            const float indY = bounds.getCentreY() - 3.0f;
            g.setColour(juce::Colour(0xff666666));
            g.drawRect(indX, indY, 7.0f, 7.0f, 1.2f);
        }
    };

    juce::AudioProcessorValueTreeState& treeState;
    SmilePreset currentPreset = SmilePreset::FaceMeltingJoy;

    juce::Label categoryLabel;
    OrangeArrowButton prevButton { true };
    OrangeArrowButton nextButton { false };
    WhiteCenterBoxButton presetBoxButton;
};
