package io.parity.signer.screens.onboarding.eachstartchecks.airgap

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AirplanemodeActive
import androidx.compose.material.icons.filled.Cable
import androidx.compose.material.icons.filled.Wifi
import androidx.compose.material.icons.outlined.CheckCircle
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.graphics.compositeOver
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.theme.*


@Composable
fun AirgapScreen() {
	Column(horizontalAlignment = Alignment.CenterHorizontally) {
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
				AirgapItem(AirgapItemType.WIFI, true)
				SignerDivider(modifier = Modifier.padding(start = 40.dp))
				AirgapItem(AirgapItemType.AIRPLANE_MODE, false)
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
				Row(verticalAlignment = Alignment.CenterVertically,
					modifier = Modifier.padding(16.dp),) {
					Image(
						imageVector = Icons.Filled.Cable,
						contentDescription = null,
						colorFilter = ColorFilter.tint(MaterialTheme.colors.textSecondary),
						modifier = Modifier
							.size(20.dp)
					)
					Text(
						text = stringResource(R.string.airgap_onboarding_disconnect_cable_header),
						color = MaterialTheme.colors.textSecondary,
						style = SignerTypeface.TitleS,
						modifier = Modifier
							.padding(horizontal = 16.dp, vertical = 14.dp)
							.weight(1f)
					)
				}
				SignerDivider()

				AirgapItem(AirgapItemType.AIRPLANE_MODE, false)
			}
		}
	}
}

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
				){}
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


private enum class AirgapItemType { WIFI, AIRPLANE_MODE }


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
			AirgapScreen()
		}
	}
}


