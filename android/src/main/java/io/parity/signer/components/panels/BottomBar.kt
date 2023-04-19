package io.parity.signer.components.panels

import android.content.res.Configuration
import androidx.annotation.DrawableRes
import androidx.annotation.StringRes
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.BottomAppBar
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action

/**
 * Bar to be shown on the bottom of screen;
 * Redesigned version
 * @param onBeforeActionWhenClicked is when some actiond need to be done before
 * we can navigate to tapped state. Workaround for state machine
 */
@Composable
fun BottomBar(
	navigator: Navigator,
	state: BottomBarState,
	skipRememberCameraParent: Boolean = false,
	onBeforeActionWhenClicked: Callback? = null,
) {
	if (!skipRememberCameraParent) {
		LaunchedEffect(key1 = Unit) {
			CameraParentSingleton.lastPossibleParent =
				CameraParentScreen.BottomBarScreen(state)
		}
	}
	BottomAppBar(
		backgroundColor = MaterialTheme.colors.backgroundSecondary,
		elevation = 8.dp,
		modifier = Modifier.height(50.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.SpaceEvenly,
			modifier = Modifier.fillMaxWidth(1f),
			verticalAlignment = Alignment.CenterVertically
		) {
			BottomBarButton(
				navigator = navigator,
				iconId = R.drawable.ic_key_outlined_24,
				action = Action.NAVBAR_KEYS,
				labelResId = R.string.bottom_bar_label_key_sets,
				isEnabled = state == BottomBarState.KEYS,
				onBeforeActionWhenClicked = onBeforeActionWhenClicked,
			)
			//todo dmitry new design https://www.figma.com/file/k0F8XYk9XVYdKLtkj0Vzp5/Signer-(Vault)-%C2%B7-Redesign?node-id=11930-77855&t=khcNTdojUGw29HQu-4
			BottomBarMiddleButton(
				navigator = navigator,
				iconId = R.drawable.ic_qe_code_24,
				action = Action.NAVBAR_SCAN,
				labelResId = R.string.bottom_bar_label_scanner,
				isEnabled = state == BottomBarState.SCANNER,
				onBeforeActionWhenClicked = onBeforeActionWhenClicked,
			)
			BottomBarButton(
				navigator = navigator,
				iconId = R.drawable.ic_settings_outlined_24,
				enabledIconId = R.drawable.ic_settings_filled_24,
				action = Action.NAVBAR_SETTINGS,
				labelResId = R.string.bottom_bar_label_settings,
				isEnabled = state == BottomBarState.SETTINGS,
				onBeforeActionWhenClicked = onBeforeActionWhenClicked,
			)
		}
	}
}

enum class BottomBarState { KEYS, SCANNER, SETTINGS }

/**
 * Unified bottom bar button view for [BottomBar]
 */
@Composable
fun BottomBarButton(
	navigator: Navigator,
	@DrawableRes iconId: Int,
	@DrawableRes enabledIconId: Int? = null,
	action: Action,
	@StringRes labelResId: Int,
	isEnabled: Boolean,
	onBeforeActionWhenClicked: Callback?
) {
	val color = if (isEnabled) {
		MaterialTheme.colors.primary
	} else {
		MaterialTheme.colors.textTertiary
	}
	Column(horizontalAlignment = Alignment.CenterHorizontally,
		modifier = Modifier
			.clickable {
				onBeforeActionWhenClicked?.invoke()
				navigator.navigate(action)
			}
			.width(66.dp)) {
		Icon(
			painter = painterResource(
				id = if (isEnabled) enabledIconId ?: iconId else iconId
			),
			contentDescription = stringResource(id = labelResId),
			tint = color,
			modifier = Modifier
				.size(28.dp)
				.padding(bottom = 2.dp)
		)
		Text(
			text = stringResource(id = labelResId),
			color = color,
			style = SignerTypeface.CaptionS,
		)
	}
}


@Composable
fun BottomBarMiddleButton(
	navigator: Navigator,
	@DrawableRes iconId: Int,
	@DrawableRes enabledIconId: Int? = null,
	@StringRes labelResId: Int,
	action: Action,
	isEnabled: Boolean,
	onBeforeActionWhenClicked: Callback?
) {
	Box(contentAlignment = Alignment.Center,
		modifier = Modifier
			.padding(vertical = 4.dp)
			.border(2.dp, MaterialTheme.colors.fill12, RoundedCornerShape(32.dp))
			.clickable {
				onBeforeActionWhenClicked?.invoke()
				navigator.navigate(action)
			}
			.width(80.dp)
			.fillMaxHeight(1f)
	) {
		Icon(
			painter = painterResource(
				id = if (isEnabled) enabledIconId ?: iconId else iconId
			),
			contentDescription = stringResource(id = labelResId),
			tint = MaterialTheme.colors.primary,
			modifier = Modifier
				.size(28.dp)
				.padding(bottom = 2.dp)
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
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 550.dp)) {
			BottomBar(EmptyNavigator(), BottomBarState.KEYS)
		}
	}
}
