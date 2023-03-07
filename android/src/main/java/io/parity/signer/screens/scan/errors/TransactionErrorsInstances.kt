package io.parity.signer.screens.scan.errors

import android.content.res.Configuration
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.*
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textTertiary

const val COMPOSE_URL_TAG_ANNOTATION = "URL"

@Composable
fun TransactionError.toBottomSheetModel(): TransactionErrorModel {
	return when (this) {
		is TransactionError.Generic -> {
			TransactionErrorModel(
				title = "Something has gone wrong.",
				subtitle = "View the details below to find out more.",
				descriptionSteps = listOf(AnnotatedString(message))
			)
		}
		is TransactionError.MetadataAlreadyAdded -> {
			TransactionErrorModel(
				title = "This $name Network $version Metadata version has already been added",
				subtitle = "Go to Settings > Networks. Tap on the network you need to check and modify the latest metadata."
			)
		}
		is TransactionError.MetadataForUnknownNetwork -> {
			TransactionErrorModel(
				title = "Please add the Network",//todo dmitry work on it!!!
				subtitle = "You’re trying to update metadata on a network that hasn’t yet been added.",
				descriptionSteps = getDescriptionForUpdateMetadata()
			)
		}
		is TransactionError.NetworkAlreadyAdded -> TransactionErrorModel(
			title = "$name Network has already been added",
			subtitle = "Go to Settings > Networks to check all the added networks.",
		)
		is TransactionError.NoMetadataForNetwork -> {
			TransactionErrorModel(
				title = "Please download the $name Network Metadata",
				subtitle = "There’s no metadata downloaded for the Unique network on what you’re trying to sign the transaction",
				descriptionSteps = getDescriptionForUpdateMetadata(),
			)
		}
		is TransactionError.OutdatedMetadata -> {
			//todo dmitry
			TransactionErrorModel(
				title = "Please update Your $name Network Metadata",
				subtitle = "Unique network's 3220 metadata is different to the 3219 data you're using in the app. Please update it to sign the transaction.",
				descriptionSteps = getDescriptionForUpdateMetadata(),
			)
		}
		is TransactionError.UnknownNetwork -> {
			TransactionErrorModel(
				title = "Please add the Network you want to Transact in",
				subtitle = "You're trying to sign a transaction in a network which has not been added",
				descriptionSteps = getDescriptionForUpdateMetadata(),
			)
		}
	}
}

@Composable
@OptIn(ExperimentalTextApi::class)
private fun getDescriptionForUpdateMetadata(): List<AnnotatedString> {
	val context = LocalContext.current
	val firstStringElement = buildAnnotatedString {
		append(stringResource(R.string.transaction_error_steps_1))
		append("\n\n")
		withStyle(SpanStyle(color = MaterialTheme.colors.pink300)) {
			withAnnotation(
				COMPOSE_URL_TAG_ANNOTATION,
				"https://${context.getString(R.string.transaction_error_steps_2_url_core_networks)}"
			) {
				append(context.getString(R.string.transaction_error_steps_2_url_core_networks))
			}
		}
		append(stringResource(R.string.transaction_error_steps_2_core_networks_description))
		append("\n\n")
		withStyle(SpanStyle(color = MaterialTheme.colors.pink300)) {
			withAnnotation(
				COMPOSE_URL_TAG_ANNOTATION,
				"https://${context.getString(R.string.transaction_error_steps_3_url_parachains)}"
			) {
				append(context.getString(R.string.transaction_error_steps_3_url_parachains))
			}
		}
		append(stringResource(R.string.transaction_error_steps_3_description_parachains))
		append("\n\n")
		withStyle(SpanStyle(color = MaterialTheme.colors.textTertiary)) {
			append(stringResource(R.string.transaction_error_steps_4_notes_for_other_networks))
		}
	}
	return listOf<AnnotatedString>(
		firstStringElement,
		AnnotatedString(stringResource(R.string.transaction_error_steps_choose_network)),
		AnnotatedString(stringResource(R.string.transaction_error_steps_scan_qr_code))
	)
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
private fun PreviewTransactionErrorGeneric() {
	SignerNewTheme {
		val errorMessage =
			"Network name and version from metadata received in `load_metadata` message already have a corresponding entry in `METATREE` tree of the Signer database. However, the received metadata is different from the one already stored in the database."
		val model =
			TransactionError.Generic(errorMessage).toBottomSheetModel()
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
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
private fun PreviewTransactionErrorUnknownNetwork() {
	SignerNewTheme {
		val model =
			TransactionError.UnknownNetwork("Westend").toBottomSheetModel()
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
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
private fun PreviewTransactionErrorNoMetadataForNetwork() {
	SignerNewTheme {
		val model =
			TransactionError.NoMetadataForNetwork("Westend").toBottomSheetModel()
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
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
private fun PreviewTransactionErrorNetworkAlreadyAdded() {
	SignerNewTheme {
		val model =
			TransactionError.NetworkAlreadyAdded("Westend").toBottomSheetModel()
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
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
private fun PreviewTransactionErrorMetadataAlreadyAdded() {
	SignerNewTheme {
		val model =
			TransactionError.MetadataAlreadyAdded("Westend").toBottomSheetModel()
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
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
private fun PreviewTransactionErrorOutdatedMetadata() {
	SignerNewTheme {
		val model =
			TransactionError.OutdatedMetadata("Westend").toBottomSheetModel()
		TransactionErrorBottomSheet(
			model = model, onOk = {}
		)
	}
}
