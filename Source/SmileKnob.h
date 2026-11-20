#pragma once
#include <juce_gui_basics/juce_gui_basics.h>
#include <juce_audio_processors/juce_audio_processors.h>

class SmileKnob : public juce::Slider
{
public:
    SmileKnob(juce::RangedAudioParameter* parameterToControl)
        : param(parameterToControl)
    {
        setSliderStyle(juce::Slider::RotaryVerticalDrag);
        setTextBoxStyle(juce::Slider::NoTextBox, false, 0, 0);
        setRotaryParameters(juce::MathConstants<float>::pi * 1.25f,  // 225 deg (7:30 o'clock)
                            juce::MathConstants<float>::pi * 2.75f,  // 495 deg (4:30 o'clock)
                            true);
        setRange(0.0, 1.10, 0.001);
        setDoubleClickReturnValue(true, 0.0);
    }

    void mouseDown(const juce::MouseEvent& e) override
    {
        if (e.mods.isPopupMenu())
        {
            const auto mousePos = e.getPosition();
            juce::MessageManager::callAsync([this, mousePos]
            {
                if (auto* editor = findParentComponentOfClass<juce::AudioProcessorEditor>())
                {
                    if (auto* host = editor->getHostContext())
                    {
                        if (auto menu = host->getContextMenuForParameter(param))
                        {
                            const auto posInEditor = editor->getLocalPoint(this, mousePos);
                            menu->showNativeMenu(posInEditor);
                            return;
                        }
                    }
                }

                // Fallback popup menu if not hosted in VST3 with host context
                juce::PopupMenu m;
                m.addItem("Reset to 0", [this] {
                    setValue(0.0, juce::sendNotificationAsync);
                });
                m.showMenuAsync(juce::PopupMenu::Options{}.withTargetComponent(this));
            });
            return;
        }

        juce::Slider::mouseDown(e);
    }

    void paint(juce::Graphics& g) override
    {
        const auto bounds = getLocalBounds().toFloat();
        const float centerX = bounds.getCentreX();
        const float centerY = bounds.getCentreY();

        const float startAngle = juce::MathConstants<float>::pi * 1.25f;
        const float endAngle   = juce::MathConstants<float>::pi * 2.75f;
        const float curVal = static_cast<float>(getValue());
        const float normVal = std::clamp(curVal / 1.10f, 0.0f, 1.0f);
        const float curAngle = startAngle + normVal * (endAngle - startAngle);

        // 1. Perimeter Dot Ticks (13 ticks)
        const float tickRadius = 66.0f;
        const int numTicks = 13;
        for (int i = 0; i < numTicks; ++i)
        {
            const float tNorm = static_cast<float>(i) / static_cast<float>(numTicks - 1);
            const float tAngle = startAngle + tNorm * (endAngle - startAngle);
            const float tx = centerX + tickRadius * std::cos(tAngle - juce::MathConstants<float>::halfPi);
            const float ty = centerY + tickRadius * std::sin(tAngle - juce::MathConstants<float>::halfPi);

            // Active or subtle color
            if (tAngle <= curAngle + 0.05f && curVal > 0.005f)
                g.setColour(juce::Colour(0xfff5f0e8).withAlpha(0.75f));
            else
                g.setColour(juce::Colour(0xff777777).withAlpha(0.55f));

            g.fillEllipse(tx - 2.0f, ty - 2.0f, 4.0f, 4.0f);
        }

        // 2. Track & Glowing Arc
        const float trackRadius = 57.0f;
        const float trackThickness = 6.5f;

        // Inactive background groove
        juce::Path bgTrack;
        bgTrack.addCentredArc(centerX, centerY, trackRadius, trackRadius, 0.0f,
                              startAngle, endAngle, true);
        g.setColour(juce::Colour(0xff222326));
        g.strokePath(bgTrack, juce::PathStrokeType(trackThickness, juce::PathStrokeType::curved, juce::PathStrokeType::rounded));

        // Active Red/Orange Progress Arc
        if (normVal > 0.005f)
        {
            juce::Path activeTrack;
            activeTrack.addCentredArc(centerX, centerY, trackRadius, trackRadius, 0.0f,
                                      startAngle, curAngle, true);

            // Red/coral gradient glow
            const juce::Colour redColor = (curVal > 1.0f) ? juce::Colour(0xffff2d55) : juce::Colour(0xffff3b4e);
            g.setColour(redColor);
            g.strokePath(activeTrack, juce::PathStrokeType(trackThickness, juce::PathStrokeType::curved, juce::PathStrokeType::rounded));

            // Glowing indicator dot at head of arc
            const float headX = centerX + trackRadius * std::cos(curAngle - juce::MathConstants<float>::halfPi);
            const float headY = centerY + trackRadius * std::sin(curAngle - juce::MathConstants<float>::halfPi);
            
            const juce::Colour headColor = (curVal > 1.0f) ? juce::Colour(0xffffd03b) : juce::Colour(0xffff6b78);
            g.setColour(headColor);
            g.fillEllipse(headX - 4.5f, headY - 4.5f, 9.0f, 9.0f);
        }

        // 3. Drop Shadow behind dial
        const float dialRadius = 46.0f;
        g.setColour(juce::Colours::black.withAlpha(0.40f));
        g.fillEllipse(centerX - dialRadius - 2.0f, centerY - dialRadius + 2.0f, (dialRadius + 2.0f) * 2.0f, (dialRadius + 2.0f) * 2.0f);

        // 4. Cream Convex Knob Dial
        juce::ColourGradient dialGrad(
            juce::Colour(0xfff7f2ea), centerX - 25.0f, centerY - 25.0f,
            juce::Colour(0xffdcd3c4), centerX + 25.0f, centerY + 25.0f,
            false
        );
        g.setGradientFill(dialGrad);
        g.fillEllipse(centerX - dialRadius, centerY - dialRadius, dialRadius * 2.0f, dialRadius * 2.0f);

        // Rim border
        g.setColour(juce::Colour(0xffb8ab9a));
        g.drawEllipse(centerX - dialRadius, centerY - dialRadius, dialRadius * 2.0f, dialRadius * 2.0f, 1.2f);

        // 5. Concentric Inner Plateau
        const float innerRadius = 33.0f;
        juce::ColourGradient innerGrad(
            juce::Colour(0xffded5c6), centerX - 15.0f, centerY - 15.0f,
            juce::Colour(0xffeae3d6), centerX + 15.0f, centerY + 15.0f,
            false
        );
        g.setGradientFill(innerGrad);
        g.fillEllipse(centerX - innerRadius, centerY - innerRadius, innerRadius * 2.0f, innerRadius * 2.0f);

        g.setColour(juce::Colour(0xffcfc4b3));
        g.drawEllipse(centerX - innerRadius, centerY - innerRadius, innerRadius * 2.0f, innerRadius * 2.0f, 1.0f);

        // 6. Pointer Notch Indicator
        const float notchAngle = curAngle - juce::MathConstants<float>::halfPi;
        const float nInnerX = centerX + (innerRadius - 4.0f) * std::cos(notchAngle);
        const float nInnerY = centerY + (innerRadius - 4.0f) * std::sin(notchAngle);
        const float nOuterX = centerX + (dialRadius - 2.0f) * std::cos(notchAngle);
        const float nOuterY = centerY + (dialRadius - 2.0f) * std::sin(notchAngle);

        g.setColour(juce::Colour(0xff22211e));
        g.drawLine(nInnerX, nInnerY, nOuterX, nOuterY, 3.2f);

        // 7. Centered Numeric Readout ("0" to "110")
        const int displayVal = static_cast<int>(std::round(curVal * 100.0f));
        const juce::String valStr = juce::String(displayVal);

        g.setFont(juce::Font(20.0f, juce::Font::bold));
        g.setColour(juce::Colour(0xff2a2723));
        g.drawText(valStr,
                   static_cast<int>(centerX - 30.0f), static_cast<int>(centerY - 14.0f), 60, 28,
                   juce::Justification::centred, false);
    }

private:
    juce::AudioProcessorParameter* param = nullptr;
};
