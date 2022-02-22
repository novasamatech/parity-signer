package io.parity.signer.components

import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.provider.Settings
import android.util.Log
import android.widget.Toast
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity


class Authentication(val setAuth: (Boolean) -> Unit) {
	var strongCredentials: Boolean = false
	private val promptInfo =
		if (strongCredentials) {
			BiometricPrompt.PromptInfo.Builder()
				.setTitle("UNLOCK SIGNER")
				.setSubtitle("Please authenticate yourself")
				.setAllowedAuthenticators(BiometricManager.Authenticators.DEVICE_CREDENTIAL)
				.build()
		} else {
			BiometricPrompt.PromptInfo.Builder()
				.setTitle("UNLOCK SIGNER")
				.setSubtitle("Please authenticate yourself")
				.setNegativeButtonText("Cancel")
				.setAllowedAuthenticators(BiometricManager.Authenticators.BIOMETRIC_STRONG)
				.build()
		}

	private lateinit var biometricPrompt: BiometricPrompt
	lateinit var context: Context

	fun authenticate(activity: FragmentActivity, onSuccess: () -> Unit) {
		Log.d("auth", "test")
		val biometricManager = BiometricManager.from(context)
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



