package io.parity.signer.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material.Colors
import androidx.compose.material.MaterialTheme
import androidx.compose.material.darkColors
import androidx.compose.material.lightColors
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

private val DarkColorPaletteOld = darkColors(
	primary = Color(0xFF3996EC),
	primaryVariant = Color(0xFF000000),
	secondary = Color(0xFFAEAEAE),
	secondaryVariant = Color(0xFF343434),
	background = Color(0xFF111111),
	surface = Color(0xFF1A1A1B),
	error = Color(0xFF2F2424),
	onPrimary = Color(0xFFFEFEFE),
	onSecondary = Color(0xFFFEFEFE),
	onBackground = Color(0xFFFEFEFE),
	onSurface = Color(0xFFFEFEFE),
	onError = Color(0xFFFF3B30)
)

private val LightColorPaletteOld = lightColors(
	primary = Color(0xFF4FA5F5),
	primaryVariant = Color(0xFFFFFFFF),
	secondary = Color(0xFF8F8E8E),
	secondaryVariant = Color(0xFF343434),
	background = Color(0xFFF3F4F5),
	surface = Color(0xFFFFFFFF),
	error = Color(0xFFFFD3D0),
	onPrimary = Color(0xFF000000),
	onSecondary = Color(0xFF000000),
	onBackground = Color(0xFF000000),
	onSurface = Color(0xFF000000),
	onError = Color(0xFFFF3B30)
)

@Composable
fun SignerOldTheme(
	darkTheme: Boolean = isSystemInDarkTheme(),
	content: @Composable () -> Unit
) {
	val colors = if (darkTheme) DarkColorPaletteOld else LightColorPaletteOld

	MaterialTheme(
		colors = colors,
		typography = Typography,
		shapes = Shapes,
		content = content
	)
}

private val DarkColorPaletteNew = darkColors(
	primary = Color(0xFFFFFFFF), // text and icons primary
	primaryVariant = Color(0xFF000000),
	secondary = Color(0xFFAEAEAE),
	secondaryVariant = Color(0xFF343434),
	background = Color(0xFF101015), //system background
	surface = Color(0xFF1A1A1B),
	error = Color(0xFF2F2424),
	onPrimary = Color(0xFF1E1E23),
	onSecondary = Color(0xFFFEFEFE),
	onBackground = Color(0xFFFEFEFE),
	onSurface = Color(0xFFFEFEFE),
	onError = Color(0xFFFF3B30)
)

private val LightColorPaletteNew = lightColors(
	primary = Color(0xFF000000), // text and icons primary
	primaryVariant = Color(0xFFFFFFFF),
	secondary = Color(0xFF8F8E8E),
	secondaryVariant = Color(0xFF343434),
	background = Color(0xFFF3F3F2),//system background
	surface = Color(0xFFFFFFFF),
	error = Color(0xFFFFD3D0),
	onPrimary = Color(0xFF000000),
	onSecondary = Color(0xFFFFFFFF),
	onBackground = Color(0xFF000000),
	onSurface = Color(0xFF000000),
	onError = Color(0xFFFF3B30)
)

@Composable
fun SignerNewTheme(
	darkTheme: Boolean = isSystemInDarkTheme(),
	content: @Composable () -> Unit
) {
	val colors = if (darkTheme) DarkColorPaletteNew else LightColorPaletteNew

	MaterialTheme(
		colors = colors,
		typography = Typography,
		shapes = Shapes,
		content = content
	)
}

val Colors.fill30: Color
	get() = if (isLight) Color(0x4D000000) else Color(0x4DFFFFFF)

val Colors.fill24: Color
	get() = if (isLight) Color(0x3D000000) else Color(0x3DFFFFFF)

val Colors.fill18: Color
	get() = if (isLight) Color(0x2E000000) else Color(0x2EFFFFFF)

val Colors.fill12: Color
	get() = if (isLight) Color(0x1F000000) else Color(0x1FFFFFFF)

val Colors.fill6: Color
	get() = if (isLight) Color(0x0F000000) else Color(0x0FFFFFFF)


val Colors.backgroundPrimary: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF1E1E23)

val Colors.backgroundSecondary: Color
	get() = if (isLight) Color(0xFFFFFFFF) else Color(0xFF2C2C30)

val Colors.textSecondary: Color
	get() = if (isLight) Color(0xA8000000) else Color(0xB0FFFFFF)

val Colors.textTertiary: Color
	get() = if (isLight) Color(0x73000000) else Color(0x7AFFFFFF)
