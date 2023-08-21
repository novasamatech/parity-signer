package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

internal fun colorTheme(hue: Float, config: Config) : List<String> {
    return listOf(
        // Dark gray
        Color.hsl(0f, 0f, config.grayscaleLightness(0f)),
        // Mid color
        Color.correctedHsl(hue, config.saturation, config.colorLightness(0.5f)),
        // Light gray
        Color.hsl(0f, 0f, config.grayscaleLightness(1f)),
        // Light color
        Color.correctedHsl(hue, config.saturation, config.colorLightness(1f)),
        // Dark color
        Color.correctedHsl(hue, config.saturation, config.colorLightness(0f))
    )
}
