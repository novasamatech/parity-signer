package io.parity.signer.components.base

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontStyle
import com.halilibo.richtext.markdown.Markdown
import com.halilibo.richtext.ui.RichText
import com.halilibo.richtext.ui.material.MaterialRichText
import io.parity.signer.screens.scan.transaction.transactionElements.RichTextString


@Composable
fun MarkdownText(
	content: RichTextString,
	modifier: Modifier = Modifier,
	//todo dmitry pass it
	color: Color = Color.Unspecified,
	fontStyle: FontStyle? = null,
	onLinkClicked: ((String) -> Unit)? = null
) {
	MaterialRichText(modifier = modifier) {
		RichText {
			Markdown(content.string, onLinkClicked)
		}
	}
}
