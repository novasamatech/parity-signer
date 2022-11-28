package io.parity.signer.screens.scan.transaction

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
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
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PrimaryButtonBottomSheet
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonBottomSheet
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.models.Callback
import io.parity.signer.ui.theme.*


/**
 * Approval request for metadata update
 */
@Composable
fun TransactionSignScreen(
	model: MetadataUpdateModel,
	onCancel: Callback,
	onApprove: Callback,
) {
	Column() {
		ScreenHeader(
			stringId = R.string.screen_title_update_metadata,
			onBack = onCancel
		)

		Column(
			modifier = Modifier
				.weight(1f)
				.verticalScroll(rememberScrollState())
		) {
			val qrRounding = dimensionResource(id = R.dimen.qrShapeCornerRadius)
			val plateShape =
				RoundedCornerShape(qrRounding, qrRounding, qrRounding, qrRounding)

			//verifier
			Text(
				text = stringResource(R.string.metadata_subtitle_keys),
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.LabelM,
				modifier = Modifier.padding(top = 24.dp, start = 24.dp, end = 24.dp),
			)
			Column(
				Modifier
					.padding(horizontal = 8.dp)
					.background(MaterialTheme.colors.fill6, plateShape)
					.padding(horizontal = 16.dp, vertical = 12.dp)
			) {
				Text(
					stringResource(R.string.update_metadata_keyword_key),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.BodyL,
				)
				Spacer(modifier = Modifier.padding(top = 8.dp))
				Text(
					model.verifierKey,
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.BodyL,
				)
				Spacer(modifier = Modifier.padding(top = 12.dp))
				SignerDivider()
				Spacer(modifier = Modifier.padding(top = 12.dp))
				Row(Modifier.fillMaxWidth()) {
					Text(
						text = stringResource(R.string.update_metadata_keyword_crypto),
						color = MaterialTheme.colors.textTertiary,
						style = SignerTypeface.BodyL,
					)
					Spacer(modifier = Modifier.weight(1f))
					Text(
						text = model.verifierAlg,
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.BodyL,
					)
				}
			}

//metadata
			Text(
				text = stringResource(R.string.metadata_subtitle_add_metadata),
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.LabelM,
				modifier = Modifier.padding(top = 12.dp, start = 24.dp, end = 24.dp),
			)
			Column(
				Modifier
					.padding(horizontal = 8.dp)
					.background(MaterialTheme.colors.fill6, plateShape)
					.padding(horizontal = 16.dp, vertical = 12.dp)
			) {
				Row(Modifier.fillMaxWidth()) {
					Text(
						text = stringResource(R.string.update_metadata_keyword_metadata),
						color = MaterialTheme.colors.textTertiary,
						style = SignerTypeface.BodyL,
					)
					Spacer(modifier = Modifier.weight(1f))
					Text(
						text = model.metadataName,
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.BodyL,
					)
				}
				Spacer(modifier = Modifier.padding(top = 12.dp))
				SignerDivider()
				Spacer(modifier = Modifier.padding(top = 12.dp))
				Text(
					text = model.metadataHash,
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.BodyL,
				)
			}
		}
		PrimaryButtonBottomSheet(
			label = stringResource(R.string.approve_confirm_button),
			modifier = Modifier.padding(horizontal = 24.dp),
			onClicked = onApprove
		)
		SecondaryButtonBottomSheet(
			label = stringResource(id = R.string.generic_cancel),
			modifier = Modifier.padding(horizontal = 24.dp),
			onClicked = onCancel
		)
		Spacer(modifier = Modifier.padding(top = 40.dp))
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
private fun PreviewTransactionSignScreen() {
	val model = MetadataUpdateModel.createStub()
	SignerNewTheme {
		TransactionSignScreen(
			model, {}, {},
		)
	}
}
