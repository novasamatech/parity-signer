package io.parity.signer.domain.storage

import android.annotation.SuppressLint
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

	private lateinit var seedSharedPreferences: SharedPreferences
	private lateinit var associatedValuesSharedPreferences: SharedPreferences

	// This is wrongly named from beginning, but we need a migration from one pref file to another to change that
	private val MAIN_SEED_PREF_FILE = "AndroidKeyStore"
	private val ASSOCIATED_VALUES_PREF_FILE = "SeedAssociatedValues"

	// This is wrongly named from beginning, but we need a migration from one pref file to another to change that
	private val DEV_MAIN_SEED_FILE_NAME = "FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT"
	private val DEV_ASSOCIATED_VALUES_PREF_FILE = "DevSeedAssociatedValues"

	fun isInitialized(): Boolean = this::seedSharedPreferences.isInitialized

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

		//we need to be authenticated for this
		try {
			seedSharedPreferences = createEncryptedPrefsFile(
				appContext = appContext,
				filename = MAIN_SEED_PREF_FILE,
				developmentFileName = DEV_MAIN_SEED_FILE_NAME
			)
			associatedValuesSharedPreferences = createEncryptedPrefsFile(
				appContext = appContext,
				filename = ASSOCIATED_VALUES_PREF_FILE,
				developmentFileName = DEV_ASSOCIATED_VALUES_PREF_FILE
			)
		} catch (e: Exception) {
			return OperationResult.Err(consumeStorageAuthError(e, appContext))
		}
		return OperationResult.Ok(Unit)
	}

	private fun createEncryptedPrefsFile(
		appContext: Context,
		filename: String,
		developmentFileName: String
	): SharedPreferences {
		return if (FeatureFlags.isEnabled(FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT)) {
			appContext.getSharedPreferences(
				developmentFileName,
				Context.MODE_PRIVATE
			)
		} else {
			EncryptedSharedPreferences(
				appContext,
				filename,
				masterKey,
				EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
				EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
			)
		}
	}

	/**
	 * @throws UserNotAuthenticatedException
	 */
	@Throws(UserNotAuthenticatedException::class)
	fun getSeedNames(): Array<String> =
		seedSharedPreferences.all.keys.sorted().toTypedArray().also {
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
		with(seedSharedPreferences.edit()) {
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
		val result = seedSharedPreferences.all.values.contains(seedPhrase)
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
		val seedPhrase = seedSharedPreferences.getString(seedName, "") ?: ""
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
		val keyName = SeedAssociatedValue.BANANA_SPLIT_PASSWORD.keyName(seedName)
		return associatedValuesSharedPreferences.getString(keyName, "") ?: ""
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
		val keyName = SeedAssociatedValue.BANANA_SPLIT_PASSWORD.keyName(seedName)
		if (associatedValuesSharedPreferences.contains(keyName)) {
			throw IllegalArgumentException("element with this name already exists in the storage")
		}
		associatedValuesSharedPreferences.edit()
			.putString(keyName, passPhrase)
			.apply()
	}

	@Throws(UserNotAuthenticatedException::class)
	fun removeBSData(seedName: String) {
		val keyName = SeedAssociatedValue.BANANA_SPLIT_PASSWORD.keyName(seedName)
		associatedValuesSharedPreferences.edit()
			.remove(keyName)
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
		seedSharedPreferences.edit()
			.remove(seedName)
			.apply()

		associatedValuesSharedPreferences.edit()
			.apply {
				SeedAssociatedValue.entries.forEach { seedAssociatedValue ->
					remove(seedAssociatedValue.keyName(seedName))
				}
			}.apply()

		_lastKnownSeedNames.update { lastState ->
			lastState.filter { it != seedName }.toTypedArray()
		}
	}

	/**
	 * @throws UserNotAuthenticatedException
	 */
	@SuppressLint("ApplySharedPref")
	@Throws(UserNotAuthenticatedException::class)
	fun wipe() {
		seedSharedPreferences.edit().clear().commit() // No, not apply(), do it now!
		associatedValuesSharedPreferences.edit().clear().commit()
	}

	private enum class SeedAssociatedValue(private val keySuffix: String) {
		BANANA_SPLIT_PASSWORD("bs_passphrase");

		fun keyName(seedName: String): String {
			return seedName + "\$\$" + keySuffix
		}
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


