package io.parity.signer.components.panels

import android.content.res.Configuration
import androidx.annotation.StringRes
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.BottomAppBar
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.CalendarViewDay
import androidx.compose.material.icons.filled.CropFree
import androidx.compose.material.icons.filled.Pattern
import androidx.compose.material.icons.filled.Settings
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.screens.KeySetViewModel
import io.parity.signer.screens.KeySetsScreen
import io.parity.signer.screens.KeySetsSelectViewModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.FooterButton
import io.parity.signer.uniffi.actionGetName

/**
 * Bar to be shown on the bottom of screen;
 * Redesigned version
 */
@Composable
fun BottomBar2(
	navigator: Navigator,
	footerButton: FooterButton?,
) {
	BottomAppBar(
		backgroundColor = MaterialTheme.colors.backgroundSecondary,
		elevation = 0.dp,
		modifier = Modifier.height(56.dp)
	) {
		Row(
			horizontalArrangement = Arrangement.SpaceEvenly,
			modifier = Modifier.fillMaxWidth(1f)
		) {
			BottomBarButton2(
				footerButton = footerButton,
				navigator = navigator,
				image = Icons.Default.Pattern,
				action = Action.NAVBAR_KEYS,
				labelResId = R.string.bottom_bar_label_key_sets,
			)
			BottomBarButton2(
				footerButton = footerButton,
				navigator = navigator,
				image = Icons.Default.CropFree,
				action = Action.NAVBAR_SCAN,
				labelResId = R.string.bottom_bar_label_scanner,
			)
			BottomBarButton2(
				footerButton = footerButton,
				navigator = navigator,
				image = Icons.Default.CalendarViewDay,
				action = Action.NAVBAR_LOG,
				labelResId = R.string.bottom_bar_label_logs,
			)
			BottomBarButton2(
				footerButton = footerButton,
				navigator = navigator,
				image = Icons.Default.Settings,
				action = Action.NAVBAR_SETTINGS,
				labelResId = R.string.bottom_bar_label_settings,
			)
		}
	}
}

/**
 * Unified bottom bar button view for [BottomBar2]
 */
@Composable
fun BottomBarButton2(
	footerButton: FooterButton?,
	navigator: Navigator,
	image: ImageVector,
	action: Action,
	@StringRes labelResId: Int,
) {
	val selected = footerButton == actionGetName(action)
	val color = if (selected) {
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
			imageVector = image,
			contentDescription = actionGetName(action).toString(),
			tint = color,
			modifier = Modifier.size(28.dp)
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
			BottomBar2(EmptyNavigator(), null)
		}
	}
}
