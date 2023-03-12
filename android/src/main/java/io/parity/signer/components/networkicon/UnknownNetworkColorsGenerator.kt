package io.parity.signer.components.networkicon

import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.pink500


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
	PINK300, PINK500, GREY, GREEN_LIGHT, GREEN_DARK, BLUE, BLUE_DARK
}

data class UnknownNetworkColorDrawable(val background: Color, val text: Color)


/**
 * it's composable to be able to access theme colors.
 */
@Composable
fun UnknownNetworkColors.toUnknownNetworkColorsDrawable(): UnknownNetworkColorDrawable {
	return when (this) {
		UnknownNetworkColors.PINK300 ->  UnknownNetworkColorDrawable(background = Color(0xFFF272B6), text = Color.White)
		UnknownNetworkColors.PINK500 -> UnknownNetworkColorDrawable(background = Color(0xFFE6007A), text = Color.White)
		UnknownNetworkColors.GREY -> UnknownNetworkColorDrawable(background = Color(0xFF9E9E9E), text = Color.Black)
		UnknownNetworkColors.GREEN_LIGHT -> UnknownNetworkColorDrawable(background = Color(0xFF98DF48), text = Color.Black)
		UnknownNetworkColors.GREEN_DARK -> UnknownNetworkColorDrawable(background = Color(0xFF3F5E1C), text = Color.White)
		UnknownNetworkColors.BLUE -> UnknownNetworkColorDrawable(background = Color(0xFF656DFC), text = Color.Black)
		UnknownNetworkColors.BLUE_DARK -> UnknownNetworkColorDrawable(background = Color(0xFF031F6D), text = Color.White)
	}
}


