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
	pink300, pink500, grey, green, dark_green
}

data class UnknownNetworkColorDrawable(val background: Color, val text: Color)


@Composable
fun UnknownNetworkColors.toUnknownNetworkColorsDrawable(): UnknownNetworkColorDrawable {
	val accentForegroundText = Color.White
	val accentForegroundTextAlt = Color.Black
	return when (this) {
		UnknownNetworkColors.pink300 ->  UnknownNetworkColorDrawable(background = MaterialTheme.colors.pink300, text = accentForegroundText)
		UnknownNetworkColors.pink500 -> UnknownNetworkColorDrawable(background = MaterialTheme.colors.pink500, text = accentForegroundText)
		UnknownNetworkColors.grey -> TODO() //todo dmitry finish
		UnknownNetworkColors.green -> TODO()
		UnknownNetworkColors.dark_green -> TODO()
	}
}




//todo dmitry ios colors in ios/PolkadotVault/Components/Image/UnknownNetworkColorsGenerator.swift:44
