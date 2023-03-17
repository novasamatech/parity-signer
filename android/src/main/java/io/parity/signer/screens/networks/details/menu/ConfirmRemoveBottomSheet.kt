package io.parity.signer.screens.networks.details.menu

import android.content.res.Configuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetConfirmDialog
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun ConfirmRemoveNetworkBottomSheet(
	onCancel: Callback,
	onRemoveNetwork: Callback,
) {
	BottomSheetConfirmDialog(
		title = stringResource(R.string.network_details_remove_confirm_title),
		message = stringResource(R.string.network_details_remove_confirm_description),
		ctaLabel = stringResource(R.string.remove_key_set_confirm_cta),
		onCancel = onCancel,
		onCta = onRemoveNetwork,
	)
}


@Composable
fun ConfirmRemoveMetadataBottomSheet(
	onCancel: Callback,
	onRemoveMetadata: Callback,
) {
	BottomSheetConfirmDialog(
		title = stringResource(R.string.network_details_remove_metadata_confirm_title),
		message = stringResource(R.string.network_details_remove_metadata_confirm_description),
		ctaLabel = stringResource(R.string.remove_key_set_confirm_cta),
		onCancel = onCancel,
		onCta = onRemoveMetadata,
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
private fun PreviewConfirmRemoveNetworkBottomSheet() {
	SignerNewTheme {
		ConfirmRemoveNetworkBottomSheet(
			{}, {},
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
private fun PreviewConfirmRemoveMetadataBottomSheet() {
	SignerNewTheme {
		ConfirmRemoveMetadataBottomSheet(
			{}, {},
		)
	}
}
