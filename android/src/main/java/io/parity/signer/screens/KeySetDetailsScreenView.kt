package io.parity.signer.screens

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircleOutline
import androidx.compose.material.icons.outlined.ExpandCircleDown
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.NetworkLogoName
import io.parity.signer.models.*
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.Action400
import io.parity.signer.ui.theme.Bg100
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.*

//todo old screen is KeyManager
/**
 * Single Seed/Key set is selected is it's details
 * For non-multiselect state
 */
@Composable
fun KeySetDetailsScreenView(
	mKeys: MKeys,
	navigator: Navigator,
	signer: SignerDataModel,
	alertState: State<AlertState?>, //for shield icon
) {
	val rootKey = mKeys.root
	val keySet = mKeys.set
	val multiselectMode = mKeys.multiselectMode
	val multiselectCount = mKeys.multiselectCount
	var offsetX by remember { mutableStateOf(0f) }

	Column {
		//header network
		Row() {
			Column() {
//				Text(text =)
			}
		}
		//divider
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier
				.clickable { navigator.navigate(Action.NETWORK_SELECTOR, "") }
				.padding(top = 3.dp, start = 12.dp, end = 12.dp)
				.background(MaterialTheme.colors.Bg100)
				.fillMaxWidth()
				.padding(top = 8.dp, start = 20.dp, end = 12.dp)
		) {
			mKeys.network.let { network ->
				NetworkLogoName(
					logo = network.logo,
					name = network.title
				)
			}
			Spacer(Modifier.width(8.dp))
			Icon(
				Icons.Outlined.ExpandCircleDown,
				"More networks",
				tint = MaterialTheme.colors.Action400
			)
			Spacer(modifier = Modifier.weight(1f))
		}
		Row(
			verticalAlignment = Alignment.CenterVertically,
			modifier = Modifier
				.padding(top = 3.dp, start = 12.dp, end = 12.dp)
				.fillMaxWidth(1f)
				.padding(horizontal = 8.dp)
		) {
			Text("DERIVED KEYS")
			Spacer(Modifier.weight(1f, true))
			IconButton(onClick = {
				if (alertState.value == AlertState.None)
					navigator.navigate(Action.NEW_KEY, "")
				else
					navigator.navigate(Action.SHIELD, "")
			}) {
				Icon(
					Icons.Default.AddCircleOutline,
					contentDescription = "New derived key",
					tint = MaterialTheme.colors.Action400
				)
			}
		}
//			KeySelector(
//				navigator.navigate,
//				{ number -> signer.increment(number, rootKey.seedName) },
//				keySet,
//				multiselectMode,
//				rootKey.seedName,
//			)
	}
}


data class KeySetDetailsViewModel(
	val set: List<MKeysCard>,
	val root: MKeysCard,
	val network: MNetworkCard,
	val multiselectMode: Boolean,
	val multiselectCount: String,
)


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
private fun PreviewKeySetDetailsScreen() {
	val keys = mutableListOf(
		KeySetViewModel(
			"first seed name",
			PreviewData.exampleIdenticon,
			1.toUInt()
		),
		KeySetViewModel(
			"second seed name",
			PreviewData.exampleIdenticon,
			3.toUInt()
		),
	)
	repeat(30) {
		keys.add(
			KeySetViewModel(
				"second seed name",
				PreviewData.exampleIdenticon,
				3.toUInt()
			)
		)
	}
	val state = remember { mutableStateOf(AlertState.None) }
	val mockModel = KeySetsSelectViewModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			KeySetsScreen(mockModel, EmptyNavigator(), state)
		}
	}
}
