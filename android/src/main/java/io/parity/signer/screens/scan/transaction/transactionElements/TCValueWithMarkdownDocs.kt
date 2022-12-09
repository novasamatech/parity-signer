package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.QuestionMark
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.MarkdownText
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.uniffi.MscCall
import io.parity.signer.uniffi.MscEnumVariantName

@Composable
fun TCValueWithToogleDocs(
	payload: TCWithMarkdownDocsModel,
) {
	var showDoc by remember {
		mutableStateOf(false)
	}
	Column(
		modifier = Modifier
			.animateContentSize()
			.run {
				if (payload.docs.string.isNotEmpty()) {
					clickable { showDoc = !showDoc }
				} else {
					this
				}
			},
	) {
		Row(Modifier.fillMaxWidth(1f)) {
			TCNameValueElement(
				name = if (payload.isMethod) stringResource(R.string.transaction_field_method_call) else null,
				value = payload.methodName
			)
			if (payload.docs.string.isNotEmpty()) {
				Image(
					imageVector = Icons.Default.QuestionMark,
					contentDescription = null,
					colorFilter = ColorFilter.tint(MaterialTheme.colors.textDisabled),
					modifier = Modifier
						.padding(horizontal = 8.dp)
						.size(16.dp)
						.align(Alignment.CenterVertically)
				)
			}
		}
		if (showDoc) {
			MarkdownText(content = payload.docs,
				modifier = Modifier.padding(16.dp),
			)
//			Text(
//				text = payload.docs,
//				style = SignerTypeface.BodyL,
//				color = MaterialTheme.colors.primary,
//				modifier = Modifier.padding(16.dp),
//			)
		}
	}
}


/**
 * Local copy of shared [MscCall] amd [MscEnumVariantName] class
 */
data class TCWithMarkdownDocsModel(
	val methodName: String,
	val docs: RichTextString,
	val isMethod: Boolean,
) {
	companion object {
		fun createStub(): TCWithMarkdownDocsModel =
			//todo dmitry how do you do  Text.markdownWithFallback(value.docs) and whether preview sample with umbers is the current one?
			TCWithMarkdownDocsModel(
				"method name",
				PreviewData.exampleMarkdownDocs,
				true
			)
	}
}

/**
 * String with markdown lablels, show as rich text
 */
data class RichTextString(val string: String)
fun String.toRichTextStr() = RichTextString(this)

fun MscCall.toTransactionCallModel() = TCWithMarkdownDocsModel(
	methodName = methodName,
	docs = docs.toRichTextStr(),
	isMethod = true,
)

fun MscEnumVariantName.toTransactionCallModel() = TCWithMarkdownDocsModel(
	methodName = name,
	docs = docsEnumVariant.toRichTextStr(),
	isMethod = true,
)


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTCCall() {
	SignerNewTheme {
		Column {
			TCValueWithToogleDocs(
				TCWithMarkdownDocsModel.createStub().copy(isMethod = true)
			)
			SignerDivider()
			TCValueWithToogleDocs(
				TCWithMarkdownDocsModel.createStub().copy(isMethod = false)
			)

		}
	}
}
