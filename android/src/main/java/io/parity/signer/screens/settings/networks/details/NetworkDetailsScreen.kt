package io.parity.signer.screens.settings.networks.details

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.runtime.Composable
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
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.Callback
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueOppositeElement
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.red500
import io.parity.signer.ui.theme.textSecondary
import io.parity.signer.ui.theme.textTertiary

@Composable
fun NetworkDetailsScreen(
    model: NetworkDetailsModel,
    onBack: Callback,
    onMenu: Callback,
    onAddNetwork: Callback,
		onSignMetadata: (metadataSpecVersion: String) -> Unit,
    onRemoveMetadataCallback: (metadataSpecVersion: String) -> Unit,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {
		ScreenHeader(
			title = null,
			onBack = onBack,
			onMenu = onMenu,
		)
		Column(
			Modifier
				.weight(1f)
				.padding(horizontal = 16.dp)
				.verticalScroll(rememberScrollState()),
			horizontalAlignment = Alignment.CenterHorizontally,
		) {

			NetworkIcon(networkLogoName = model.logo, size = 56.dp)
			Text(
				text = model.title,
				style = SignerTypeface.TitleM,
				color = MaterialTheme.colors.primary,
				modifier = Modifier
					.padding(top = 12.dp, bottom = 16.dp)
					.padding(horizontal = 24.dp)
			)

			Text(
				text = stringResource(R.string.network_details_header_chain_specs),
				style = SignerTypeface.BodyL,
				color = MaterialTheme.colors.textSecondary,
				modifier = Modifier
					.padding(horizontal = 16.dp)
					.fillMaxWidth(1f)
			)
			Spacer(modifier = Modifier.padding(top = 4.dp))
			Column(
				verticalArrangement = Arrangement.spacedBy(8.dp),
				modifier = Modifier
					.padding(bottom = 8.dp)
					.background(
						MaterialTheme.colors.fill6,
						RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
					)
					.padding(16.dp)
			) {
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_network_name),
					value = model.title
				)
				SignerDivider(sidePadding = 0.dp) // already have paddings
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_prefix),
					value = model.base58prefix.toString()
				)
				SignerDivider(sidePadding = 0.dp)
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_decimals),
					value = model.decimals.toString()
				)
				SignerDivider(sidePadding = 0.dp)
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_unit),
					value = model.unit.toString()
				)
				SignerDivider(sidePadding = 0.dp)
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_network_hash),
					value = model.genesisHash,
					valueInSameLine = false
				)
				SignerDivider(sidePadding = 0.dp)
				VerifierContent(model.currentVerifier)
			}

			//metadata
			if (model.meta.isNotEmpty()) {
				Text(
					text = stringResource(R.string.network_details_header_metadata),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textSecondary,
					modifier = Modifier
						.padding(horizontal = 16.dp)
						.padding(top = 8.dp)
						.fillMaxWidth(1f)
				)
				model.meta.forEach { metadata ->
					Spacer(modifier = Modifier.padding(top = 4.dp))
					Column(
						verticalArrangement = Arrangement.spacedBy(8.dp),
						modifier = Modifier
							.background(
								MaterialTheme.colors.fill6,
								RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
							)
							.padding(16.dp),
					) {
						TCNameValueOppositeElement(
							name = stringResource(R.string.network_details_field_version),
							value = metadata.specsVersion
						)
						SignerDivider(sidePadding = 0.dp)
						TCNameValueOppositeElement(
							name = stringResource(R.string.network_details_field_metadata_hash),
							value = metadata.metaHash,
							valueInSameLine = false
						)
						SignerDivider(sidePadding = 0.dp)
						//sign metadata
						Row(Modifier.clickable(onClick = {onSignMetadata(metadata.specsVersion)})) {
							Text(
								text = stringResource(R.string.network_details_metadata_sign_field_label),
								style = SignerTypeface.BodyL,
								color = MaterialTheme.colors.pink300,
								modifier = Modifier
									.weight(1f)
							)
							Image(
								imageVector = Icons.Filled.ChevronRight,
								contentDescription = null,
								colorFilter = ColorFilter.tint(MaterialTheme.colors.textTertiary),
							)
						}
						SignerDivider(sidePadding = 0.dp)
						//delete metadata
						Text(
							text = stringResource(R.string.network_details_metadata_delete_label),
							style = SignerTypeface.BodyL,
							color = MaterialTheme.colors.red500,
							modifier = Modifier
								.clickable {
									onRemoveMetadataCallback(metadata.specsVersion)
								}
								.fillMaxWidth(1f)
						)
					}
				}
			}
			//add network
			Row(
				Modifier
					.clickable(onClick = onAddNetwork)
					.padding(vertical = 16.dp),
				verticalAlignment = Alignment.CenterVertically
			) {
				Box(
					modifier =
					Modifier
						.padding(start = 8.dp, end = 12.dp)
						.background(MaterialTheme.colors.fill6, CircleShape)
						.padding(4.dp)
						.size(24.dp)
				) {
					Image(
						imageVector = Icons.Default.Add,
						contentDescription = stringResource(R.string.network_details_add_network_metadata_label),
						colorFilter = ColorFilter.tint(MaterialTheme.colors.textSecondary),
					)
				}
				Text(
					text = stringResource(R.string.network_details_add_network_metadata_label),
					style = SignerTypeface.TitleS,
					color = MaterialTheme.colors.pink300,
					modifier = Modifier.fillMaxWidth(1f)
				)
			}
		}
	}
}

@Composable
private fun VerifierContent(verifier: VerifierModel) {
	when (verifier.ttype) {
		"none" -> {
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_field_verifier),
				value = verifier.ttype.capitalize()
			)
		}
		"general" -> {
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_field_verifier),
				value = verifier.ttype.capitalize()
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_verifier_public_key_for_general),
				value = verifier.publicKey
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_verifier_crypto_field),
				value = verifier.encryption
			)
		}
		"custom" -> {
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_field_verifier),
				value = verifier.ttype.capitalize()
			)
			SignerDivider(sidePadding = 0.dp)
			Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
				Text(
					text = stringResource(R.string.network_details_verifier_identicon_field),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textTertiary
				)
				Spacer(modifier = Modifier.weight(1f))
				IdentIconImage(identicon = verifier.identicon)
			}
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_verifier_public_key_for_custom),
				value = verifier.publicKey
			)
			SignerDivider(sidePadding = 0.dp)
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_verifier_crypto_field),
				value = verifier.encryption
			)
		}
		else -> {
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_field_verifier),
				value = stringResource(R.string.network_details_verifier_unknown_type)
			)
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
private fun PreviewNetworkDetailsScreenSmall() {
	val model = NetworkDetailsModel.createStub().copy(meta = emptyList())
	SignerNewTheme {
		NetworkDetailsScreen(
			model,
			onBack = {},
			onMenu = {},
			onRemoveMetadataCallback = { _ -> },
			onSignMetadata = {},
			onAddNetwork = { },
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
private fun PreviewNetworkDetailsScreen() {
	val model = NetworkDetailsModel.createStub()
	SignerNewTheme {
		NetworkDetailsScreen(
			model,
			onBack = {},
			onMenu = {},
			onRemoveMetadataCallback = { _ -> },
			onSignMetadata = {},
			onAddNetwork = { },
		)
	}
}

