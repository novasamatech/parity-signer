package io.parity.signer.ui.theme

import androidx.compose.material.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.sp
import io.parity.signer.R

private val InterBoldFont = Font(R.font.inter_bold, FontWeight.Bold)
private val InterSemiBoldFont = Font(R.font.inter_semibold, FontWeight.SemiBold)
private val InterMediumFont = Font(R.font.inter_medium, FontWeight.Medium)
private val InterRegularFont = Font(R.font.inter_regular, FontWeight.Normal)
private val RobotoMonoLightFont = Font(R.font.robotomono_light, FontWeight.Light)
private val RobotoMonoMediumFont = Font(R.font.robotomono_medium, FontWeight.Medium)
private val InterFontFamily = FontFamily(
	InterBoldFont,
	InterSemiBoldFont,
	InterMediumFont,
	InterRegularFont
)
private val RobotoFontFamily = FontFamily(
	RobotoMonoMediumFont, RobotoMonoLightFont
)

// Set of Material typography styles to start with
val Typography = Typography(
	h1 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Bold,
		fontSize = 19.sp
	),
	h2 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.SemiBold,
		fontSize = 19.sp
	),
	h3 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.SemiBold,
		fontSize = 16.sp
	),
	h4 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Medium,
		fontSize = 16.sp
	),
	body1 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Normal,
		fontSize = 16.sp
	),
	body2 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Normal,
		fontSize = 15.sp
	),
	button = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.SemiBold,
		fontSize = 17.sp
	),
	subtitle1 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Medium,
		fontSize = 15.sp
	),
	subtitle2 = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Normal,
		fontSize = 13.sp
	),
	overline = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Medium,
		fontSize = 13.sp
	),
)

val CryptoTypography = Typography(
	body1 = TextStyle(
		fontFamily = RobotoFontFamily,
		fontWeight = FontWeight.Medium,
		fontSize = 16.sp,
		lineHeight = 1.4.em
	),
	body2 = TextStyle(
		fontFamily = RobotoFontFamily,
		fontWeight = FontWeight.Medium,
		fontSize = 12.sp,
		lineHeight = 1.4.em
	)
)

//Special font for network labels
//TODO: labels could be stored in db instead!
private val Web3Font = Font(R.font.web3_regular)
private val Web3FontFamily = FontFamily(Web3Font)

val Web3Typography = Typography(
	defaultFontFamily = Web3FontFamily,
	h4 = TextStyle(
		fontFamily = Web3FontFamily,
		fontSize = 16.sp
	)
)
