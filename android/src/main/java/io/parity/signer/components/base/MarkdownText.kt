package io.parity.signer.components.base

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.halilibo.richtext.markdown.Markdown
import com.halilibo.richtext.ui.RichTextStyle
import com.halilibo.richtext.ui.material.MaterialRichText
import com.halilibo.richtext.ui.string.RichTextStringStyle


@Composable
fun MarkdownText(
	content: RichTextString,
	modifier: Modifier = Modifier,
	onLinkClicked: ((String) -> Unit)? = null
) {
	MaterialRichText(
		modifier = modifier,
		style = RichTextStyle(
			stringStyle = RichTextStringStyle()
		)
	) {
		Markdown(content.string, onLinkClicked = onLinkClicked)
	}
}

/**
 * String with markdown labels, show as rich text
 */
data class RichTextString(val string: String)

fun String.toRichTextStr() = RichTextString(this)
