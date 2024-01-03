package io.parity.signer.screens.initial.eachstartchecks.airgap

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Adb
import androidx.compose.material.icons.filled.AirplanemodeActive
import androidx.compose.material.icons.filled.Bluetooth
import androidx.compose.material.icons.filled.Cable
import androidx.compose.material.icons.filled.Wifi
import androidx.compose.material.icons.outlined.CheckCircle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.graphics.compositeOver
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.components.base.CheckboxIcon
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*

@Composable
fun AirgapScreen(
	isInitialOnboarding: Boolean,
	onProceed: Callback,
) {
	val viewModel = viewModel<AirGapViewModel>()
	val state = viewModel.state.collectAsStateWithLifecycle()
	DisposableEffect(Unit) {
		viewModel.init()
		onDispose {
			viewModel.unInit()
		}
	}

	AirgapScreen(
		state = state.value,
		isInitialOnboarding = isInitialOnboarding,
		onCta = {
			viewModel.onConfirmedAirgap()
			onProceed()
		},
	)
}

@Composable
private fun AirgapScreen(
	state: AirGapScreenState,
	isInitialOnboarding: Boolean,
	onCta: Callback,
) {
	Column() {
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.weight(1f, true)
		) {
			Text(
				modifier = Modifier
					.fillMaxWidth(1f)
					.padding(horizontal = 24.dp)
					.padding(top = 32.dp, bottom = 12.dp),
				text = if (isInitialOnboarding) stringResource(R.string.airgap_onboarding_title_onboarding)
				else stringResource(R.string.airgap_onboarding_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
				textAlign = TextAlign.Center,
			)
			Text(
				modifier = Modifier
					.fillMaxWidth(1f)
					.padding(horizontal = 24.dp)
					.padding(bottom = 16.dp),
				text = if (isInitialOnboarding) stringResource(R.string.airgap_onboarding_subtitle_onboarding)
				else stringResource(R.string.airgap_onboarding_subtitle),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyL,
				textAlign = TextAlign.Center,
			)

			Surface(
				shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
				border = BorderStroke(1.dp, color = MaterialTheme.colors.fill12),
				color = MaterialTheme.colors.fill6,
				modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp)
			) {
				Column(
					horizontalAlignment = Alignment.CenterHorizontally,
				) {
					AirgapItem(AirgapItemType.AIRPLANE_MODE, state.airplaneModeEnabled)
					SignerDivider(modifier = Modifier.padding(start = 40.dp))
					AirgapItem(AirgapItemType.WIFI, state.wifiDisabled)
					SignerDivider(modifier = Modifier.padding(start = 40.dp))
					AirgapItem(AirgapItemType.BLUETOOTH, state.bluetoothDisabled)
					SignerDivider(modifier = Modifier.padding(start = 40.dp))
					AirgapItem(AirgapItemType.ADB_ENABLED, state.isAdbDisabled)
					SignerDivider(modifier = Modifier.padding(start = 40.dp))
					AirgapItem(AirgapItemType.USB, state.isUsbDisconnected)
				}
			}
		}
		PrimaryButtonWide(
			label = if (isInitialOnboarding) stringResource(R.string.button_next)
			else stringResource(R.string.generic_done),
			modifier = Modifier.padding(24.dp),
			isEnabled = state.isReadyToProceed(),
			onClicked = onCta,
		)
	}
}

data class AirGapScreenState(
	val airplaneModeEnabled: Boolean,
	val wifiDisabled: Boolean,
	val bluetoothDisabled: Boolean,
	val isUsbDisconnected: Boolean,
	val isAdbDisabled: Boolean,
)

private fun AirGapScreenState.isReadyToProceed() =
	airplaneModeEnabled && wifiDisabled && bluetoothDisabled && isUsbDisconnected && isAdbDisabled

@Composable
private fun AirgapItem(type: AirgapItemType, isPassed: Boolean) {
	val color =
		if (isPassed) MaterialTheme.colors.accentGreen else MaterialTheme.colors.accentRed
	val backgroundColor =
		MaterialTheme.colors.fill6.compositeOver(MaterialTheme.colors.background)
	Row(
		verticalAlignment = Alignment.CenterVertically,
		modifier = Modifier.padding(16.dp),
	) {
		val icon = when (type) {
			AirgapItemType.WIFI -> Icons.Filled.Wifi
			AirgapItemType.AIRPLANE_MODE -> Icons.Filled.AirplanemodeActive
			AirgapItemType.BLUETOOTH -> Icons.Filled.Bluetooth
			AirgapItemType.USB -> Icons.Filled.Cable
			AirgapItemType.ADB_ENABLED -> Icons.Filled.Adb
		}
		IconWithCheckmark(color, icon, backgroundColor, isPassed)

		val text = when (type) {
			AirgapItemType.WIFI -> stringResource(R.string.airgap_onboarding_wifi_header)
			AirgapItemType.AIRPLANE_MODE -> stringResource(R.string.airgap_onboarding_airplane_mode_header)
			AirgapItemType.BLUETOOTH -> stringResource(R.string.airgap_onboarding_bluetooth_header)
			AirgapItemType.USB -> stringResource(R.string.airgap_onboarding_disconnect_cable_checkbox_description)
			AirgapItemType.ADB_ENABLED -> stringResource(R.string.airgap_onboarding_adb_disable)
		}
		Text(
			text = text,
			color = color,
			style = SignerTypeface.TitleS,
			modifier = Modifier
				.padding(horizontal = 16.dp, vertical = 14.dp)
				.weight(1f)
		)
	}
}

@Composable
private fun IconWithCheckmark(
	color: Color, icon: ImageVector, backgroundColor: Color, isPassed: Boolean
) {
	Box(contentAlignment = Alignment.BottomEnd) {
//			icon
		Box(
			contentAlignment = Alignment.Center,
			modifier = Modifier
				.size(40.dp)
				.background(color, CircleShape)
		) {
			Image(
				imageVector = icon,
				contentDescription = null,
				colorFilter = ColorFilter.tint(backgroundColor),
				modifier = Modifier.size(20.dp)
			)
		}
		//checkmark
		if (isPassed) {
			//because icon have paddings on a side we need to draw background separately with different paddings
			Surface(
				color = color, shape = CircleShape, modifier = Modifier.size(16.dp)
			) {}
			Image(
				imageVector = Icons.Outlined.CheckCircle,
				contentDescription = null,
				colorFilter = ColorFilter.tint(backgroundColor),
				modifier = Modifier
					.size(18.dp)
					.offset(x = 2.dp, y = 2.dp)
			)
		}
	}
}

private enum class AirgapItemType { WIFI, AIRPLANE_MODE, BLUETOOTH, USB, ADB_ENABLED, }


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewAirgapScreenObnoarding() {
	Box(modifier = Modifier.fillMaxSize(1f)) {
		SignerNewTheme() {
			val state = AirGapScreenState(
				airplaneModeEnabled = true,
				wifiDisabled = false,
				bluetoothDisabled = true,
				isAdbDisabled = false,
				isUsbDisconnected = true,
			)
			AirgapScreen(state = state,
				isInitialOnboarding = true,
				onCablesConfirmCheckbox = {},
				onCta = {})
		}
	}
}

@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewAirgapScreenBlocker() {
	Box(modifier = Modifier.fillMaxSize(1f)) {
		SignerNewTheme() {
			val state = AirGapScreenState(
				airplaneModeEnabled = true,
				wifiDisabled = false,
				bluetoothDisabled = true,
				isAdbDisabled = false,
				isUsbDisconnected = true,
			)
			AirgapScreen(state = state,
				isInitialOnboarding = false,
				onCablesConfirmCheckbox = {},
				onCta = {})
		}
	}
}


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewAirgapScreenSmall() {
	Box(modifier = Modifier.size(width = 250.dp, height = 450.dp)) {
		SignerNewTheme() {
			val state = AirGapScreenState(
				airplaneModeEnabled = true,
				wifiDisabled = false,
				bluetoothDisabled = true,
				isAdbDisabled = false,
				isUsbDisconnected = true,
			)
			AirgapScreen(state = state,
				isInitialOnboarding = true,
				onCablesConfirmCheckbox = {},
				onCta = {})
		}
	}
}
