package io.parity.signer.screens.keysetdetails.seedselectmenu

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetModel
import io.parity.signer.domain.KeySetsListModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme


@Composable
fun SeedSelectMenuView(
	keySetsListModel: KeySetsListModel,
	selectedSeed: String,
	onSelectSeed: (String) -> Unit,
	onNewKeySet: Callback,
	onRecoverKeySet: Callback,
	onClose: Callback,
) {
	Column {
		//todo Dmitry scrollable?

		BottomSheetHeader(
			title = stringResource(R.string.key_sets_screem_title),
			onCloseClicked = onClose
		)
		SignerDivider()

		keySetsListModel.keys.forEach {
//todo dmitry items
		}
		SignerDivider()
		//todo dmitry to create add and create
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
private fun PreviewSeedSelectMenu() {
	val keys = mutableListOf(
		KeySetModel(
			"first seed name",
			PreviewData.Identicon.dotIcon,
			listOf("westend", "some"),
			1.toUInt()
		),
		KeySetModel(
			"second seed name",
			PreviewData.Identicon.dotIcon,
			listOf("kusama", "some"),
			3.toUInt()
		),
	)
	val mockModel = KeySetsListModel(keys)
	SignerNewTheme {
		SeedSelectMenuView(
			keySetsListModel = mockModel,
			selectedSeed = "first seed name",
			onSelectSeed = {},
			onNewKeySet = {},
			onRecoverKeySet = {},
			onClose = {}
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
private fun PreviewSeedSelectMenuALot() {
	val keys = mutableListOf(
		KeySetModel(
			"first seed name",
			PreviewData.Identicon.dotIcon,
			listOf("westend", "some"),
			1.toUInt()
		),
		KeySetModel(
			"second seed name",
			PreviewData.Identicon.dotIcon,
			listOf("kusama", "some"),
			3.toUInt()
		),
	)
	repeat(30) {
		keys.add(
			KeySetModel(
				"second seed name2",
				PreviewData.Identicon.dotIcon,
				listOf("westend", "some"),
				3.toUInt()
			)
		)
	}
	val mockModel = KeySetsListModel(keys)
	SignerNewTheme {
		SeedSelectMenuView(
			keySetsListModel = mockModel,
			selectedSeed = "second seed name",
			onSelectSeed = {},
			onNewKeySet = {},
			onRecoverKeySet = {},
			onClose = {}
		)
	}
}
