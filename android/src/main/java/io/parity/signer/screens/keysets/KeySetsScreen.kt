package io.parity.signer.screens.keysets

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.items.KeySetItem
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.models.*
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.backgroundSystem
import io.parity.signer.uniffi.Action

/**
 * Default main screen with list Seeds/root keys
 */
@Composable
fun KeySetsScreen(
	model: KeySetsSelectModel,
	rootNavigator: Navigator,
	localNavigator: NavController,
	alertState: State<AlertState?>, //for shield icon
) {
	Column(Modifier.background(MaterialTheme.colors.backgroundSystem)) {
		ScreenHeader(
			stringResource(R.string.key_sets_screem_title),
			onMenu = { localNavigator.navigate(KeySetsNavSubgraph.homeMenu) }
		)
		Box(modifier = Modifier.weight(1f)) {
			LazyColumn(
				contentPadding = PaddingValues(horizontal = 12.dp),
				verticalArrangement = Arrangement.spacedBy(10.dp),
			) {
				val cards = model.keys
				items(cards.size) { i ->
					KeySetItem(model = cards[i]) {
						rootNavigator.navigate(Action.SELECT_SEED, cards[i].seedName)
					}
					if (i == cards.lastIndex) {
						//to put elements under the button
						Spacer(modifier = Modifier.padding(bottom = 100.dp))
					}
				}
			}
			Column(modifier = Modifier.align(Alignment.BottomCenter)) {
				ExposedIcon(
					alertState = alertState, navigator = rootNavigator,
					Modifier
						.align(Alignment.End)
						.padding(end = 16.dp)
				)
				PrimaryButtonWide(
					label = stringResource(R.string.key_sets_screem_add_key_button),
					modifier = Modifier
						.padding(top = 16.dp, bottom = 24.dp, start = 24.dp, end = 24.dp)
				) {
					rootNavigator.navigate(Action.RIGHT_BUTTON_ACTION) //new seed for this state
				}
			}
		}
		BottomBar2(rootNavigator, BottomBar2State.KEYS)
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
			PreviewData.exampleIdenticonPng,
			1.toUInt()
		),
		KeySetModel(
			"second seed name",
			PreviewData.exampleIdenticonPng,
			3.toUInt()
		),
	)
	repeat(30) {
		keys.add(
			KeySetModel(
				"second seed name",
				PreviewData.exampleIdenticonPng,
				3.toUInt()
			)
		)
	}
	val state = remember { mutableStateOf(AlertState.Past) }
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetsScreen(mockModel, EmptyNavigator(), rememberNavController(), state)
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
private fun PreviewKeySetsSelectScreenFew() {
	val keys = mutableListOf(
		KeySetModel(
			"first seed name",
			PreviewData.exampleIdenticonPng,
			1.toUInt()
		),
		KeySetModel(
			"second seed name",
			PreviewData.exampleIdenticonPng,
			3.toUInt()
		),
	)
	val state = remember { mutableStateOf(AlertState.Past) }
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetsScreen(mockModel, EmptyNavigator(), rememberNavController(), state)
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
	val state = remember { mutableStateOf(AlertState.Past) }
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetsScreen(mockModel, EmptyNavigator(), rememberNavController(), state)
		}
	}
}
