package io.parity.signer.screens.settings.general

import android.content.res.Configuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetConfirmDialog
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
internal fun ConfirmFactorySettingsBottomSheet(
	onCancel: Callback,
	onFactoryReset: Callback,
) {
	BottomSheetConfirmDialog(
		title = stringResource(R.string.confirm_factory_reset_title),
		message = stringResource(R.string.confirm_factory_reset_message),
		ctaLabel = stringResource(R.string.remove_key_set_confirm_cta),
		onCancel = onCancel,
		isCtaDangerous = true,
		onCta = onFactoryReset,
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
		ConfirmFactorySettingsBottomSheet(
			{}, {},
		)
	}
}


