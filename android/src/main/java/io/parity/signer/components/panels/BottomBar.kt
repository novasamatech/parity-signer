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
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import io.parity.signer.R
import io.parity.signer.domain.Callback
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.ui.mainnavigation.CoreUnlockedNavSubgraph
import io.parity.signer.ui.theme.*

/**
 * Bar to be shown on the bottom of screen;
 */
@Composable
fun BottomBar(
	navController: NavController,
	selectedOption: BottomBarOptions,
) {
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
				onClick = { navigateBottomBar(navController, BottomBarOptions.KEYS) },
				iconId = R.drawable.ic_key_outlined_24,
				labelResId = R.string.bottom_bar_label_key_sets,
				isEnabled = selectedOption == BottomBarOptions.KEYS,
			)
			BottomBarMiddleButton(
				onClick = {
					navigateBottomBar(navController, BottomBarOptions.SCANNER)
				},
				iconId = R.drawable.ic_qe_code_24,
				labelResId = R.string.bottom_bar_label_scanner,
				isEnabled = selectedOption == BottomBarOptions.SCANNER,
			)
			BottomBarButton(
				onClick = {
					navigateBottomBar(navController, BottomBarOptions.SETTINGS)
				},
				iconId = R.drawable.ic_settings_outlined_24,
				enabledIconId = R.drawable.ic_settings_filled_24,
				labelResId = R.string.bottom_bar_label_settings,
				isEnabled = selectedOption == BottomBarOptions.SETTINGS,
			)
		}
	}
}

enum class BottomBarOptions { KEYS, SCANNER, SETTINGS }

private fun navigateBottomBar(
	navController: NavController,
	buttonType: BottomBarOptions,
) {
	when (buttonType) {
		BottomBarOptions.KEYS -> navController.navigate(CoreUnlockedNavSubgraph.KeySet.destination(null))
		BottomBarOptions.SCANNER -> navController.navigate(CoreUnlockedNavSubgraph.camera)
		BottomBarOptions.SETTINGS -> navController.navigate(CoreUnlockedNavSubgraph.settings)
	}
}

/**
 * Unified bottom bar button view for [BottomBar]
 */
@Composable
fun BottomBarButton(
	onClick: Callback,
	@DrawableRes iconId: Int,
	@DrawableRes enabledIconId: Int? = null,
	@StringRes labelResId: Int,
	isEnabled: Boolean,
) {
	val color = if (isEnabled) {
		MaterialTheme.colors.primary
	} else {
		MaterialTheme.colors.textTertiary
	}
	Column(
		horizontalAlignment = Alignment.CenterHorizontally,
		modifier = Modifier
			.clickable(onClick = onClick)
			.width(66.dp)
	) {
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
	onClick: Callback,
	@DrawableRes iconId: Int,
	@DrawableRes enabledIconId: Int? = null,
	@StringRes labelResId: Int,
	isEnabled: Boolean,
) {
	Box(
		contentAlignment = Alignment.Center,
		modifier = Modifier
            .padding(vertical = 4.dp)
            .border(
                2.dp,
                MaterialTheme.colors.fill12,
                RoundedCornerShape(32.dp)
            )
            .clickable(onClick = onClick)
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
			BottomBar(rememberNavController(), BottomBarOptions.KEYS)
		}
	}
}
