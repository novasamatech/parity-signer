package io.parity.signer.screens.networks.details

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.networkicon.NetworkIcon
import io.parity.signer.domain.EmptyNavigator
import io.parity.signer.domain.Navigator
import io.parity.signer.screens.scan.transaction.transactionElements.TCNameValueOppositeElement
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6

@Composable
fun NetworkDetailsScreen(
	model: NetworkDetailsModel,
	rootNavigator: Navigator
) {
	Column(Modifier.background(MaterialTheme.colors.background)) {

		ScreenHeader(title = null, onBack = { rootNavigator.backAction() },
			onMenu = {
				//todo dmitry implement
			})
		Column(
			Modifier
				.weight(1f)
				.verticalScroll(rememberScrollState()),
			horizontalAlignment = Alignment.CenterHorizontally,
		) {

			NetworkIcon(networkLogoName = model.logo, size = 56.dp)
			Text(
				text = model.title,
				style = SignerTypeface.TitleM,
				color = MaterialTheme.colors.primary,
				modifier = Modifier
					.padding(start = 24.dp)
			)

			Text(
				text = "Chain Specs",
				style = SignerTypeface.BodyL,
				color = MaterialTheme.colors.secondary,
				modifier = Modifier
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
					name = "Network name",
					value = model.title
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = "base58 prefix",
					value = model.base58prefix.toString()
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = "decimals",
					value = model.decimals.toString()
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = "unit",
					value = model.unit.toString()
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = "Genesis hash",
					value = model.genesisHash,
					valueInSameLine = false
				)
				SignerDivider()
				TCNameValueOppositeElement(
					name = "Verifier Certificate",
					value = model.currentVerifier.ttype
				)

				if (model.meta.isNotEmpty()) {
					Text(
						text = "Metadata Available",
						style = SignerTypeface.BodyL,
						color = MaterialTheme.colors.secondary,
						modifier = Modifier
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
								name = "Version",
								value = metadata.specsVersion
							)
							SignerDivider()
							TCNameValueOppositeElement(
								name = "Hash",
								value = metadata.metaHash
							)
							SignerDivider()
							//Sign item
							SignerDivider()
							//delete item
							SignerDivider()
						}
					}
				}
			}
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
		)
	}
}

