package io.parity.signer.screens.createderivation

import android.content.Context
import android.util.Log
import android.widget.Toast
import androidx.lifecycle.ViewModel
import io.parity.signer.R
import io.parity.signer.domain.backend.mapError
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.domain.Navigator
import io.parity.signer.domain.NetworkModel
import io.parity.signer.domain.storage.SeedRepository
import io.parity.signer.domain.storage.mapError
import io.parity.signer.domain.usecases.AllNetworksUseCase
import io.parity.signer.uniffi.DerivationCheck
import io.parity.signer.uniffi.tryCreateAddress
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext


class DerivationCreateViewModel : ViewModel() {

	private val uniffiInteractor = ServiceLocator.uniffiInteractor
	private var seedRepository: SeedRepository =
		ServiceLocator.activityScope!!.seedRepository
	private val pathAnalyzer: DerivationPathAnalyzer = DerivationPathAnalyzer()
	private val allNetworksUseCase = AllNetworksUseCase(uniffiInteractor)

	fun getAllNetworks(): List<NetworkModel> = allNetworksUseCase.getAllNetworks()

	private lateinit var rootNavigator: Navigator
	private lateinit var seedName: String

	private val _path: MutableStateFlow<String> =
		MutableStateFlow(INITIAL_DERIVATION_PATH)
	val path: StateFlow<String> = _path.asStateFlow()

	private val _selectedNetwork: MutableStateFlow<SelectedNetwork> =
		MutableStateFlow(SelectedNetwork.Network(getAllNetworks().first()))
	val selectedNetwork: StateFlow<SelectedNetwork> =
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

	fun setInitValues(seed: String, rootNavigator: Navigator) {
		refreshCachedDependencies()
		seedName = seed
		this.rootNavigator = rootNavigator
	}

	fun updateSelectedNetwork(newNetwork: SelectedNetwork) {
		_selectedNetwork.value = newNetwork
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
		return when (val selectedNetwork = selectedNetwork.value) {
			SelectedNetwork.AllNetworks -> runBlocking {
				getAllNetworks()
					.map { network ->
						async(Dispatchers.IO) {
							uniffiInteractor.validateDerivationPath(
								path,
								seedName,
								network.key
							).mapError()
						}
					}
					.map { it.await() }
					.firstOrNull { it?.buttonGood == false || it?.collision != null
						|| it?.error != null }
			}
			is SelectedNetwork.Network -> runBlocking {
				uniffiInteractor.validateDerivationPath(
					path,
					seedName,
					selectedNetwork.networkModel.key
				).mapError()
			}
		}
	}


	suspend fun proceedCreateKey(context: Context) {
		try {
			val phrase =
				seedRepository.getSeedPhraseForceAuth(seedName).mapError() ?: return
			if (phrase.isNotBlank()) {
				try {
					when (val selectedNetwork = selectedNetwork.value) {
						SelectedNetwork.AllNetworks -> {
							withContext(Dispatchers.IO) {
								getAllNetworks()
									.map {
										async {
											tryCreateAddress(
												seedName,
												phrase,
												path.value,
												it.key,
											)
										}
									}
									.map { it.await() }
							}
						}
						is SelectedNetwork.Network -> tryCreateAddress(
							seedName,
							phrase,
							path.value,
							selectedNetwork.networkModel.key
						)
					}
					Toast.makeText(
						context,
						context.getString(R.string.create_derivations_success),
						Toast.LENGTH_SHORT
					).show()
					resetState()
				} catch (e: Exception) {
					Toast.makeText(
						context,
						context.getString(
							R.string.create_derivations_failure,
							e.localizedMessage
						),
						Toast.LENGTH_SHORT
					).show()
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

sealed class SelectedNetwork {
	class Network(val networkModel: NetworkModel) : SelectedNetwork()
	object AllNetworks : SelectedNetwork()
}

internal const val INITIAL_DERIVATION_PATH = "//"
private const val TAG = "DerivationCreateViewModel"

