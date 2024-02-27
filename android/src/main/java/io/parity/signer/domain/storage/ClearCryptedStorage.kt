package io.parity.signer.domain.storage

import android.content.Context
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.os.Build
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.screens.error.ErrorStateDestinationState
import timber.log.Timber


/**
 * Entrypted storage that doesn't require authentication to read the values
 */
class ClearCryptedStorage {
	private lateinit var sharedPreferences: SharedPreferences

	fun init(appContext: Context): OperationResult<Unit, ErrorStateDestinationState> {
		val hasStrongbox = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
			appContext
				.packageManager
				.hasSystemFeature(PackageManager.FEATURE_STRONGBOX_KEYSTORE)
		} else {
			false
		}

		Timber.d(TAG, "strongbox available: $hasStrongbox")

		// Init crypto for seeds:
		// https://developer.android.com/training/articles/keystore
		val masterKey = if (hasStrongbox) {
			MasterKey.Builder(appContext)
				.setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
				.setRequestStrongBoxBacked(true)
				.setUserAuthenticationRequired(false)
				.build()
		} else {
			MasterKey.Builder(appContext)
				.setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
				.setUserAuthenticationRequired(false)
				.build()
		}

		try {
			sharedPreferences = EncryptedSharedPreferences(
				appContext,
				KEYSTORE_NAME,
				masterKey,
				EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
				EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
			)
		} catch (e: Exception) {
			return OperationResult.Err(consumeStorageAuthError(e, appContext))
		}
		return OperationResult.Ok(Unit)
	}
}

private const val TAG = "ClearCryptedStorage"
private const val KEYSTORE_NAME = "ClearCryptedStorage"
