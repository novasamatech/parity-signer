package io.parity.signer.screens.scan.errors

import android.content.res.Configuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun TransactionError.toBottomSheetModel(): TransactionErrorModel {
	return when (this) {
		is TransactionError.Generic -> TODO()
		is TransactionError.MetadataAlreadyAdded -> TODO()
		is TransactionError.MetadataForUnknownNetwork -> TransactionErrorModel(
			title = "Please add the Network",//todo dmitry work on it!!!
			subtitle = "You’re trying to update metadata on a network that hasn’t yet been added.",
			descriptionSteps = listOf(
				"Go to Portal where your network is stored\n\n" +
					"parity.metadata.io for Polkadot, Kusama, and Westend\n\n" +
					"metadata.novasama.io for Parachains and Solochains \n\n" +
					"ask network's developers directly \n" +
					"if you haven’t found it in the portals",
				"Choose the network you need ",
				"Scan \"Chain spec\" QR to add the missing network"
			)
		)
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

	val model = TransactionError.MetadataForUnknownNetwork("Westend").toBottomSheetModel()
	SignerNewTheme {
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
	}
}
