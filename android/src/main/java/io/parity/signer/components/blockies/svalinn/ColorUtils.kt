package io.parity.signer.components.blockies.svalinn

/**
 * Based on svalinn-kotlin project which is MIT licensed.
 */
internal fun hueToRGB(p: Float, q: Float, h: Float): Float {
    var hue = h
    if (hue < 0) hue += 1f
    if (hue > 1) hue -= 1f
    if (6 * hue < 1) return p + (q - p) * 6f * hue
    if (2 * hue < 1) return q

    return if (3 * hue < 2) p + (q - p) * 6f * (2.0f / 3.0f - hue) else p
}
