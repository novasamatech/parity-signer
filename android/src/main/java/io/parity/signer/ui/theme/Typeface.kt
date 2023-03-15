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
private val RobotoMonoLightFont =
	Font(R.font.robotomono_light, FontWeight.Light)
private val RobotoMonoMediumFont =
	Font(R.font.robotomono_medium, FontWeight.Medium)
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
@Deprecated("use new SignerTypeface")
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
	),
)

/**
 * Our Typefase schema is shared with iOS and not mapped to android [androidx.compose.material.Typography] so mainly defined here
 */
object SignerTypeface {
	val TitleXl = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Bold,
		fontSize = 28.sp
	)
	val TitleL = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Bold,
		fontSize = 24.sp
	)
	val TitleM = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Bold,
		fontSize = 22.sp
	)
	val TitleS = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.SemiBold,
		fontSize = 16.sp
	)
	val LabelL = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.SemiBold,
		fontSize = 17.sp
	)
	val LabelM = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.SemiBold,
		fontSize = 16.sp
	)
	val LabelS = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.SemiBold,
		fontSize = 14.sp
	)
	val BodyL = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Normal,
		fontSize = 16.sp
	)
	val BodyM = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Normal,
		fontSize = 14.sp
	)
	val CaptionM = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Normal,
		fontSize = 12.sp
	)
	val CaptionS = TextStyle(
		fontFamily = InterFontFamily,
		fontWeight = FontWeight.Normal,
		fontSize = 10.sp
	)
}
