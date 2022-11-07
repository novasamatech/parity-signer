package io.parity.signer.screens.keysetdetails.backup

import io.parity.signer.models.KeyModel
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.uniffi.MBackup
import io.parity.signer.uniffi.DerivationPack

data class SeedBackupModel(
	val seedName: String,
	val seedBase58: String,
	val derivations: List<KeyModel>
)
fun KeySetDetailsModel.toSeedBackupModel() =
	SeedBackupModel(root.seedName, root.base58, keys)
