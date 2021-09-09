package io.parity.signer.components

import android.content.Context
import android.widget.Toast
import androidx.biometric.BiometricManager.Authenticators.DEVICE_CREDENTIAL
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity


class Authentication {
	private val promptInfo = BiometricPrompt.PromptInfo.Builder()
		.setTitle("UNLOCK SEED")
		.setSubtitle("Please authenticate yourself")
		.setAllowedAuthenticators(DEVICE_CREDENTIAL)
		.build()

	lateinit var biometricPrompt: BiometricPrompt
	lateinit var context: Context

	fun authenticate(activity: FragmentActivity, onSuccess: () -> Unit) {
		val executor = ContextCompat.getMainExecutor(context)

		biometricPrompt = BiometricPrompt(
			activity, executor,
			object : BiometricPrompt.AuthenticationCallback() {
				override fun onAuthenticationError(errorCode: Int,
																					 errString: CharSequence) {
					super.onAuthenticationError(errorCode, errString)
					Toast.makeText(context,
						"Authentication error: $errString", Toast.LENGTH_SHORT)
						.show()
				}

				override fun onAuthenticationSucceeded(
					result: BiometricPrompt.AuthenticationResult) {
					super.onAuthenticationSucceeded(result)
					Toast.makeText(context,
						"Authentication succeeded!", Toast.LENGTH_SHORT)
						.show()
					onSuccess()
				}

				override fun onAuthenticationFailed() {
					super.onAuthenticationFailed()
					Toast.makeText(context, "Authentication failed",
						Toast.LENGTH_SHORT)
						.show()
				}
			})

		biometricPrompt.authenticate(promptInfo)
	}


}



