package io.parity.signer.screens.scan.errors

import android.content.res.Configuration
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.text.*
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textTertiary

const val COMPOSE_URL_TAG_ANNOTATION = "URL"

@OptIn(ExperimentalTextApi::class)
@Composable
fun TransactionError.toBottomSheetModel(): TransactionErrorModel {
	return when (this) {
		is TransactionError.Generic -> TODO()
		is TransactionError.MetadataAlreadyAdded -> TODO()
		is TransactionError.MetadataForUnknownNetwork -> {
			val firstStringElement: AnnotatedString = buildAnnotatedString {
				append("Go to Portal where your network is stored")
				append("\n\n")
				withStyle(SpanStyle(color = MaterialTheme.colors.pink300)) {
					withAnnotation(COMPOSE_URL_TAG_ANNOTATION, "https://metadata.parity.io") {
						append("metadata.parity.io")
					}
				}
				append(" for Polkadot, Kusama, and Westend")
				append("\n\n")
				withStyle(SpanStyle(color = MaterialTheme.colors.pink300)) {
					withAnnotation(COMPOSE_URL_TAG_ANNOTATION, "https://metadata.novasama.io") {
					append("metadata.novasama.io")
				}
				}
				append(" for Parachains and Solochains")
				append("\n\n")
				withStyle(SpanStyle(color = MaterialTheme.colors.textTertiary)) {
					append("ask network's developers directly if you haven’t found it in the portals")
				}
			}

			TransactionErrorModel(
				title = "Please add the Network",//todo dmitry work on it!!!
				subtitle = "You’re trying to update metadata on a network that hasn’t yet been added.",
				descriptionSteps = listOf<AnnotatedString>(
					firstStringElement,
					AnnotatedString("Choose the network you need"),
					AnnotatedString("Scan \"Chain spec\" QR to add the missing network")
				)
			)
		}
		is TransactionError.NetworkAlreadyAdded -> TransactionErrorModel(
			title = "Unique Network has already been added",
			subtitle = "Go to Settings > Networks to check all the added networks.",
		)
		is TransactionError.NoMetadataForNetwork -> TODO()
		is TransactionError.OutdatedMetadata -> TODO()
		is TransactionError.UnknownNetwork -> TODO()
	}
}

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
private fun PreviewTransactionErrorMetadataForUnknownNetwork() {
	SignerNewTheme {
		val model =
			TransactionError.MetadataForUnknownNetwork("Westend").toBottomSheetModel()
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
	}
}
