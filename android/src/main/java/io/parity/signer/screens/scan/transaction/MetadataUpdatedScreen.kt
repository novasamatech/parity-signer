package io.parity.signer.screens.scan.transaction

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonBottomSheet
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill6


/**
 * Approval request for metadata update
 */
@Composable
fun MetadataUpdateScreen(model: MetadataUpdateModel,
												 onBack: Callback,
) {
	Column() {
		ScreenHeader(
			stringId = R.string.screen_title_update_metadata,
			onBack = onBack
		)

		Column(
			modifier = Modifier
				.weight(1f)
				.verticalScroll(rememberScrollState())
		) {
			val qrRounding = dimensionResource(id = R.dimen.qrShapeCornerRadius)
			val plateShape =
				RoundedCornerShape(qrRounding, qrRounding, qrRounding, qrRounding)

			Text(stringResource(R.string.metadata_subtitle_keys))
			Column(
				Modifier
					.background(MaterialTheme.colors.fill6, plateShape)
			) {
				Text("Key")
				Text("sdfsdfsdf")
				SignerDivider()
				Row(Modifier.fillMaxWidth()) {
					Text(text = "Crypto")
					Spacer(modifier = Modifier.weight(1f))
					Text(text = "scr....")
				}
			}
			Text(stringResource(R.string.metadata_subtitle_add_metadata))
			Column(
				Modifier
					.background(MaterialTheme.colors.fill6, plateShape)
			) {

			}
		}
		PrimaryButtonBottomSheet(label = stringResource(R.string.approve_confirm_button)) {
			//todo dmitry approve action
		}
		SecondaryButtonBottomSheet(label = stringResource(id = R.string.generic_cancel)) {
			//todo dmitry approve action
		}
	}
}


data class MetadataUpdateModel(
	val metadataName: String,
	val metadataHash: String,
)


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
private fun PreviewMetadataUpdateScreen() {
	val model = MetadataUpdateModel("scrpr", "sdfkjskfdjksjdfks")
	SignerNewTheme {
		MetadataUpdateScreen(
			model, {},
		)
	}
}
