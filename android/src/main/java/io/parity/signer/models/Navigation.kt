package io.parity.signer.models

import android.util.Log
import android.widget.Toast
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.backendAction


@Deprecated("obsolete, for backwards compatibility, use SignerNavigator class")
fun SignerDataModel.navigate(
	button: Action,
	details: String = "",
	seedPhrase: String = ""
) {
	SignerNavigator(this).navigate(button, details, seedPhrase)
}


interface Navigator {
	fun navigate(
		button: Action,
		details: String = "",
		seedPhrase: String = ""
	)

	fun navigate(action: LocalNavigationAction)
}

class SignerNavigator(private val singleton: SignerDataModel): Navigator {

	override fun navigate(button: Action, details: String, seedPhrase: String) {
		try {
			singleton._actionResult.value = backendAction(button, details, seedPhrase)
		} catch (e: java.lang.Exception) {
			Log.e("Navigation error", e.toString())
			Toast.makeText(singleton.context, e.toString(), Toast.LENGTH_SHORT).show()
		}
	}

	override fun navigate(action: LocalNavigationAction) {
	}
}

class EmptyNavigator : Navigator {
	override fun navigate(button: Action, details: String, seedPhrase: String) {
		//do nothing
	}

	override fun navigate(action: LocalNavigationAction) {
		//do nothing
	}
}


sealed class LocalNavigationAction {
	class ShowExportPrivateKey(hash: String): LocalNavigationAction()
}
