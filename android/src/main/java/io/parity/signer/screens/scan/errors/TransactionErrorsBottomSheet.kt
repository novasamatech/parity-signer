package io.parity.signer.screens.scan.errors

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.components.base.RichTextString
import io.parity.signer.ui.theme.SignerNewTheme


/**
 * todo dmitry similar to ios/PolkadotVault/Modals/Errors/ErrorBottomModalViewModel.swift:10
 *
 * todo dmitry ios/PolkadotVault/Core/Adapters/BackendNavigationAdapter.swift:48
 */


@Composable
fun TransactionErrorBottomSheet() {
	Column() {

	}
}

data class TransactionErrorModel(val title: String, val subtitle: String, val content: RichTextString)



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
private fun PreviewTransactionErrorBottomSheet() {
	SignerNewTheme {
		TransactionErrorBottomSheet(
		)
	}
}
