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
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.runBlocking


class DerivationCreateViewModel : ViewModel() {

	private val uniffiInteractor = ServiceLocator.backendScope.uniffiInteractor
	private val seedRepository: SeedRepository by lazy { ServiceLocator.activityScope!!.seedRepository }
	private val pathAnalyzer: DerivationPathAnalyzer = DerivationPathAnalyzer()

	private lateinit var rootNavigator: Navigator
	private lateinit var seedName: String

	val allNetworks: List<NetworkModel> = runBlocking { getNetworks() }!!

	private val _path: MutableStateFlow<String> =
		MutableStateFlow(INITIAL_DERIVATION_PATH)
	val path: StateFlow<String> = _path.asStateFlow()

	private val _selectedNetwork: MutableStateFlow<NetworkModel> =
		MutableStateFlow(allNetworks.first())
	val selectedNetwork: StateFlow<NetworkModel> = _selectedNetwork.asStateFlow()

	fun updatePath(newPath: String) {
		_path.value = newPath
	}

	fun setInitValues(seed: String, network: String, rootNavigator: Navigator) {
		seedName = seed
		allNetworks.firstOrNull { it.key == network }
			?.let { _selectedNetwork.value = it }
		this.rootNavigator = rootNavigator
	}

	private suspend fun getNetworks(): List<NetworkModel>? {
		return uniffiInteractor.getAllNetworks().mapError()
	}

	fun updateSelectedNetwork(newNetwork: NetworkModel) {
		_selectedNetwork.value = newNetwork
	}

	fun checkPath(path: String): DerivationPathValidity {
		return when {
			DerivationPathAnalyzer.getPassword(path) == null -> DerivationPathValidity.EMPTY_PASSWORD
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
				selectedNetwork.value.key
			).mapError()
		}
	}


	suspend fun proceedCreateKey() {
		try {
			val phrase = seedRepository.getSeedPhraseForceAuth(seedName).mapError()
			if (phrase?.isNotBlank() == true) {
				rootNavigator.navigate(Action.GO_FORWARD, path.value, phrase)
			}
		} catch (e: java.lang.Exception) {
			Log.e("Add key error", e.toString())
		}
	}

	enum class DerivationPathValidity {
		ALL_GOOD, WRONG_PATH, COLLISION_PATH, EMPTY_PASSWORD,
	}
}

internal const val INITIAL_DERIVATION_PATH = "//"


