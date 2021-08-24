package io.parity.signer.models

import android.content.Context
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import io.parity.signer.MainActivity

/**
 * This is single object to handle all interactions with backend,
 * except for some logging features and transaction handling
 */
class SignerDataModel: ViewModel() {
	private val _onBoardingDone = MutableLiveData(false)
	val onBoardingDone: LiveData<Boolean> = _onBoardingDone
	lateinit var context: Context

	fun totalRefresh() {
		_onBoardingDone.value = true
	}

}
