package io.parity.signer.ui.theme

import androidx.compose.material.Text
import androidx.compose.material.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp
import io.parity.signer.R

private val InterFont = Font(R.font.inter)
private val RobotoFont = Font(R.font.robotomono_regular)
//private val RobotoFontThin = Font(R.font.robotomono_thin)
private val InterFontFamily = FontFamily(InterFont)
private val RobotoFontFamily = FontFamily(RobotoFont)//, RobotoFontThin)

// Set of Material typography styles to start with
val Typography = Typography(
	h1 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(700),
		fontSize = 19.sp
	),
	h4 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(400),
		fontSize = 17.sp
	),
	body1 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(400),
		fontSize = 16.sp
	),
	body2 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(400),
		fontSize = 15.sp
	),
	button = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(600),
		fontSize = 16.sp
	),
	subtitle1 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(500),
		fontSize = 15.sp
	),
	subtitle2 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(500),
		fontSize = 13.sp
	),
	overline = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight(500),
		fontSize = 12.sp
	),
)

val CryptoTypography = Typography (
	body1 = TextStyle(
		fontFamily = RobotoFontFamily,
		fontWeight = FontWeight(500),
		fontSize = 13.sp
	),
	body2 = TextStyle(
		fontFamily = RobotoFontFamily,
		fontWeight = FontWeight(300),
		fontSize = 13.sp
	)
)

//Special font for network labels
//TODO: labels could be stored in db instead!
val Web3Font = Font(R.font.web3_regular)
