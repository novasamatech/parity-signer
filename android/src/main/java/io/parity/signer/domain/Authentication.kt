package io.parity.signer.domain

import android.content.Context
import android.widget.Toast
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import io.parity.signer.R
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlin.coroutines.suspendCoroutine


class Authentication {

	companion object {
		fun canAuthenticate(context: Context): Boolean {
			val biometricManager = BiometricManager.from(context)

			return when (biometricManager.canAuthenticate(
				BiometricManager.Authenticators.DEVICE_CREDENTIAL
			)) {
				BiometricManager.BIOMETRIC_ERROR_HW_UNAVAILABLE,
				BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED,
				BiometricManager.BIOMETRIC_ERROR_NO_HARDWARE,
				BiometricManager.BIOMETRIC_ERROR_SECURITY_UPDATE_REQUIRED,
				BiometricManager.BIOMETRIC_ERROR_UNSUPPORTED,
				BiometricManager.BIOMETRIC_STATUS_UNKNOWN -> {
					false
				}
				BiometricManager.BIOMETRIC_SUCCESS -> {
					true
				}
				else -> {
					submitErrorState("unexpected biometric response value, this should be impossible")
					true
				}
			}
		}
	}

	private lateinit var biometricPrompt: BiometricPrompt

	private val _auth = MutableStateFlow<Boolean>(false)
	val auth: StateFlow<Boolean> = _auth

	fun authenticate(activity: FragmentActivity, onSuccess: () -> Unit) {
		if (FeatureFlags.isEnabled(FeatureOption.SKIP_UNLOCK_FOR_DEVELOPMENT)) {
			_auth.value = true
			onSuccess()
			return
		}

		val context = activity.baseContext
		val biometricManager = BiometricManager.from(context)

		val promptInfo =
			BiometricPrompt.PromptInfo.Builder()
				.setTitle(context.getString(R.string.unlock_device_title))
				.setSubtitle(context.getString(R.string.unlock_device_subtitle))
				.setAllowedAuthenticators(
					BiometricManager.Authenticators.DEVICE_CREDENTIAL
				)
				.build()

		when (biometricManager.canAuthenticate(
			BiometricManager.Authenticators.DEVICE_CREDENTIAL
		)) {
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
								context.getString(R.string.auth_error_message, errString),
								Toast.LENGTH_SHORT
							).show()
							_auth.value = false
						}

						override fun onAuthenticationSucceeded(
							result: BiometricPrompt.AuthenticationResult
						) {
							super.onAuthenticationSucceeded(result)
							_auth.value = true
							onSuccess()
						}

						override fun onAuthenticationFailed() {
							super.onAuthenticationFailed()
							Toast.makeText(
								context, context.getString(R.string.auth_failed_message),
								Toast.LENGTH_SHORT
							).show()
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
					context,
					context.getString(R.string.auth_error_security_update_required),
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

	suspend fun authenticate(activity: FragmentActivity): AuthResult =
		suspendCoroutine { continuation ->
			val context = activity.baseContext
			val biometricManager = BiometricManager.from(context)

			val promptInfo =
				BiometricPrompt.PromptInfo.Builder()
					.setTitle(context.getString(R.string.unlock_device_title))
					.setSubtitle(context.getString(R.string.unlock_device_subtitle))
					.setAllowedAuthenticators(BiometricManager.Authenticators.DEVICE_CREDENTIAL)
					.build()

			when (biometricManager.canAuthenticate(
				BiometricManager.Authenticators.DEVICE_CREDENTIAL
			)) {
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
									context.getString(R.string.auth_error_message, errString),
									Toast.LENGTH_SHORT
								).show()
								_auth.value = false
								continuation.resumeWith(Result.success(AuthResult.AuthError))
							}

							override fun onAuthenticationSucceeded(
								result: BiometricPrompt.AuthenticationResult
							) {
								super.onAuthenticationSucceeded(result)
								_auth.value = true
								continuation.resumeWith(Result.success(AuthResult.AuthSuccess))
							}

							override fun onAuthenticationFailed() {
								super.onAuthenticationFailed()
								Toast.makeText(
									context, context.getString(R.string.auth_failed_message),
									Toast.LENGTH_SHORT
								).show()
							}
						}
					)
					biometricPrompt.authenticate(promptInfo)
				}
				BiometricManager.BIOMETRIC_ERROR_NO_HARDWARE -> {
					Toast.makeText(
						context, context.getString(R.string.auth_error_no_hardware),
						Toast.LENGTH_LONG
					).show()
					continuation.resumeWith(Result.success(AuthResult.AuthUnavailable))
				}
				BiometricManager.BIOMETRIC_ERROR_HW_UNAVAILABLE -> {
					Toast.makeText(
						context,
						context.getString(R.string.auth_error_hardware_unavailable),
						Toast.LENGTH_LONG
					).show()
					continuation.resumeWith(Result.success(AuthResult.AuthUnavailable))
				}
				BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED -> {
					Toast.makeText(
						context, context.getString(R.string.auth_error_none_enrolled),
						Toast.LENGTH_LONG
					).show()
					continuation.resumeWith(Result.success(AuthResult.AuthUnavailable))
				}
				BiometricManager.BIOMETRIC_ERROR_SECURITY_UPDATE_REQUIRED -> {
					Toast.makeText(
						context,
						context.getString(R.string.auth_error_security_update_required),
						Toast.LENGTH_LONG
					).show()
					continuation.resumeWith(Result.success(AuthResult.AuthUnavailable))
				}
				BiometricManager.BIOMETRIC_ERROR_UNSUPPORTED -> {
					Toast.makeText(
						context, context.getString(R.string.auth_error_unsupported),
						Toast.LENGTH_LONG
					).show()
					continuation.resumeWith(Result.success(AuthResult.AuthUnavailable))
				}
				BiometricManager.BIOMETRIC_STATUS_UNKNOWN -> {
					Toast.makeText(
						context, context.getString(R.string.auth_error_status_unknown),
						Toast.LENGTH_LONG
					).show()
					continuation.resumeWith(Result.success(AuthResult.AuthUnavailable))
				}
			}
		}

}

sealed class AuthResult {
	object AuthSuccess : AuthResult()
	object AuthError : AuthResult()
	object AuthFailed : AuthResult()
	object AuthUnavailable : AuthResult()
}


