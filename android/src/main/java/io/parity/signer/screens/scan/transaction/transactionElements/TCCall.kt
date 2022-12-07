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
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.textDisabled
import io.parity.signer.uniffi.MscCall

@Composable
fun TCCall(
	payload: TCCallModel,
) {
	var showDoc by remember {
		mutableStateOf(false)
	}
	Column(
		modifier = Modifier
			.animateContentSize()
			.run {
				if (payload.docs.isNotEmpty()) {
					clickable { showDoc = !showDoc }
				} else {
					this
				}
			},
	) {
		Row(Modifier.fillMaxWidth(1f)) {
			TCNameValueElement(
				name = stringResource(R.string.transaction_field_method_call),
				value = payload.methodName
			)
			if (payload.docs.isNotEmpty()) {
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
			Text(
				text = payload.docs,
				style = SignerTypeface.BodyL,
				color = MaterialTheme.colors.primary
			)
		}
	}
}


/**
 * Local copy of shared [MscCall] class
 */
data class TCCallModel(
	val methodName: String,
	val docs: String
) {
	companion object {
		fun createStub(): TCCallModel =
			//todo dmitry how do you do  Text.markdownWithFallback(value.docs) and whether preview sample with umbers is the current one?
			TCCallModel("method name", PreviewData.exampleMarkdownDocs)
	}
}

fun MscCall.toTransactionCallModel() = TCCallModel(
	methodName = methodName,
	docs = docs,
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
			TCCall(TCCallModel.createStub())
//			SignerDivider()
		}
	}
}
