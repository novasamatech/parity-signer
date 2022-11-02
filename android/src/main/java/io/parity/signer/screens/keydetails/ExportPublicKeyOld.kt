package io.parity.signer.screens.keydetails

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.HeaderBar
import io.parity.signer.components.KeyCardOld
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.uniffi.MAddressCard
import io.parity.signer.uniffi.MKeyDetails

/**
 * Other name is Key Details screen
 */
@Composable
fun ExportPublicKey(keyDetails: MKeyDetails) {
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.verticalScroll(rememberScrollState())
	) {
		Row(
			Modifier
				.padding(top = 3.dp, start = 12.dp, end = 12.dp)
				.background(
					MaterialTheme.colors.Bg200
				)
				.fillMaxWidth()
		) {
			KeyCardOld(identity = MAddressCard(
				address = keyDetails.address,
				base58 = keyDetails.base58,
				multiselect = keyDetails.multiselect,
			))
		}
		Row(
			Modifier.padding(top = 3.dp, start = 12.dp, end = 12.dp)
		) {
			NetworkCard(
				network = NetworkCardModel(keyDetails.networkInfo)
			)
		}
		Image(
			keyDetails.qr.intoImageBitmap(),
			contentDescription = stringResource(id = R.string.qr_with_address_to_scan_description),
			contentScale = ContentScale.FillWidth,
			modifier = Modifier
				.padding(12.dp)
				.fillMaxWidth(1f)
		)
		HeaderBar(line1 = "KEY DETAILS", line2 = "")
		Row {
			Text("Base58 key: ")
			Text(keyDetails.base58)
		}
		Row {
			Text("Hex key: ")
			Text(keyDetails.pubkey)
		}
		Row {
			Text("Seed name: ")
			Text(keyDetails.address.seedName)
		}
	}
}
