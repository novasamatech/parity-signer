package io.parity.signer.screens.settings.verifiercert

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.*
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueOppositeElement
import io.parity.signer.ui.theme.*


@Composable
internal fun VerifierScreen(
    verifierDetails: VerifierDetailsModel,
    onBack: Callback,
    onRemove: Callback,
) {
	Column(
		Modifier
			.background(MaterialTheme.colors.backgroundPrimary)
			.fillMaxSize(1f)
	) {
		ScreenHeader(
			title = stringResource(R.string.verifier_certificate_title),
			onBack = onBack,
		)
		Column(
			verticalArrangement = Arrangement.spacedBy(8.dp),
			modifier = Modifier
				.padding(top = 16.dp, bottom = 8.dp, start = 8.dp, end = 8.dp)
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
				)
				.padding(16.dp),
		) {
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_verifier_public_key_for_general),
				value = verifierDetails.publicKey,
				valueInSameLine = false,
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_verifier_crypto_field),
				value = verifierDetails.encryption
			)
		}
		Text(
			text = stringResource(R.string.verifier_certificate_remove_cta),
			style = SignerTypeface.TitleS,
			color = MaterialTheme.colors.red400,
			modifier = Modifier
				.clickable(onClick = onRemove)
				.fillMaxWidth(1f)
				.padding(vertical = 14.dp, horizontal = 24.dp)
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
private fun PreviewNetworksList() {
	SignerNewTheme {
		VerifierScreen(VerifierDetailsModel.createStub(), {}, {})
	}
}
