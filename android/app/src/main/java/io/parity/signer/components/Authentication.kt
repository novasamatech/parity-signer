package io.parity.signer.components

import android.content.Context
import android.os.Build
import android.widget.Toast
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity


class Authentication {
	private val promptInfo = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
		BiometricPrompt.PromptInfo.Builder()
			.setTitle("UNLOCK SEED")
			.setSubtitle("Please authenticate yourself")
			.setAllowedAuthenticators(BiometricManager.Authenticators.DEVICE_CREDENTIAL)
			.build()
	} else {
		BiometricPrompt.PromptInfo.Builder()
			.setTitle("UNLOCK SEED")
			.setSubtitle("Please authenticate yourself")
			.setNegativeButtonText("Cancel")
			.setAllowedAuthenticators(BiometricManager.Authenticators.BIOMETRIC_STRONG)
			.build()
	}

	private lateinit var biometricPrompt: BiometricPrompt
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



