package io.parity.signer.components

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.size
import androidx.compose.material.LocalTextStyle
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.platform.LocalFontFamilyResolver
import androidx.compose.ui.text.Paragraph
import androidx.compose.ui.text.TextLayoutResult
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.*
import kotlin.math.absoluteValue

/**
 * fill all the space for text
 */
@Composable
fun AutoSizeText(
	text: String,
	modifier: Modifier = Modifier,
	acceptableError: Dp = 5.dp,
	maxFontSize: TextUnit = TextUnit.Unspecified,
	color: Color = Color.Unspecified,
	fontStyle: FontStyle? = null,
	fontWeight: FontWeight? = null,
	fontFamily: FontFamily? = null,
	letterSpacing: TextUnit = TextUnit.Unspecified,
	textDecoration: TextDecoration? = null,
	textAlign: TextAlign? = null,
	contentAlignment: Alignment? = null,
	lineHeight: TextUnit = TextUnit.Unspecified,
	maxLines: Int = Int.MAX_VALUE,
	onTextLayout: (TextLayoutResult) -> Unit = {},
	style: TextStyle = LocalTextStyle.current
) {
	val alignment: Alignment = contentAlignment ?: when (textAlign) {
		TextAlign.Left -> Alignment.TopStart
		TextAlign.Right -> Alignment.TopEnd
		TextAlign.Center -> Alignment.Center
		TextAlign.Justify -> Alignment.TopCenter
		TextAlign.Start -> Alignment.TopStart
		TextAlign.End -> Alignment.TopEnd
		else -> Alignment.TopStart
	}
	BoxWithConstraints(modifier = modifier, contentAlignment = alignment) {
		var shrunkFontSize = if (maxFontSize.isSpecified) maxFontSize else 100.sp

		val calculateIntrinsics = @Composable {
			val mergedStyle = style.merge(
				TextStyle(
					color = color,
					fontSize = shrunkFontSize,
					fontWeight = fontWeight,
					textAlign = textAlign,
					lineHeight = lineHeight,
					fontFamily = fontFamily,
					textDecoration = textDecoration,
					fontStyle = fontStyle,
					letterSpacing = letterSpacing
				)
			)
			Paragraph(
				text = text,
				style = mergedStyle,
				constraints = Constraints(maxWidth = kotlin.math.ceil(LocalDensity.current.run { maxWidth.toPx() })
					.toInt()),
				density = LocalDensity.current,
				fontFamilyResolver = LocalFontFamilyResolver.current,
				spanStyles = listOf(),
				placeholders = listOf(),
				maxLines = maxLines,
				ellipsis = false
			)
		}

		var intrinsics = calculateIntrinsics()

		val targetWidth = maxWidth - acceptableError / 2f

		check(targetWidth.isFinite || maxFontSize.isSpecified) { "maxFontSize must be specified if the target with isn't finite!" }

		if (text.isNotEmpty()) {
			with(LocalDensity.current) {
				// this loop will attempt to quickly find the correct size font by scaling it by the error
				// it only runs if the max font size isn't specified or the font must be smaller
				// minIntrinsicWidth is "The width for text if all soft wrap opportunities were taken."
				if (maxFontSize.isUnspecified || targetWidth < intrinsics.minIntrinsicWidth.toDp())
					while ((targetWidth - intrinsics.minIntrinsicWidth.toDp()).toPx().absoluteValue.toDp() > acceptableError / 2f) {
						shrunkFontSize *= targetWidth.toPx() / intrinsics.minIntrinsicWidth
						intrinsics = calculateIntrinsics()
					}
				// checks if the text fits in the bounds and scales it by 90% until it does
				while (intrinsics.didExceedMaxLines || maxHeight < intrinsics.height.toDp() || maxWidth < intrinsics.minIntrinsicWidth.toDp()) {
					shrunkFontSize *= 0.9f
					intrinsics = calculateIntrinsics()
				}
			}
		}

		if (maxFontSize.isSpecified && shrunkFontSize > maxFontSize)
			shrunkFontSize = maxFontSize

		Text(
			text = text,
			color = color,
			fontSize = shrunkFontSize,
			fontStyle = fontStyle,
			fontWeight = fontWeight,
			fontFamily = fontFamily,
			letterSpacing = letterSpacing,
			textDecoration = textDecoration,
			textAlign = textAlign,
			lineHeight = lineHeight,
			onTextLayout = onTextLayout,
			maxLines = maxLines,
			style = style
		)
	}
}


@Preview
@Composable
private fun AutoSizePreview1() {
	Box(Modifier.size(200.dp, 300.dp)) {
		AutoSizeText(text = "This is a bunch of text that will fill the box", maxFontSize = 250.sp, maxLines = 2)
	}
}

@Preview
@Composable
private fun AutoSizePreview2() {
	Box(Modifier.size(200.dp, 300.dp)) {
		AutoSizeText(text = "This is a bunch of text that will fill the box", maxFontSize = 25.sp)
	}
}

@Preview
@Composable
private fun AutoSizePreview3() {
	Box(Modifier.size(200.dp, 300.dp)) {
		AutoSizeText(text = "This is a bunch of text that will fill the box")
	}
}
