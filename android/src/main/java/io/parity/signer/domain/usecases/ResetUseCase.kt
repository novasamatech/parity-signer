package io.parity.signer.domain.usecases

import android.widget.Toast
import androidx.fragment.app.FragmentActivity
import io.parity.signer.R
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.AuthResult
import io.parity.signer.domain.Callback
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.getDbNameFromContext
import io.parity.signer.domain.isDbCreatedAndOnboardingPassed
import io.parity.signer.domain.storage.DatabaseAssetsInteractor
import io.parity.signer.screens.error.ErrorStateDestinationState
import io.parity.signer.uniffi.historyInitHistoryNoCert
import io.parity.signer.uniffi.historyInitHistoryWithCert
import io.parity.signer.uniffi.initNavigation


class ResetUseCase {

	private val seedStorage = ServiceLocator.seedStorage
	private val databaseAssetsInteractor: DatabaseAssetsInteractor =
		ServiceLocator.databaseAssetsInteractor
	private val appContext = ServiceLocator.appContext
	private val networkExposedStateKeeper =
		ServiceLocator.networkExposedStateKeeper
	private val activity: FragmentActivity
		get() = ServiceLocator.activityScope!!.activity

	suspend fun wipeToFactoryWithAuth(onAfterWipe: Callback): OperationResult<Unit, ErrorStateDestinationState> {
		val authentication = ServiceLocator.authentication
		return when (authentication.authenticate(activity)) {
			AuthResult.AuthError,
			AuthResult.AuthFailed ,
			AuthResult.AuthUnavailable -> {
				Toast.makeText(
					activity.baseContext,
					activity.baseContext.getString(R.string.auth_failed_message),
					Toast.LENGTH_SHORT
				).show()
				OperationResult.Ok(Unit)
			}
			AuthResult.AuthSuccess -> {
				databaseAssetsInteractor.wipe()
				val result = totalRefresh()
				onAfterWipe()
				return result
			}
		}
	}

	/**
	 * Auth user and wipe Vault to state without general verifier certificate
	 */
	suspend fun wipeNoGeneralCertWithAuth(onAfterWide: Callback): OperationResult<Unit, ErrorStateDestinationState> {
		val authentication = ServiceLocator.authentication
		return when (authentication.authenticate(activity)) {
			AuthResult.AuthError,
			AuthResult.AuthFailed,
			AuthResult.AuthUnavailable -> {
				Toast.makeText(
					activity.baseContext,
					activity.baseContext.getString(R.string.auth_failed_message),
					Toast.LENGTH_SHORT
				).show()
				OperationResult.Ok(Unit)
			}
			AuthResult.AuthSuccess -> {
				databaseAssetsInteractor.wipe()
				databaseAssetsInteractor.copyAsset("")
				val result = totalRefresh()
				historyInitHistoryNoCert()
				onAfterWide()
				return result
			}
		}
	}

	private fun totalRefreshDbExist() {
		val allNames = seedStorage.getSeedNames()
		initNavigation(appContext.getDbNameFromContext(), allNames.toList())
		ServiceLocator.uniffiInteractor.wasRustInitialized.value = true
		networkExposedStateKeeper.updateAlertStateFromHistory()
	}

	/**
	 * Populate database!
	 * This is first start of the app
	 */
	private fun initAssetsAndTotalRefresh() {
		databaseAssetsInteractor.wipe()
		databaseAssetsInteractor.copyAsset("")
		totalRefreshDbExist()
		historyInitHistoryWithCert()
	}

	/**
	 * This returns the app into starting state;
	 * Do not forget to reset navigation UI state!
	 */
	fun totalRefresh(): OperationResult<Unit, ErrorStateDestinationState> {
		if (!seedStorage.isInitialized()) {
			val result = seedStorage.init(appContext)
			if (result is OperationResult.Err) {
				return result
			}
		}
		if (!appContext.isDbCreatedAndOnboardingPassed()) {
			initAssetsAndTotalRefresh()
		} else {
			totalRefreshDbExist()
		}
		return OperationResult.Ok(Unit)
	}
}
