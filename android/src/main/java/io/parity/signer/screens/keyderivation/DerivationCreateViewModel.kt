package io.parity.signer.screens.keyderivation

import android.util.Log
import androidx.lifecycle.ViewModel
import io.parity.signer.backend.mapError
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.Navigator
import io.parity.signer.models.NetworkModel
import io.parity.signer.models.storage.SeedRepository
import io.parity.signer.models.storage.mapError
import io.parity.signer.uniffi.Action
import kotlinx.coroutines.runBlocking


class DerivationCreateViewModel : ViewModel() {

	private val uniffiInteractor = ServiceLocator.backendScope.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private val pathAnalyzer: DerivationPathAnalyzer = DerivationPathAnalyzer()

	private lateinit var rootNavigator: Navigator
	private lateinit var seedName: String
	private lateinit var selectedNetworkSpecs: String

	fun setInitValues(seed: String, network: String, rootNavigator: Navigator) {
		seedName = seed
		selectedNetworkSpecs = network
		this.rootNavigator = rootNavigator
	}

	private suspend fun getNetworks(): List<NetworkModel>? {
		return uniffiInteractor.getAllNetworks().mapError()
	}

	fun checkPath(path: String): DerivationPathValidity {
		return when {
			pathAnalyzer.getPassword(path) == null -> DerivationPathValidity.EMPTY_PASSWORD
			!pathAnalyzer.isCorrect(path) -> DerivationPathValidity.WRONG_PATH
//			uniffiInteractor.validateDerivationPath(path).mapError() //todo derivation
			else -> DerivationPathValidity.ALL_GOOD
		}
	}

	private fun getBackendCheck(path: String) {
		runBlocking {
			uniffiInteractor.validateDerivationPath(
				path,
				seedName,
				selectedNetworkSpecs
			).mapError()
		}
	}


	suspend fun addKey(path: String, seedName: String) {
		try {
			val phrase = seedRepository.getSeedPhraseForceAuth(seedName).mapError()
			if (phrase?.isNotBlank() == true) {
				rootNavigator.navigate(Action.GO_FORWARD, path, phrase)
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
		}
	}

	enum class DerivationPathValidity {
		ALL_GOOD, WRONG_PATH, COLLISION_PATH, EMPTY_PASSWORD,
	}
}




