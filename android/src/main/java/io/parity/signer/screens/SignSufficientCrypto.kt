package io.parity.signer.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.parity.signer.components.KeyCardOld
import io.parity.signer.uniffi.Address
import io.parity.signer.uniffi.MSignSufficientCrypto

@Composable
fun SignSufficientCrypto(
	sc: MSignSufficientCrypto,
	signSufficientCrypto: (seedName: String, addressKey: String) -> Unit
) {
	val identities = sc.identities
	Column {
		Text("Select key for signing")
		LazyColumn {
			items(identities.size) { index ->
				val identity = identities[index]
				Row(Modifier.clickable {
					signSufficientCrypto(identity.seedName, identity.addressKey)
				}) {
					KeyCardOld(identity = Address(
						base58 = identity.publicKey,
						path = identity.path,
						hasPwd = identity.hasPwd,
						identicon = identity.identicon,
						seedName = identity.seedName,
						secretExposed = identity.secretExposed,
						multiselect = null
					))
				}
			}
		}
	}
}
