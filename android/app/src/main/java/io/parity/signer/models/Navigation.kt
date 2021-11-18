package io.parity.signer.models

import android.util.Log
import androidx.core.app.ActivityCompat
import io.parity.signer.KeyManagerModal
import io.parity.signer.SettingsModal
import io.parity.signer.SignerScreen
import io.parity.signer.TransactionState

//MARK: Navigation begin

/**
 * Bottom navigation action
 */
fun SignerDataModel.navigate(screen: SignerScreen) {
	_signerScreen.value = screen
	if (screen == SignerScreen.Scan) {
		//TODO: testing to make sure this goes smoothly
		if (!allPermissionsGranted()) {
			ActivityCompat.requestPermissions(
				activity,
				REQUIRED_PERMISSIONS,
				REQUEST_CODE_PERMISSIONS
			)
		}
	}
	if (screen == SignerScreen.Keys) {
		selectSeedEngage()
	}
	if (screen == SignerScreen.Log) {
		engageHistoryScreen()
	}
}

/**
 * Handle back button
 */
fun SignerDataModel.goBack() {
	when (signerScreen.value) {
		SignerScreen.Log -> {
			totalRefresh()
		}
		SignerScreen.Scan -> {
			clearTransaction()
		}
		SignerScreen.Keys -> {
			when (keyManagerModal.value) {
				KeyManagerModal.None -> {
					selectSeedEngage()
				}
				KeyManagerModal.NewSeed -> {
					selectSeedEngage()
				}
				KeyManagerModal.NewKey -> {
					clearKeyManagerScreen()
				}
				KeyManagerModal.ShowKey -> {
					clearKeyManagerScreen()
				}
				KeyManagerModal.SeedBackup -> {
					selectSeedEngage()
				}
				KeyManagerModal.KeyDeleteConfirm -> {
					clearKeyManagerScreen()
				}
				KeyManagerModal.SeedSelector -> {
					selectSeedEngage()
				}
				KeyManagerModal.NetworkManager -> {
					clearKeyManagerScreen()
				}
				KeyManagerModal.NetworkDetails -> {
					clearKeyManagerScreen()
				}
			}
		}
		SignerScreen.Settings -> {
			clearHistoryScreen()
		}
	}
}

fun SignerDataModel.isBottom(): Boolean {
	return (settingsModal.value == SettingsModal.None && keyManagerModal.value == KeyManagerModal.SeedSelector && transactionState.value == TransactionState.None)
}

fun SignerDataModel.getScreenName(): String {
	Log.d("getscreenname", "called")
	return when (signerScreen.value) {
		SignerScreen.Scan -> ""
		SignerScreen.Keys -> when (keyManagerModal.value) {
			KeyManagerModal.None -> ""
			KeyManagerModal.NewSeed -> ""
			KeyManagerModal.NewKey -> "New Derived Key"
			KeyManagerModal.ShowKey -> if (selectedIdentity.value == getRootIdentity(
					selectedSeed.value ?: ""
				)
			) {
				"Seed key"
			} else {
				"Derived Key"
			}
			KeyManagerModal.SeedBackup -> "Backup Seed"
			KeyManagerModal.KeyDeleteConfirm -> ""
			KeyManagerModal.SeedSelector -> "Select Seed"
			KeyManagerModal.NetworkManager -> ""
			KeyManagerModal.NetworkDetails -> ""
			null -> "error"
		}
		SignerScreen.Settings -> ""
		SignerScreen.Log -> "History"
		null -> "error"
	}
}
//MARK: Navigation end

//MARK: Modals control begin

//KeyManager

/**
 * This happens when backup seed acknowledge button is pressed in seed creation screen.
 * TODO: This might misfire
 */
fun SignerDataModel.acknowledgeBackup() {
	_backupSeedPhrase.value = ""
	clearKeyManagerScreen()
}

/**
 * Use this to bring up seed selection screen in key manager
 */
private fun SignerDataModel.selectSeedEngage() {
	selectSeed("")
	_keyManagerModal.value = KeyManagerModal.SeedSelector
}

/**
 * Activate new seed screen on KeyManager screen
 */
fun SignerDataModel.newSeedScreenEngage() {
	_keyManagerModal.value = KeyManagerModal.NewSeed
}

/**
 * Derive new key
 */
fun SignerDataModel.newKeyScreenEngage() {
	_keyManagerModal.value = KeyManagerModal.NewKey
}

/**
 * Show public key QR screen
 */
fun SignerDataModel.exportPublicKeyEngage() {
	_keyManagerModal.value = KeyManagerModal.ShowKey
}

/**
 * Remove key manager modals
 */
fun SignerDataModel.clearKeyManagerScreen() {
	_keyManagerModal.value = if (selectedSeed.value == "") {
		KeyManagerModal.SeedSelector
	} else {
		KeyManagerModal.None
	}
}

/**
 * Key deletion confirmation
 */
fun SignerDataModel.deleteKeyConfirmation() {
	_keyManagerModal.value = KeyManagerModal.KeyDeleteConfirm
}

//Settings

private fun SignerDataModel.engageHistoryScreen() {
	refreshHistory()
	getGeneralVerifier()
	_signerScreen.value = SignerScreen.Log
}

private fun SignerDataModel.clearHistoryScreen() {
	_settingsModal.value = SettingsModal.None
}

//MARK: Modals control end
