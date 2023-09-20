package io.parity.signer.screens.keysets

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.panels.BottomBar
import io.parity.signer.components.panels.BottomBarOptions
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeySetModel
import io.parity.signer.domain.KeySetsSelectModel
import io.parity.signer.domain.NetworkState
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundSystem
import io.parity.signer.ui.theme.textSecondary

/**
 * Default main screen with list Seeds/root keys
 */

@Composable
internal fun KeySetsListScreenFull(
	model: KeySetsSelectModel,
	navController: NavController,
	onSelectSeed: (seedName: String) -> Unit,
	onExposedShow: Callback,
	onNewKeySet: Callback,
	onRecoverKeySet: Callback,
	networkState: State<NetworkState?>, //for shield icon
) {
	Column(Modifier.background(MaterialTheme.colors.backgroundSystem)) {
		ScreenHeader(
			stringResource(R.string.key_sets_screem_title),
			onMenu = null,
		)
		Box(modifier = Modifier.weight(1f)) {
			if (model.keys.isNotEmpty()) {
				LazyColumn(
					contentPadding = PaddingValues(horizontal = 12.dp),
					verticalArrangement = Arrangement.spacedBy(10.dp),
				) {
					val cards = model.keys
					items(cards.size) { i ->
						KeySetItem2(model = cards[i]) {
							onSelectSeed(cards[i].seedName)
						}
						if (i == cards.lastIndex) {
							//to put elements under the button
							Spacer(modifier = Modifier.padding(bottom = 100.dp))
						}
					}
				}
			} else {
				KeySetsEmptyList()
			}
			Column(modifier = Modifier.align(Alignment.BottomCenter)) {
				ExposedIcon(
					networkState = networkState,
					onClick = onExposedShow,
					Modifier
						.align(Alignment.End)
						.padding(end = 16.dp)
				)
				PrimaryButtonWide(
					label = stringResource(R.string.key_sets_screem_add_key_button),
					modifier = Modifier
						.padding(top = 16.dp, bottom = 8.dp, start = 24.dp, end = 24.dp),
					onClicked = onNewKeySet,
				)
				SecondaryButtonWide(
					label = stringResource(R.string.add_key_set_menu_recover),
					modifier = Modifier
						.padding(top = 0.dp, bottom = 24.dp, start = 24.dp, end = 24.dp),
					withBackground = true,
					onClicked = onRecoverKeySet,
				)
			}
		}
		BottomBar(navController, BottomBarOptions.KEYS)
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
private fun PreviewKeySetsSelectScreenFull() {
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
			listOf("westend", "some"),
			3.toUInt()
		),
	)
	repeat(30) {
		keys.add(
			KeySetModel(
				"second seed name",
				PreviewData.Identicon.dotIcon,
				listOf("westend", "some"),
				3.toUInt()
			)
		)
	}
	val state = remember { mutableStateOf(NetworkState.Past) }
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetsListScreenFull(
				mockModel,
				rememberNavController(),
				{},
				{},
				{},
				{},
				state
			)
		}
	}
}

@Composable
private fun KeySetsEmptyList() {
	Column(
		modifier = Modifier
			.fillMaxHeight(1f)
			.padding(horizontal = 64.dp),
		horizontalAlignment = Alignment.CenterHorizontally
	) {
		Spacer(modifier = Modifier.weight(0.5f))
		Text(
			text = stringResource(R.string.key_sets_empty_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleM,
			textAlign = TextAlign.Center,
		)
		Text(
			text = stringResource(R.string.key_sets_empty_message),
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyL,
			textAlign = TextAlign.Center,
		)
		//space for button to make text in the center of the rest of screen
		Spacer(modifier = Modifier.padding(top = (56 + 24 + 24).dp))
		Spacer(modifier = Modifier.weight(0.5f))
	}
}


@Preview(
	name = "light", group = "few", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "few",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeySetsSelectScreenFew() {
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
	val state = remember { mutableStateOf(NetworkState.Past) }
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetsListScreenFull(
				mockModel,
				rememberNavController(),
				{},
				{},
				{},
				{},
				state
			)
		}
	}
}

@Preview(
	name = "light", group = "few", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "few",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewKeySetsSelectScreenEmpty() {
	val keys = emptyList<KeySetModel>()
	val state = remember { mutableStateOf(NetworkState.Past) }
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetsListScreenFull(
				mockModel,
				rememberNavController(),
				{},
				{},
				{},
				{},
				state
			)
		}
	}
}
