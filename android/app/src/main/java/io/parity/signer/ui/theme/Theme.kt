package io.parity.signer.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material.Colors
import androidx.compose.material.MaterialTheme
import androidx.compose.material.darkColors
import androidx.compose.material.lightColors
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

private val DarkColorPalette = darkColors(
	primary = Action400,
	primaryVariant = Bg000,
	secondary = Crypto400,
	secondaryVariant = Bg500,
	background = Bg100,
	surface = Bg200,
	error = BaseDanger,
	onPrimary = Text600,
	onSecondary = Text600,
	onBackground = Text600,
	onSurface = Text600,
	onError = Text600
)

private val LightColorPalette = lightColors(
	primary = Bg200,
	primaryVariant = Bg100,
	secondary = Bg600,
	secondaryVariant = Bg500,
	background = Bg200,
	surface = Bg000,
	error = BaseDanger,
	onPrimary = Text600,
	onSecondary = Crypto400,
	onBackground = Text600,
	onSurface = Text600,
	onError = Text600
)

@Composable
fun ParitySignerTheme(
	darkTheme: Boolean = isSystemInDarkTheme(),
	content: @Composable() () -> Unit
) {
	val colors = if (darkTheme) {
		DarkColorPalette
	} else {
		LightColorPalette
	}

	MaterialTheme(
		colors = colors,
		typography = Typography,
		shapes = Shapes,
		content = content
	)
}
