package io.parity.signer.screens.onboarding.eachstartchecks.airgap

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AirplanemodeActive
import androidx.compose.material.icons.filled.Bluetooth
import androidx.compose.material.icons.filled.Cable
import androidx.compose.material.icons.filled.Wifi
import androidx.compose.material.icons.outlined.CheckCircle
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.graphics.compositeOver
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import io.parity.signer.R
import io.parity.signer.components.base.CheckboxWithText
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*

@Composable
fun AirgapScreen(
	onCta: Callback,
) {

	val viewModel = viewModel<AirGapViewModel>()
	val state = viewModel.state.collectAsState()
	DisposableEffect(Unit) {
		viewModel.init()
		onDispose {
			viewModel.unInit()
		}
	}

	AirgapScreen(
		state = state.value,
		onCablesConfirmCheckbox = viewModel::onCableCheckboxClicked,
		onCta = onCta
	)
}

@Composable
private fun AirgapScreen(
	state: AirGapScreenState,
	onCablesConfirmCheckbox: Callback,
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
					.padding(horizontal = 24.dp, vertical = 12.dp),
				text = stringResource(R.string.airgap_onboarding_title),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleL,
				textAlign = TextAlign.Center,
			)
			Text(
				modifier = Modifier
					.fillMaxWidth(1f)
					.padding(horizontal = 24.dp),
				text = stringResource(R.string.airgap_onboarding_subtitle),
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyL,
				textAlign = TextAlign.Center,
			)

			Surface(
				shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
				border = BorderStroke(1.dp, color = MaterialTheme.colors.fill12),
				color = MaterialTheme.colors.fill6,
				modifier = Modifier.padding(16.dp)
			) {
				Column(
					horizontalAlignment = Alignment.CenterHorizontally,
				) {
					AirgapItem(AirgapItemType.AIRPLANE_MODE, state.airplaneModeEnabled)
					SignerDivider(modifier = Modifier.padding(start = 40.dp))
					AirgapItem(AirgapItemType.WIFI, state.wifiDisabled)
					SignerDivider(modifier = Modifier.padding(start = 40.dp))
					AirgapItem(AirgapItemType.BLUETOOTH, state.bluetoothDisabled)
				}
			}

			Surface(
				shape = RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius)),
				border = BorderStroke(1.dp, color = MaterialTheme.colors.fill12),
				color = MaterialTheme.colors.fill6,
				modifier = Modifier.padding(16.dp)
			) {
				Column(
					horizontalAlignment = Alignment.CenterHorizontally,
				) {
					Row(
						verticalAlignment = Alignment.CenterVertically,
						modifier = Modifier.padding(16.dp),
					) {
						Image(
							imageVector = Icons.Filled.Cable,
							contentDescription = null,
							colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
							modifier = Modifier
								.size(24.dp)
						)
						Text(
							text = stringResource(R.string.airgap_onboarding_disconnect_cable_header),
							color = MaterialTheme.colors.textTertiary,
							style = SignerTypeface.TitleS,
							modifier = Modifier
								.padding(horizontal = 16.dp, vertical = 14.dp)
								.weight(1f)
						)
					}
					SignerDivider()
					CheckboxWithText(
						checked = state.cablesDisconnected,
						text = stringResource(R.string.airgap_onboarding_disconnect_cable_checkbox_description),
						modifier = Modifier.padding(16.dp),
					) { newIsChecked ->
						onCablesConfirmCheckbox()
					}
				}
			}
		}
		PrimaryButtonWide(
			label = stringResource(R.string.button_next),
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
	val cablesDisconnected: Boolean = false, //false default
)

private fun AirGapScreenState.isReadyToProceed() =
	airplaneModeEnabled && wifiDisabled
		&& bluetoothDisabled && cablesDisconnected

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
		}
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
					modifier = Modifier
						.size(20.dp)
				)
			}
			//checkmark
			if (isPassed) {
				//because icon have paddings on a side we need to draw background separately with different paddings
				Surface(
					color = color,
					shape = CircleShape,
					modifier = Modifier.size(16.dp)
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

		val text = when (type) {
			AirgapItemType.WIFI -> stringResource(R.string.airgap_onboarding_wifi_header)
			AirgapItemType.AIRPLANE_MODE -> stringResource(R.string.airgap_onboarding_airplane_mode_header)
			AirgapItemType.BLUETOOTH -> stringResource(R.string.airgap_onboarding_bluetooth_header)
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


private enum class AirgapItemType { WIFI, AIRPLANE_MODE, BLUETOOTH }


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewAirgapScreen() {
	Box(modifier = Modifier.fillMaxSize(1f)) {
		SignerNewTheme() {
			val state = AirGapScreenState(
				airplaneModeEnabled = true,
				wifiDisabled = false,
				bluetoothDisabled = true
			)
			AirgapScreen(state, {}, {})
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
				bluetoothDisabled = true
			)
			AirgapScreen(state, {}, {})
		}
	}
}
