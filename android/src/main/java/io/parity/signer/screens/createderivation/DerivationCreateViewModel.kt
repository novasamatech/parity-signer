package io.parity.signer.screens.createderivation

import android.content.Context
import android.util.Log
import android.widget.Toast
import androidx.lifecycle.ViewModel
import io.parity.signer.R
import io.parity.signer.backend.mapError
import io.parity.signer.dependencygraph.ServiceLocator
import io.parity.signer.models.Navigator
import io.parity.signer.models.NetworkModel
import io.parity.signer.models.storage.SeedRepository
import io.parity.signer.models.storage.mapError
import io.parity.signer.uniffi.DerivationCheck
import io.parity.signer.uniffi.tryCreateAddress
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

	fun setInitValues(seed: String, rootNavigator: Navigator) {
		seedName = seed
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
				selectedNetwork.value.key
			).mapError()
		}
	}


	suspend fun proceedCreateKey(context: Context) {
		try {
			val phrase =
				seedRepository.getSeedPhraseForceAuth(seedName).mapError() ?: return
			if (phrase.isNotBlank()) {
				try {
					tryCreateAddress(seedName, phrase, path.value, selectedNetwork.value.key)
					Toast.makeText(
						context,
						context.getString(R.string.create_derivations_success),
						Toast.LENGTH_SHORT
					).show()
				} catch (e: Exception) {
					Toast.makeText(
						context, context.getString(R.string.create_derivations_failure, e.localizedMessage),
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

	enum class DerivationPathValidity {
		ALL_GOOD, WRONG_PATH, COLLISION_PATH, EMPTY_PASSWORD, CONTAIN_SPACES
	}
}

internal const val INITIAL_DERIVATION_PATH = "//"
private const val TAG = "DerivationCreateViewModel"

