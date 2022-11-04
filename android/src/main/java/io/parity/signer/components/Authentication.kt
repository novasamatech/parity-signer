package io.parity.signer.components

import android.content.Context
import android.widget.Toast
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import io.parity.signer.R


class Authentication(val setAuth: (Boolean) -> Unit) {

	private lateinit var biometricPrompt: BiometricPrompt
	lateinit var context: Context

	fun authenticate(activity: FragmentActivity, onSuccess: () -> Unit) {
		val biometricManager = BiometricManager.from(context)

		val promptInfo =
				BiometricPrompt.PromptInfo.Builder()
					.setTitle(context.getString(R.string.unlock_device_title))
					.setSubtitle(context.getString(R.string.unlock_device_subtitle))
					.setAllowedAuthenticators(BiometricManager.Authenticators.DEVICE_CREDENTIAL or BiometricManager.Authenticators.BIOMETRIC_WEAK)
					.build()

		when (biometricManager.canAuthenticate(BiometricManager.Authenticators.DEVICE_CREDENTIAL
			or BiometricManager.Authenticators.BIOMETRIC_WEAK)) {
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
								context.getString(R.string.auth_error_message, errString), Toast.LENGTH_SHORT
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
								context, context.getString(R.string.auth_failed_message),
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
					context, context.getString(R.string.auth_error_no_hardware),
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_HW_UNAVAILABLE -> {
				Toast.makeText(
					context,
					context.getString(R.string.auth_error_hardware_unavailable),
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED -> {
				Toast.makeText(
					context, context.getString(R.string.auth_error_none_enrolled),
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_SECURITY_UPDATE_REQUIRED -> {
				Toast.makeText(
					context, context.getString(R.string.auth_error_security_update_required),
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_ERROR_UNSUPPORTED -> {
				Toast.makeText(
					context, context.getString(R.string.auth_error_unsupported),
					Toast.LENGTH_LONG
				).show()
				return
			}
			BiometricManager.BIOMETRIC_STATUS_UNKNOWN -> {
				Toast.makeText(
					context, context.getString(R.string.auth_error_status_unknown),
					Toast.LENGTH_LONG
				).show()
				return
			}
		}
	}

}



