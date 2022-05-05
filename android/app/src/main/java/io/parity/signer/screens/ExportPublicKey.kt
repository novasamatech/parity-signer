package io.parity.signer.screens

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
import androidx.compose.ui.unit.dp
import io.parity.signer.components.HeaderBar
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.uniffi.MKeyDetails

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
			// TODO: KeyCard(identity = address)
		}
		Row(
			Modifier.padding(top = 3.dp, start = 12.dp, end = 12.dp)
		) {
			// TODO: NetworkCard(address)
		}
		Image(
			keyDetails.qr.intoImageBitmap(),
			contentDescription = "QR with address to scan",
			contentScale = ContentScale.FillWidth,
			modifier = Modifier.padding(12.dp).fillMaxWidth(1f)
		)
		HeaderBar(line1 = "KEY DETAILS", line2 = "")
		Row {
			Text("Base58 key:")
			Text(keyDetails.base58)
		}
		Row {
			Text("Hex key:")
			Text(keyDetails.pubkey)
		}
		Row {
			Text("Seed name:")
			Text(keyDetails.seedName)
		}
	}
}
