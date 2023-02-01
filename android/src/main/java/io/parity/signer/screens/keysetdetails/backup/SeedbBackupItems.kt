package io.parity.signer.screens.keysetdetails.backup

import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.KeySetDetailsModel

data class SeedBackupModel(
	val seedName: String,
	val seedBase58: String,
	val derivations: List<KeyModel>
)
fun KeySetDetailsModel.toSeedBackupModel(): SeedBackupModel? {
	val root = root ?: return null
	return SeedBackupModel(root.seedName, root.base58, keysAndNetwork.map { it.key })
}
