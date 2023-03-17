package io.parity.signer.screens.networks.details

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
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
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.Callback
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.FakeNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueOppositeElement
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action

@Composable
fun NetworkDetailsScreen(
	model: NetworkDetailsModel,
	rootNavigator: Navigator,
	onMenu: Callback,
	onAddNetwork: Callback,
	onRemoveMetadataCallback: (version: String) -> Unit,
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {

		ScreenHeader(
			title = null,
			onBack = { rootNavigator.backAction() },
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
				SignerDivider()
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_prefix),
					value = model.base58prefix.toString()
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_decimals),
					value = model.decimals.toString()
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_unit),
					value = model.unit.toString()
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = stringResource(R.string.network_details_field_network_hash),
					value = model.genesisHash,
					valueInSameLine = false
				)
				SignerDivider()
				VerifierContent(model.currentVerifier)
			}
			Spacer(modifier = Modifier.padding(top = 16.dp))

			//metadata
			if (model.meta.isNotEmpty()) {
				Text(
					text = stringResource(R.string.network_details_header_metadata),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textSecondary,
					modifier = Modifier
						.padding(horizontal = 16.dp)
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
						SignerDivider()
						TCNameValueOppositeElement(
							name = stringResource(R.string.network_details_field_metadata_hash),
							value = metadata.metaHash,
							valueInSameLine = false
						)
						SignerDivider()
						//sign metadata
						Row(Modifier.clickable {
							FakeNavigator().navigate(
								Action.MANAGE_METADATA,
								metadata.specsVersion
							)
							rootNavigator.navigate(Action.SIGN_METADATA)
						}) {
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
						SignerDivider()
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
			Row(
				Modifier
					.padding(horizontal = 16.dp)
					.clickable(onClick = onAddNetwork)) {

				Text(
					text = stringResource(R.string.network_details_metadata_delete_label),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.red500,
					modifier = Modifier
						.fillMaxWidth(1f)
				)
			}
			//todo dmitry add network metadata here
		}
	}
}

@Composable
private fun VerifierContent(verifier: VerifierModel) {
	when (verifier.ttype) {
		"general", "none" -> {
			TCNameValueOppositeElement(
				name = stringResource(R.string.network_details_field_verifier),
				value = verifier.ttype
			)
		}
		"custom" -> {
			Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
				Text(
					text = stringResource(R.string.network_details_field_verifier),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.textTertiary
				)
				Row {
					IdentIcon(identicon = verifier.identicon)
					Spacer(modifier = Modifier.padding(start = 8.dp))
					Text(
						verifier.ttype,
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.primary
					)
				}
				Text(
					verifier.publicKey,
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary
				)
				Text(
					stringResource(
						R.string.network_details_verifier_encryption_field,
						verifier.encryption
					),
					style = SignerTypeface.BodyL,
					color = MaterialTheme.colors.primary
				)
			}
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
private fun PreviewNetworkDetailsScreen() {
	val model = NetworkDetailsModel.createStub()
	SignerNewTheme {
		NetworkDetailsScreen(
			model,
			rootNavigator = EmptyNavigator(),
			onMenu = {},
			onRemoveMetadataCallback = { _ -> }
		)
	}
}

