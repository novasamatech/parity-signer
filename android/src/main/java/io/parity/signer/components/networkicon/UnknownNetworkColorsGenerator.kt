package io.parity.signer.components.networkicon

import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color


class UnknownNetworkColorsGenerator {
	private val mapping = mutableMapOf<String, UnknownNetworkColors>()

	private var remainingColors = UnknownNetworkColors.values().toMutableSet()

	fun getBackground(networkLogoName: String): UnknownNetworkColors {
		mapping[networkLogoName]?.let { return it }

		if (remainingColors.isEmpty()) {
			remainingColors = UnknownNetworkColors.values().toMutableSet()
		}

		val color = remainingColors.random()
		remainingColors.remove(color)
		mapping[networkLogoName] = color
		return color
	}
}

enum class UnknownNetworkColors {
	GREEN700, PURPLE600, CYAN500, LIME600, GREY600, PURPLE400, GREY500, GREY800, PINK500, PINK300
}

data class UnknownNetworkColorDrawable(val background: Color, val text: Color)


/**
 * it's composable to be able to access theme colors.
 */
@Composable
fun UnknownNetworkColors.toUnknownNetworkColorsDrawable(): UnknownNetworkColorDrawable {
	return when (this) {

		UnknownNetworkColors.GREEN700 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFF48CC81
			), text = Color.Black
		)
		UnknownNetworkColors.PURPLE600 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFF442299
			), text = Color.White
		)
		UnknownNetworkColors.CYAN500 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFF00B2FF
			), text = Color.Black
		)
		UnknownNetworkColors.LIME600 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFFBEE52E
			), text = Color.Black
		)
		UnknownNetworkColors.GREY600 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFF4C4B63
			), text = Color.White
		)
		UnknownNetworkColors.PURPLE400 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFF6D3AEE
			), text = Color.White
		)
		UnknownNetworkColors.GREY500 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFF6C6B80
			), text = Color.White
		)
		UnknownNetworkColors.GREY800 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFF201F37
			), text = Color.White
		)
		UnknownNetworkColors.PINK500 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFFE6007A
			), text = Color.White
		)
		UnknownNetworkColors.PINK300 -> UnknownNetworkColorDrawable(
			background = Color(
				0xFFF272B6
			), text = Color.White
		)
	}
}


