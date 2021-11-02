package io.parity.signer.components

import android.util.Log
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.abbreviateString
import io.parity.signer.ui.theme.Typography

@Composable
fun SeedCard(
	seedName: String,
	seedSelector: Boolean = false,
	signerDataModel: SignerDataModel
) {
	Log.d("seed", "seed " + seedName)
	Row(
		modifier = Modifier
			.padding(8.dp)
	) {
		Image(
			signerDataModel.getIdenticon(
				signerDataModel.getRootIdentity(seedName)
					.optString("ss58", "failonthis"), 64
			), "identicon", modifier = Modifier.scale(0.75f)
		)
		Spacer(modifier = Modifier.width(10.dp))
		Column {
			Text(seedName, style = Typography.body1)
			if (seedSelector) {
				Text(
					signerDataModel.getRootIdentity(seedName).optString("ss58")
						.abbreviateString(8), style = Typography.body2
				)
			}
		}
	}
}
