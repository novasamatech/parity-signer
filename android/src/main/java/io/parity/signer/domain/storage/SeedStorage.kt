package io.parity.signer.domain.storage

import android.content.Context
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.os.Build
import android.security.keystore.UserNotAuthenticatedException
import timber.log.Timber
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import io.parity.signer.R
import io.parity.signer.domain.FeatureFlags
import io.parity.signer.domain.FeatureOption
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.screens.error.ErrorStateDestinationState
import io.parity.signer.uniffi.historySeedWasShown
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import java.security.UnrecoverableKeyException
import javax.crypto.AEADBadTagException


/**
 * Storing seed phrase in secure storage
 * All functions requiring user to be authenticated, properties do not.
 *
 * This is not safe class to use directly. Use wrappers that checks for authenticated.
 */
class SeedStorage {

	private val _lastKnownSeedNames = MutableStateFlow(arrayOf<String>())
	val lastKnownSeedNames: StateFlow<Array<String>> =
		_lastKnownSeedNames.asStateFlow()
	val isStrongBoxProtected: Boolean
		get() = masterKey.isStrongBoxBacked


	private lateinit var masterKey: MasterKey
	private var hasStrongbox: Boolean = false
	private lateinit var sharedPreferences: SharedPreferences
	private val KEYSTORE_NAME = "AndroidKeyStore"

	fun isInitialized(): Boolean = this::sharedPreferences.isInitialized

	/**
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun init(appContext: Context): OperationResult<Unit, ErrorStateDestinationState> {
		hasStrongbox = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
			appContext
				.packageManager
				.hasSystemFeature(PackageManager.FEATURE_STRONGBOX_KEYSTORE)
		} else {
			false
		}

		Timber.d("strongbox available:", hasStrongbox.toString())

		// Init crypto for seeds:
		// https://developer.android.com/training/articles/keystore
		masterKey = if (hasStrongbox) {
			MasterKey.Builder(appContext)
				.setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
				.setRequestStrongBoxBacked(true)
				.setUserAuthenticationRequired(
					true,
					MasterKey.getDefaultAuthenticationValidityDurationSeconds()
				)
				.build()
		} else {
			MasterKey.Builder(appContext)
				.setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
				.setUserAuthenticationRequired(
					true,
					MasterKey.getDefaultAuthenticationValidityDurationSeconds()
				)
				.build()
		}

		Timber.e("ENCRY", "$appContext $KEYSTORE_NAME $masterKey")
		//we need to be authenticated for this
		try {
			sharedPreferences =
				if (FeatureFlags.isEnabled(FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT)) {
					appContext.getSharedPreferences(
						"FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT",
						Context.MODE_PRIVATE
					)
				} else {
					EncryptedSharedPreferences(
						appContext,
						KEYSTORE_NAME,
						masterKey,
						EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
						EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
					)
				}
		} catch (e: Exception) {
			return OperationResult.Err(consumeStorageAuthError(e, appContext))
		}
		return OperationResult.Ok(Unit)
	}


	/**
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun getSeedNames(): Array<String> =
		sharedPreferences.all.keys.sorted().toTypedArray().also {
			_lastKnownSeedNames.value = it
		}

	/**
	 * Add seed, encrypt it, and create default accounts
	 *
	 * Don't forget to call tell rust seed names -so getSeedNames()
	 * called and last known elements updated
	 *
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun addSeed(
		seedName: String,
		seedPhrase: String,
	) {

		// First check for seed collision
		if (checkIfSeedNameAlreadyExists(seedPhrase)) {
			error("This seed phrase already exists")
		}

		// Encrypt and save seed
		with(sharedPreferences.edit()) {
			putString(seedName, seedPhrase)
			apply()
		}

		_lastKnownSeedNames.update { lastState ->
			lastState + seedName
		}
	}

	/**
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun checkIfSeedNameAlreadyExists(seedPhrase: String): Boolean {
		val result = sharedPreferences.all.values.contains(seedPhrase)
		Runtime.getRuntime().gc()
		return result
	}

	/**
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun getSeed(
		seedName: String,
		showInLogs: Boolean = false
	): String {
		val seedPhrase = sharedPreferences.getString(seedName, "") ?: ""
		return if (seedPhrase.isBlank()) {
			""
		} else {
			if (showInLogs) {
				historySeedWasShown(seedName)
			}
			seedPhrase
		}
	}

	/**
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun getBsPassword(
		seedName: String,
	): String {
		return sharedPreferences.getString("$seedName$BS_POSTFIX", "") ?: ""
	}

	/**
	 * @throws UserNotAuthenticatedException
	 * @throws IllegalArgumentException when name collision happening
	 */
	@Throws(IllegalArgumentException::class, UserNotAuthenticatedException::class)
	fun saveBsData(
		seedName: String,
		passPhrase: String,
	) {
		if (sharedPreferences.contains("$seedName$BS_POSTFIX")) {
			throw IllegalArgumentException("element with this name already exists in the storage")
		}
		sharedPreferences.edit()
			.putString("$seedName$BS_POSTFIX", passPhrase)
			.apply()
	}

	@Throws(UserNotAuthenticatedException::class)
	fun removeBSData(seedName: String) {
		sharedPreferences.edit()
			.remove("$seedName$BS_POSTFIX")
			.apply()
	}

	/**
	 * Don't forget to call tell rust seed names -so getSeedNames()
	 * called and last known elements updated
	 *
	 * @throws [UserNotAuthenticatedException]
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun removeSeed(seedName: String) {
		sharedPreferences.edit()
			.remove(seedName)
			.remove("$seedName$BS_POSTFIX")
			.apply()
		_lastKnownSeedNames.update { lastState ->
			lastState.filter { it != seedName }.toTypedArray()
		}
	}

	/**
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun wipe() {
		sharedPreferences.edit().clear().commit() // No, not apply(), do it now!
	}
}

internal fun consumeStorageAuthError(
	e: Exception,
	context: Context
): ErrorStateDestinationState {
	if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
		when (e) {
			is AEADBadTagException,
			is android.security.KeyStoreException,
			is UnrecoverableKeyException -> {
				return ErrorStateDestinationState(
					argHeader = context.getString(R.string.error_secure_storage_title),
					argDescription = context.getString(R.string.error_secure_storage_description),
					argVerbose = e.stackTraceToString()
				)
			}

			else -> throw e
		}
	} else {
		when (e) {
			is AEADBadTagException,
			is UnrecoverableKeyException -> {
				return ErrorStateDestinationState(
					argHeader = context.getString(R.string.error_secure_storage_title),
					argDescription = context.getString(R.string.error_secure_storage_description),
					argVerbose = e.stackTraceToString()
				)
			}

			else -> throw e
		}
	}
}

private const val BS_POSTFIX = "\$\$bs_passphrase"



