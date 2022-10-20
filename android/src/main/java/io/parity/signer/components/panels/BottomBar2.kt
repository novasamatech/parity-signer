package io.parity.signer.components.panels

import android.content.res.Configuration
import androidx.annotation.DrawableRes
import androidx.annotation.StringRes
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.BottomAppBar
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.KeySetViewModel
import io.parity.signer.models.KeySetsSelectViewModel
import io.parity.signer.models.Navigator
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action

/**
 * Bar to be shown on the bottom of screen;
 * Redesigned version
 */
@Composable
fun BottomBar2(
	navigator: Navigator,
	state: BottomBar2State,
) {
	BottomAppBar(
		backgroundColor = MaterialTheme.colors.backgroundSecondary,
		elevation = 0.dp,
		modifier = Modifier.height(50.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.SpaceEvenly,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			BottomBarButton2(
				navigator = navigator,
				iconId = R.drawable.ic_key_outlined_24,
				action = Action.NAVBAR_KEYS,
				labelResId = R.string.bottom_bar_label_key_sets,
				isEnabled = state == BottomBar2State.KEYS,
			)
			BottomBarButton2(
				navigator = navigator,
				iconId = R.drawable.ic_qe_code_24,
				action = Action.NAVBAR_SCAN,
				labelResId = R.string.bottom_bar_label_scanner,
				isEnabled = state == BottomBar2State.SCANNER,
			)
			BottomBarButton2(
				navigator = navigator,
				iconId = R.drawable.ic_view_agenda_outlined_24,
				action = Action.NAVBAR_LOG,
				labelResId = R.string.bottom_bar_label_logs,
				isEnabled = state == BottomBar2State.LOGS,
			)
			BottomBarButton2(
				navigator = navigator,
				iconId = R.drawable.ic_settings_outlined_24,
				action = Action.NAVBAR_SETTINGS,
				labelResId = R.string.bottom_bar_label_settings,
				isEnabled = state == BottomBar2State.SETTINGS,
			)
		}
	}
}

enum class BottomBar2State { KEYS, SCANNER, LOGS, SETTINGS }

/**
 * Unified bottom bar button view for [BottomBar2]
 */
@Composable
fun BottomBarButton2(
	navigator: Navigator,
	@DrawableRes iconId: Int,
	action: Action,
	@StringRes labelResId: Int,
	isEnabled: Boolean,
) {
	val color = if (isEnabled) {
		MaterialTheme.colors.pink500
	} else {
		MaterialTheme.colors.textTertiary
	}
	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		modifier = Modifier
            .clickable { navigator.navigate(action) }
            .width(66.dp)
	) {
		Icon(
			painter = painterResource(id = iconId),
			contentDescription = stringResource(id = labelResId),
			tint = color,
			modifier = Modifier.size(28.dp)
				.padding(bottom = 2.dp)
		)
		Text(
			text = stringResource(id = labelResId),
			color = color,
			style = TypefaceNew.CaptionS,
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
private fun PreviewBottomBar2() {
	val mockModel = KeySetsSelectViewModel(
		listOf(
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
	)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			BottomBar2(EmptyNavigator(), BottomBar2State.KEYS)
		}
	}
}
