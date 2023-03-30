package io.parity.signer.screens.settings.verifiercert

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.components.panels.BottomBar2
import io.parity.signer.components.panels.BottomBar2State
import io.parity.signer.components.panels.CameraParentScreen
import io.parity.signer.components.panels.CameraParentSingleton
import io.parity.signer.domain.*
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueOppositeElement
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.MManageNetworks


@Composable
fun VerifierScreen(
	verifierDetails: VerifierDetailsModels,
	onBack: Callback,
	onRemove: Callback,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(
			title = stringResource(R.string.verifier_certificate_title),
			onBack = onBack,
		)
		Column(
			verticalArrangement = Arrangement.spacedBy(8.dp),
			modifier = Modifier
				.padding(horizontal = 8.dp)
				.padding(top = 16.dp, bottom = 8.dp)
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
			//todo dmitry add here red element from settings and menu like in network details
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
		VerifierScreen(
			VerifierDetailsModels.createStub(),
			{},{}
		)
	}
}
