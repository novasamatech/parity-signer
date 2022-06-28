package io.parity.signer.components

import android.content.Context
import android.widget.Toast
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity


//This class shouldn't live here. I'd expect the component folder to have only compose classes.
//Instead Authentication should live on the core layer.
class Authentication(val setAuth: (Boolean) -> Unit) {
	// Consider isStrongCredential naming
	var strongCredentials: Boolean = false


	private lateinit var biometricPrompt: BiometricPrompt
	lateinit var context: Context

	//Why is this a FragmentActivity when there's no fragment in the project?
	fun authenticate(activity: FragmentActivity, onSuccess: () -> Unit) {
		val biometricManager = BiometricManager.from(context)

		val promptInfo =
			if (strongCredentials) {
				BiometricPrompt.PromptInfo.Builder()
					.setTitle("UNLOCK SIGNER") // Use StringRes
					.setSubtitle("Please authenticate yourself") // Use StringRes
					.setAllowedAuthenticators(BiometricManager.Authenticators.DEVICE_CREDENTIAL)
					.build()
			} else {
				BiometricPrompt.PromptInfo.Builder()
					.setTitle("UNLOCK SIGNER") // Use StringRes
					.setSubtitle("Please authenticate yourself") // Use StringRes
					.setNegativeButtonText("Cancel") // Use StringRes
					.setAllowedAuthenticators(BiometricManager.Authenticators.BIOMETRIC_STRONG)
					.build()
			}

		//This function is too long and very spagetti. Can it be cleaned ?
		//Strings should be plain, consider StringRes
		when (biometricManager.canAuthenticate(if (strongCredentials) BiometricManager.Authenticators.DEVICE_CREDENTIAL else BiometricManager.Authenticators.BIOMETRIC_STRONG)) {
			BiometricManager.BIOMETRIC_SUCCESS -> {

				val executor = ContextCompat.getMainExecutor(context)

				biometricPrompt = BiometricPrompt(
					activity, executor,
					object : BiometricPrompt.AuthenticationCallback() {
						override fun onAuthenticationError(
							errorCode: Int,
							errString: CharSequence
						) {
							super.onAuthenticationError(errorCode, errString)
							Toast.makeText(
								context,
								"Authentication error: $errString", Toast.LENGTH_SHORT
							)
								.show()
							setAuth(false)
						}

						override fun onAuthenticationSucceeded(
							result: BiometricPrompt.AuthenticationResult
						) {
							super.onAuthenticationSucceeded(result)
							setAuth(true)
							onSuccess()
						}

						override fun onAuthenticationFailed() {
							super.onAuthenticationFailed()
							Toast.makeText(
								context, "Authentication failed",
								Toast.LENGTH_SHORT
							)
								.show()
							setAuth(false)
						}
					})

				biometricPrompt.authenticate(promptInfo)
			}
			BiometricManager.BIOMETRIC_ERROR_NO_HARDWARE -> {
				Toast.makeText(
					context,
					"Insufficient security features available on this device.",
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_HW_UNAVAILABLE -> {
				Toast.makeText(
					context,
					"Security features are currently unavailable.",
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED -> {
				Toast.makeText(
					context,
					"Authentication system not enrolled; please enable "
						+ if (strongCredentials)
						"password or pin code"
					else
						"biometric authentication",
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_SECURITY_UPDATE_REQUIRED -> {
				Toast.makeText(
					context,
					"Security update is required",
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_UNSUPPORTED -> {
				Toast.makeText(
					context,
					"Security update is required",
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_STATUS_UNKNOWN -> {
				Toast.makeText(
					context,
					"Authentication system failure",
					Toast.LENGTH_LONG
				).show()
				return
			}
		}

	}


}



