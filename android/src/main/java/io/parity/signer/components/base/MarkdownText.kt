package io.parity.signer.components.base

import android.content.res.Configuration
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.halilibo.richtext.markdown.Markdown
import com.halilibo.richtext.ui.RichTextStyle
import com.halilibo.richtext.ui.material.RichText
import com.halilibo.richtext.ui.string.RichTextStringStyle
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun MarkdownText(
	content: RichTextString,
	modifier: Modifier = Modifier,
	onLinkClicked: ((String) -> Unit)? = null
) {
	Surface() { // so onSurface will be used for text color otherwise it's default black
		RichText(
			modifier = modifier,
			style = RichTextStyle(
				stringStyle = RichTextStringStyle()
			)
		) {
			Markdown(content.string, onLinkClicked = onLinkClicked)
		}
	}
}

/**
 * String with markdown labels, show as rich text
 */
data class RichTextString(val string: String)

fun String.toRichTextStr() = RichTextString(this)


/**
 * not working in preview but interactive mode will show it
 */
@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewMarkdownText() {
//todo #1516 - manage dark theme
	val content =
		"Same as the [`transfer`] call, but with a check that the transfer will not kill the"
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp)) {
			MarkdownText(content = content.toRichTextStr())
		}
	}
}
