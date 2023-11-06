package io.parity.signer.screens.createderivation

import android.content.Context
import android.util.Log
import android.widget.Toast
import androidx.lifecycle.ViewModel
import io.parity.signer.R
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.KeyAndNetworkModel
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.backend.OperationResult
import io.parity.signer.domain.backend.mapError
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.storage.mapError
import io.parity.signer.domain.toKeyAndNetworkModel
import io.parity.signer.domain.usecases.AllNetworksUseCase
import io.parity.signer.domain.usecases.CreateDerivationUseCase
import io.parity.signer.uniffi.DerivationCheck
import io.parity.signer.uniffi.keysBySeedName
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.runBlocking


class DerivationCreateViewModel : ViewModel() {

	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private var seedRepository: SeedRepository =
		ServiceLocator.activityScope!!.seedRepository
	private val pathAnalyzer: DerivationPathAnalyzer = DerivationPathAnalyzer()
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)
	private val createDerivationUseCase = CreateDerivationUseCase()

	fun getAllNetworks(): List<NetworkModel> = allNetworksUseCase.getAllNetworks()

	private lateinit var seedName: String
	private lateinit var existingKeys: Set<KeyAndNetworkModel>

	private val _path: MutableStateFlow<String> =
		MutableStateFlow(INITIAL_DERIVATION_PATH)
	val path: StateFlow<String> = _path.asStateFlow()

	private val _selectedNetwork: MutableStateFlow<NetworkModel?> =
		MutableStateFlow(null)
	val selectedNetwork: StateFlow<NetworkModel?> =
		_selectedNetwork.asStateFlow()

	fun updatePath(newPath: String) {
		_path.value = newPath
	}

	/**
	 * should be called each time we open this flow again
	 */
	fun refreshCachedDependencies() {
		allNetworksUseCase.updateCache()
		seedRepository = ServiceLocator.activityScope!!.seedRepository
	}

	fun setInitValues(seed: String) {
		refreshCachedDependencies()
		seedName = seed
		existingKeys =
			keysBySeedName(seed).set.map { it.toKeyAndNetworkModel() }.toSet()
	}

	fun updateSelectedNetwork(newNetwork: NetworkModel) {
		_selectedNetwork.value = newNetwork
		_path.value = getInitialPath(newNetwork)
	}

	suspend fun fastCreateDerivationForNetwork(
		newNetwork: NetworkModel,
		context: Context
	) {
		updateSelectedNetwork(newNetwork)
		proceedCreateKey(context)
	}

	private fun getInitialPath(netWork: NetworkModel): String {
		var path = netWork.pathId
		val keys = existingKeys.filter { it.network.networkSpecsKey == netWork.key }
		if (!keys.any { it.key.path == path }) {
			return path
		} else {
			for (i in 0..Int.MAX_VALUE) {
				path = "${netWork.pathId}//$i"
				if (!keys.any { it.key.path == path }) {
					return path
				}
			}
		}
		return path
	}

	fun checkPath(path: String): DerivationPathValidity {
		return when {
			DerivationPathAnalyzer.getPassword(path)
				?.isEmpty() == true -> DerivationPathValidity.EMPTY_PASSWORD

			path.contains(' ') -> DerivationPathValidity.CONTAIN_SPACES
			!pathAnalyzer.isCorrect(path) -> DerivationPathValidity.WRONG_PATH
			else -> {
				val backendCheck = getBackendCheck(path)
				when {
					backendCheck?.collision != null -> DerivationPathValidity.COLLISION_PATH
					backendCheck?.buttonGood == false -> DerivationPathValidity.WRONG_PATH
					else -> DerivationPathValidity.ALL_GOOD
				}
			}
		}
	}

	private fun getBackendCheck(path: String): DerivationCheck? {
		return runBlocking {
			uniffiInteractor.validateDerivationPath(
				path,
				seedName,
				selectedNetwork.value?.key ?: ""
			).mapError()
		}
	}


	suspend fun proceedCreateKey(context: Context) {
		try {
			val phrase =
				seedRepository.getSeedPhraseForceAuth(seedName).mapError() ?: return
			if (phrase.isNotBlank()) {
				val selectedNetwork = selectedNetwork.value!!
				val result = createDerivationUseCase.createDerivation(
					seedName = seedName,
					seedPhrase = phrase,
					path = path.value,
					networkKey = selectedNetwork.key
				)
				when (result) {
					is OperationResult.Ok -> {
						Toast.makeText(
							context,
							context.getString(R.string.create_derivations_success),
							Toast.LENGTH_SHORT
						).show()
						resetState()
					}

					is OperationResult.Err -> {
						Toast.makeText(
							context,
							context.getString(
								R.string.create_derivations_failure,
								result.error.localizedMessage
							),
							Toast.LENGTH_SHORT
						).show()
					}
				}
			} else {
				Log.e(TAG, "Seed phrase received but it's empty")
			}
		} catch (e: java.lang.Exception) {
			Log.e(TAG, e.toString())
		}
	}

	fun resetState() {
		_path.value = INITIAL_DERIVATION_PATH
	}

	enum class DerivationPathValidity {
		ALL_GOOD, WRONG_PATH, COLLISION_PATH, EMPTY_PASSWORD, CONTAIN_SPACES
	}
}

internal const val INITIAL_DERIVATION_PATH = "//"
private const val TAG = "DerivationCreateViewModel"

