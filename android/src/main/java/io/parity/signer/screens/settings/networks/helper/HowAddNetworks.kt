package io.parity.signer.screens.settings.networks.helper

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.components.base.ScreenHeaderWithButton
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textSecondary


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
		Row() {

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
