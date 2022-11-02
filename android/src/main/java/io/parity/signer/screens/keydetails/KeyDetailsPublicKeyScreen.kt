package io.parity.signer.screens.keydetails

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
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
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonBottomSheet
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.exposesecurity.ExposedIcon
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.models.*
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.uniffi.Action

/**
 * Default main screen with list Seeds/root keys
 */
//todo dmitry ios logic is in KeyDetailsPublicKeyViewModel
@Composable
fun KeyDetailsPublicKeyScreen(
	model: KeyDetailsModel,
	rootNavigator: Navigator,
	alertState: State<AlertState?>, //for shield icon
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeaderClose(
			stringResource(id = R.string.key_sets_screem_title),
			//todo dmitry add subtitle if root key
			onClose = { rootNavigator.backAction() },
			onMenu = { rootNavigator.navigate(Action.RIGHT_BUTTON_ACTION) }
		)
		Box(modifier = Modifier.weight(1f)) {
	Column() {
		//todo dmitry scrollable content for full screen
	}
			Column(modifier = Modifier.align(Alignment.BottomCenter)) {
				ExposedIcon(
					alertState = alertState, navigator = rootNavigator,
					Modifier
						.align(Alignment.End)
						.padding(end = 16.dp)
				)
			}
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
private fun PreviewKeyDetailsScreen() {
	val keys = mutableListOf(
		KeySetModel(
			"first seed name",
			PreviewData.exampleIdenticon,
			1.toUInt()
		),
		KeySetModel(
			"second seed name",
			PreviewData.exampleIdenticon,
			3.toUInt()
		),
	)
	repeat(30) {
		keys.add(
			KeySetModel(
				"second seed name",
				PreviewData.exampleIdenticon,
				3.toUInt()
			)
		)
	}
	val state = remember { mutableStateOf(AlertState.Past) }
	val mockModel = KeySetsSelectModel(keys)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			//todo dmitry check
//			KeyDetailsPublicKeyScreen(mockModel, EmptyNavigator(), rememberNavController(), state)
		}
	}
}
