package io.parity.signer.screens.settings.general

import android.content.Context
import androidx.lifecycle.ViewModel
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Callback
import io.parity.signer.domain.NetworkState
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.usecases.ResetUseCase
import io.parity.signer.screens.error.ErrorStateDestinationState
import kotlinx.coroutines.flow.StateFlow


class SettingsGeneralViewModel: ViewModel() {

	private val resetUseCase: ResetUseCase = ResetUseCase()

	val isStrongBoxProtected: Boolean = ServiceLocator.seedStorage.isStrongBoxProtected

	fun getAppVersion(context: Context): String {
		return context.packageManager.getPackageInfo(
			context.packageName,
			0
		).versionName!!
	}

	val networkState: StateFlow<NetworkState> =
		ServiceLocator.networkExposedStateKeeper.airGapModeState


	/**
	 * Auth user and wipe the Vault to initial state
	 */
	suspend fun wipeToFactory(onAfterWipe: Callback): OperationResult<Unit, ErrorStateDestinationState> {
		return resetUseCase.wipeToFactoryWithAuth(onAfterWipe)
	}
}
