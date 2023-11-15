package io.parity.signer.screens.scan.errors

import android.content.Context
import android.content.res.Configuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.ui.theme.SignerNewTheme

const val COMPOSE_URL_TAG_ANNOTATION = "URL"

@Composable
fun TransactionError.toBottomSheetModel(): LocalErrorSheetModel {
	val context = LocalContext.current
	return toBottomSheetModel(context)
}


fun TransactionError.toBottomSheetModel(context: Context): LocalErrorSheetModel {
	return when (this) {
		is TransactionError.Generic -> {
			LocalErrorSheetModel(
				title = context.getString(R.string.transaction_error_generic_title),
				subtitle = context.getString(R.string.transaction_error_generic_subtitle),
				details = message,
			)
		}
		is TransactionError.MetadataAlreadyAdded -> {
			LocalErrorSheetModel(
				title = context.getString(
					R.string.transaction_error_metadata_already_added_title,
					name,
					version.toString(),
				),
				subtitle = context.getString(R.string.transaction_error_metadata_already_added_subtitle)
			)
		}
		is TransactionError.MetadataForUnknownNetwork -> {
			LocalErrorSheetModel(
				title = context.getString(R.string.transaction_error_meta_unknown_network_title),
				subtitle = context.getString(R.string.transaction_error_meta_unknown_network_subtitle),
				showNetworkSteps = true,
			)
		}
		is TransactionError.NetworkAlreadyAdded -> LocalErrorSheetModel(
			title = context.getString(
				R.string.transaction_error_network_already_added_title,
				name
			),
			subtitle = context.getString(R.string.transaction_error_network_already_added_subtitle),
		)
		is TransactionError.NoMetadataForNetwork -> {
			LocalErrorSheetModel(
				title = context.getString(
					R.string.transaction_error_no_metadata_for_network_title,
					name
				),
				subtitle = context.getString(R.string.transaction_error_no_metadata_for_network_subtitle),
				showNetworkSteps = true,
			)
		}
		is TransactionError.OutdatedMetadata -> {
			LocalErrorSheetModel(
				title = context.getString(
					R.string.transaction_error_outdated_metadata_title,
					name
				),
				subtitle = context.getString(R.string.transaction_error_outdated_metadata_subtitle),
				showNetworkSteps = true,
			)
		}
		is TransactionError.UnknownNetwork -> {
			LocalErrorSheetModel(
				title = context.getString(R.string.transaction_error_unknown_network_title),
				subtitle = context.getString(R.string.transaction_error_unknown_network_subtitle),
				showNetworkSteps = true,
			)
		}
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
		LocalErrorBottomSheet(
			error = model, onOk = {}
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
		LocalErrorBottomSheet(
			error = model, onOk = {}
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
			TransactionError.UnknownNetwork("Westend", "crc3322").toBottomSheetModel()
		LocalErrorBottomSheet(
			error = model, onOk = {}
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
		LocalErrorBottomSheet(
			error = model, onOk = {}
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
			TransactionError.NetworkAlreadyAdded("Westend", "crc3322").toBottomSheetModel()
		LocalErrorBottomSheet(
			error = model, onOk = {}
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
			TransactionError.MetadataAlreadyAdded("Westend", 4356u).toBottomSheetModel()
		LocalErrorBottomSheet(
			error = model, onOk = {}
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
			TransactionError.OutdatedMetadata("Westend", 3256u, 3257u).toBottomSheetModel()
		LocalErrorBottomSheet(
			error = model, onOk = {}
		)
	}
}
