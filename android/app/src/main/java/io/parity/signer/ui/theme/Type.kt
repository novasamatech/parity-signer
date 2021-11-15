package io.parity.signer.ui.theme

import androidx.compose.material.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp

// Set of Material typography styles to start with
val Typography = Typography(
	defaultFontFamily = FontFamily.Monospace,
	h1 = TextStyle(
		fontFamily = FontFamily.SansSerif,
		fontWeight = FontWeight(700),
		fontStyle = FontStyle.Normal,
		fontSize = 55.sp
	),
	h2 = TextStyle(
		fontFamily = FontFamily.Monospace,
		fontWeight = FontWeight(500),
		fontStyle = FontStyle.Normal,
		fontSize = 40.sp
	),
	h3 = TextStyle(
		fontFamily = FontFamily.Monospace,
		fontWeight = FontWeight(400),
		fontStyle = FontStyle.Normal,
		fontSize = 28.sp
	),
	h4 = TextStyle(
		fontFamily = FontFamily.Monospace,
		fontWeight = FontWeight(500),
		fontStyle = FontStyle.Normal,
		fontSize = 18.sp
	),
	body1 = TextStyle(
		fontFamily = FontFamily.Monospace,
		fontWeight = FontWeight(400),
		fontStyle = FontStyle.Normal,
		fontSize = 16.sp
	),
	body2 = TextStyle(
		fontFamily = FontFamily.SansSerif,
		fontWeight = FontWeight(400),
		fontStyle = FontStyle.Normal,
		fontSize = 14.sp
	),
	button = TextStyle(
		fontFamily = FontFamily.Monospace,
		fontWeight = FontWeight(400),
		fontStyle = FontStyle.Normal,
		fontSize = 16.sp
	),
	subtitle1 = TextStyle(
		fontFamily = FontFamily.SansSerif,
		fontWeight = FontWeight(500),
		fontStyle = FontStyle.Normal,
		fontSize = 24.sp
	),
	subtitle2 = TextStyle(
		fontFamily = FontFamily.SansSerif,
		fontWeight = FontWeight(400),
		fontStyle = FontStyle.Italic,
		fontSize = 20.sp
	),
	overline = TextStyle(
		fontFamily = FontFamily.SansSerif,
		fontWeight = FontWeight(500),
		fontStyle = FontStyle.Normal,
		fontSize = 12.sp
	),
)
