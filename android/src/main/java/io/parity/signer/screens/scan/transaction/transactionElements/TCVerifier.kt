package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.uniffi.MVerifierDetails

@Composable
fun TCVerifier(verifier: MVerifierDetails) {
	Column {
		Text(
			text = stringResource(R.string.transaction_verifier_header),
			style = SignerTypeface.BodyL,
			color = MaterialTheme.colors.textSecondary,
			modifier = Modifier
				.padding(horizontal = 16.dp) //ios Spacing.medium
				.padding(bottom = 4.dp) //ios Spacing.extraExtraSmall
		)
		Column(
			verticalArrangement = Arrangement.spacedBy(8.dp),
			modifier = Modifier
				.background(
					MaterialTheme.colors.fill6,
					RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
				)
				.padding(16.dp)
		) {
			TCNameValueElement(
				name = stringResource(R.string.transaction_verifier_key),
				value = verifier.publicKey,
				valueInSameLine = false,
			)
			SignerDivider()
			TCNameValueElement(
				name = stringResource(R.string.transaction_verifier_crypto),
				value = verifier.encryption
			)
		}
	}
}
