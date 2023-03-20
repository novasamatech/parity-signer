package io.parity.signer.screens.scan.transaction.transactionElements

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.NetworkLabelWithIcon
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.encodeHex
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.NetworkSpecs

@Composable
fun TCAddNetwork(specs: NetworkSpecs) {
	Column {
		Text(
			text = stringResource(R.string.transaction_new_network_header),
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
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_network_network_name),
				value = specs.title
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_network_base58),
				value = specs.base58prefix.toString()
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_network_decimals),
				value = specs.decimals.toString()
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_network_unit),
				value = specs.unit
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_network_genesis),
				value = specs.genesisHash.toUByteArray().toByteArray().encodeHex(),
				valueInSameLine = false,
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_network_crypto),
				value = specs.encryption.toString()
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_network_spec_name),
				value = specs.name
			)
			SignerDivider(sidePadding = 0.dp)
			Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
				Text(
					text = stringResource(R.string.transaction_new_network_logo),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textTertiary
				)
				Spacer(modifier = Modifier.weight(1f))
				NetworkLabelWithIcon(
					networkName = specs.logo.capitalize(),
					networkLogo = specs.logo,
				)
			}
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.transaction_new_default_network_path),
				value = specs.pathId
			)
		}
	}
}
