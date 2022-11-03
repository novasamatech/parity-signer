package io.parity.signer.screens.keysetdetails.backup

import io.parity.signer.uniffi.MBackup
import io.parity.signer.uniffi.DerivationPack

/**
 * Local copy of shared [MBackup] class
 */
data class SeedBackupModel(
	val seedName: String,
	val derivations: List<BackupDerivationModel>
)
fun MBackup.toSeedBackupModel() =
	SeedBackupModel(seedName, derivations.map { it.toBackupDerivationModel() })

/**
 * Local copy of shared [DerivationPack] class
 */
data class BackupDerivationModel(
	val networkTitle: String,
	val networkLogo: String,
	//can parse more data from DerivationPack
)

fun DerivationPack.toBackupDerivationModel() =
	BackupDerivationModel(networkTitle, networkLogo)
