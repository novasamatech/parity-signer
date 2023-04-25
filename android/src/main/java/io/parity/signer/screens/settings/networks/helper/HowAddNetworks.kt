package io.parity.signer.screens.settings.networks.helper

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*


//todo text export
@Composable
fun HowAddNetworks(
	onClose: Callback,
	onNext: Callback,
	onScanClicked: Callback,
) {
	Column() {
		ScreenHeaderWithButton(
			canProceed = true,
			btnText = "Next",
			onClose = onClose,
			onDone = onNext,
		)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.padding(horizontal = 16.dp)
		) {
			Text(
				text = "Step 1/2",
				style = SignerTypeface.CaptionM,
				color = MaterialTheme.colors.textSecondary,
				modifier = Modifier.padding(top = 16.dp)
			)
			Text(
				text = "How to Add Networks",
				style = SignerTypeface.TitleL,
				color = MaterialTheme.colors.primary,
				modifier = Modifier.padding(bottom = 16.dp)
			)
			Row(
				modifier = Modifier
					.fillMaxWidth(1f)
					.background(
						MaterialTheme.colors.fill12,
						RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
					)
					.padding(16.dp)
			) {

				Box(
					modifier = Modifier
						.background(
							MaterialTheme.colors.backgroundSystem,
							CircleShape,
						)
						.defaultMinSize(32.dp, 32.dp),
					contentAlignment = Alignment.Center,
				) {
					Text(
						text = "1",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary,
						modifier = Modifier.padding(end = 12.dp)
					)
				}
				Column() {

				}
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
private fun PreviewHowAddNetworks() {
	SignerNewTheme {
		HowAddNetworks({}, {}, {})
	}
}
