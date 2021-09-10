package io.parity.signer.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material.MaterialTheme
import androidx.compose.material.darkColors
import androidx.compose.material.lightColors
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

private val DarkColorPalette = darkColors(
        primary = MainColor,
        primaryVariant = MainColor,
        secondary = SecondaryColor,
				background = BackgroundAppColor,
				onSecondary = Color.White,
				onBackground = Color.White
	)

private val LightColorPalette = lightColors(
        primary = MainColor,
        primaryVariant = MainColor,
        secondary = SecondaryColor,
				background = BackgroundAppColor,
				onSecondary = Color.White,
				onBackground = Color.White

        /* Other default colors to override
    background = Color.White,
    surface = Color.White,
    onPrimary = Color.White,
    onSecondary = Color.Black,
    onBackground = Color.Black,
    onSurface = Color.Black,
    */
)

@Composable
fun ParitySignerTheme(darkTheme: Boolean = isSystemInDarkTheme(), content: @Composable() () -> Unit) {
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
