cmake_minimum_required(VERSION 3.20)

project(SmileUp VERSION 1.0.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# JUCE dependency: use local sibling directory if available, otherwise fetch automatically
if (EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/../JUCE/CMakeLists.txt")
    add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/../JUCE" juce)
else()
    include(FetchContent)
    FetchContent_Declare(
        JUCE
        GIT_REPOSITORY https://github.com/juce-framework/JUCE.git
        GIT_TAG 7.0.12
        GIT_SHALLOW TRUE
    )
    FetchContent_MakeAvailable(JUCE)
endif()

# Embed binary assets (background texture)
juce_add_binary_data(SmileUpData
    SOURCES
        assets/background.jpg
)

juce_add_plugin(SmileUp
    COMPANY_NAME "SmileUp Audio"
    BUNDLE_ID "com.smileup.smileup"
    PLUGIN_MANUFACTURER_CODE SmUp
    PLUGIN_CODE SmUp
    FORMATS VST3
    PRODUCT_NAME "SmileUp"
    VST3_CATEGORIES Fx Filter Delay Reverb Distortion
    IS_SYNTH FALSE
    NEEDS_MIDI_INPUT FALSE
    NEEDS_MIDI_OUTPUT FALSE
    IS_MIDI_EFFECT FALSE
    EDITOR_WANTS_KEYBOARD_FOCUS FALSE
)

target_sources(SmileUp PRIVATE
    Source/PluginProcessor.h
    Source/PluginProcessor.cpp
    Source/PluginEditor.h
    Source/PluginEditor.cpp
    Source/SmileKnob.h
    Source/PresetSelector.h
    Source/DSP/SvfFilter.h
    Source/DSP/StereoReverb.h
    Source/DSP/StereoDelay.h
    Source/DSP/SaturationUnit.h
    Source/DSP/ModMatrix.h
)

target_compile_definitions(SmileUp PRIVATE
    JUCE_WEB_BROWSER=0
    JUCE_USE_CURL=0
    JUCE_VST3_CAN_REPLACE_VST2=0
)

target_link_libraries(SmileUp PRIVATE
    SmileUpData
    juce::juce_audio_utils
    juce::juce_audio_processors
    juce::juce_gui_basics
    juce::juce_graphics
    juce::juce_data_structures
    juce::juce_events
    juce::juce_core
    juce::juce_recommended_config_flags
    juce::juce_recommended_warning_flags
)
